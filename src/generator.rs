use crate::analyzer::DiffAnalysis;
use regex::Regex;

pub struct CommitGenerator {
    conventional_types: Vec<&'static str>,
}

impl CommitGenerator {
    pub fn new() -> Self {
        Self {
            conventional_types: vec![
                "feat",     // A new feature
                "fix",      // A bug fix
                "docs",     // Documentation only changes
                "style",    // Changes that do not affect the meaning of the code
                "refactor", // A code change that neither fixes a bug nor adds a feature
                "perf",     // A code change that improves performance
                "test",     // Adding missing tests or correcting existing tests
                "build",    // Changes that affect the build system or external dependencies
                "ci",       // Changes to CI configuration files and scripts
                "chore",    // Other changes that don't modify src or test files
                "revert",   // Reverts a previous commit
            ],
        }
    }

    pub fn generate_suggestions(&self, analysis: &DiffAnalysis, breaking: bool) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Primary suggestion based on analysis
        let primary = self.generate_with_type(analysis, &analysis.primary_type, breaking || analysis.has_breaking_changes);
        suggestions.push(primary);

        // Alternative suggestions with different types
        for r#type in ["fix", "refactor", "chore"].iter() {
            if *r#type != analysis.primary_type {
                let alt = self.generate_with_type(analysis, r#type, false);
                suggestions.push(alt);
            }
        }

        // Limit to 3 suggestions
        suggestions.truncate(3);
        suggestions
    }

    pub fn generate_with_type(&self, analysis: &DiffAnalysis, r#type: &str, breaking: bool) -> String {
        let scope = self.infer_scope(analysis);
        let subject = self.generate_subject(analysis);
        
        let mut message = format!("{}{}: {}", 
            r#type,
            if scope.is_empty() { "".to_string() } else { format!("({})", scope) },
            subject
        );

        if breaking {
            message.push_str("!\n\n");
            message.push_str("BREAKING CHANGE: Significant API or behavior change\n");
        }

        // Add body with details if significant changes
        if analysis.files_changed > 5 || analysis.lines_added + analysis.lines_removed > 100 {
            message.push_str(&format!(
                "\nChanges:\n- {} files modified\n- {} lines added, {} lines removed",
                analysis.files_changed, analysis.lines_added, analysis.lines_removed
            ));

            if !analysis.file_types.is_empty() {
                message.push_str(&format!(
                    "\n- Affected types: {}",
                    analysis.file_types.join(", ")
                ));
            }
        }

        message
    }

    fn infer_scope(&self, analysis: &DiffAnalysis) -> String {
        // Try to infer scope from file patterns
        let mut scopes: Vec<String> = Vec::new();

        for ext in &analysis.file_types {
            match ext.as_str() {
                "rs" => scopes.push("core".to_string()),
                "test" | "spec" => scopes.push("tests".to_string()),
                "md" => scopes.push("docs".to_string()),
                "toml" => scopes.push("config".to_string()),
                "json" => scopes.push("data".to_string()),
                _ => {}
            }
        }

        // Return most common or first scope
        scopes.into_iter()
            .next()
            .unwrap_or_else(|| "app".to_string())
    }

    fn generate_subject(&self, analysis: &DiffAnalysis) -> String {
        let action = self.select_action(analysis);
        
        // Create concise subject (max 72 chars per conventional commits)
        let mut subject = format!("{} {}", action, self.describe_changes(analysis));
        
        if subject.len() > 72 {
            subject.truncate(69);
            subject.push_str("...");
        }

        // Ensure lowercase start and no period at end
        if !subject.is_empty() {
            let chars: Vec<char> = subject.chars().collect();
            if chars[0].is_uppercase() {
                subject = format!("{}", chars[0].to_lowercase().collect::<String>() + &subject[1..]);
            }
            if subject.ends_with('.') {
                subject.pop();
            }
        }

        subject
    }

    fn select_action(&self, analysis: &DiffAnalysis) -> &'static str {
        match analysis.primary_type.as_str() {
            "feat" => {
                if analysis.lines_added > 50 {
                    "implement"
                } else {
                    "add"
                }
            },
            "fix" => "fix",
            "refactor" => "refactor",
            "test" => "update tests for",
            "docs" => "update docs for",
            "chore" => "update",
            _ => "change",
        }
    }

    fn describe_changes(&self, analysis: &DiffAnalysis) -> String {
        if analysis.files_changed == 1 {
            "single component".to_string()
        } else if analysis.files_changed <= 3 {
            format!("{} components", analysis.files_changed)
        } else {
            "multiple modules".to_string()
        }
    }

    pub fn validate_conventional(&self, message: &str) -> bool {
        let lines: Vec<&str> = message.lines().collect();
        if lines.is_empty() {
            return false;
        }

        // Check first line matches pattern and is under 72 chars
        let first_line = lines[0];
        if first_line.len() > 72 {
            return false;
        }

        let pattern = r"^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\([a-z0-9_-]+\))?!?: .{1,72}";
        Regex::new(pattern).map_or(false, |re| re.is_match(first_line))
    }

    pub fn suggest_pr_title(&self, analysis: &DiffAnalysis) -> String {
        let action = self.select_action(analysis);
        format!(
            "{} {} - {} files changed",
            action.to_string().to_uppercase(),
            self.describe_changes(analysis),
            analysis.files_changed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_conventional_valid() {
        let generator = CommitGenerator::new();
        
        assert!(generator.validate_conventional("feat: add new feature"));
        assert!(generator.validate_conventional("fix(core): fix bug in core module"));
        assert!(generator.validate_conventional("docs: update readme"));
    }

    #[test]
    fn test_validate_conventional_invalid() {
        let generator = CommitGenerator::new();
        
        assert!(!generator.validate_conventional("added new feature"));
        assert!(!generator.validate_conventional("feat: too long message that exceeds the maximum allowed length for conventional commit subjects which should be under seventy two characters total"));
    }

    #[test]
    fn test_generate_subject_length() {
        let analysis = DiffAnalysis {
            files_changed: 10,
            lines_added: 500,
            lines_removed: 200,
            primary_type: "feat".to_string(),
            file_types: vec!["rs".to_string()],
            has_breaking_changes: false,
            summary: "test".to_string(),
        };

        let generator = CommitGenerator::new();
        let subject = generator.generate_subject(&analysis);
        
        assert!(subject.len() <= 72);
    }
}
