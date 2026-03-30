use git2::Repository;
use serde::Serialize;
use std::path::{Path, PathBuf};
use thiserror::Error;
use std::fs;

#[derive(Error, Debug)]
pub enum AnalyzerError {
    #[error("Not a git repository: {0}")]
    NotARepo(String),
    
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Serialize)]
pub struct DiffAnalysis {
    pub files_changed: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub primary_type: String,
    pub file_types: Vec<String>,
    pub has_breaking_changes: bool,
    pub summary: String,
}

pub struct DiffAnalyzer {
    repo: Repository,
    repo_path: PathBuf,
}

impl DiffAnalyzer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, AnalyzerError> {
        let repo_path = path.as_ref().to_path_buf();
        let repo = Repository::open(&repo_path)
            .map_err(|_| AnalyzerError::NotARepo(repo_path.display().to_string()))?;
        
        Ok(Self { repo, repo_path })
    }

    pub fn analyze_staged(&self) -> Result<DiffAnalysis, AnalyzerError> {
        // For staged analysis, check git status
        let mut opts = git2::StatusOptions::new();
        let statuses = self.repo.statuses(Some(&mut opts))?;
        
        Ok(self.analyze_statuses(statuses))
    }

    pub fn analyze_unstaged(&self) -> Result<DiffAnalysis, AnalyzerError> {
        // For unstaged, check working directory changes
        let mut opts = git2::StatusOptions::new();
        opts.include_untracked(false);
        let statuses = self.repo.statuses(Some(&mut opts))?;
        
        Ok(self.analyze_statuses(statuses))
    }

    fn analyze_statuses(&self, statuses: git2::Statuses) -> DiffAnalysis {
        let mut files_changed = 0;
        let mut lines_added = 0;
        let mut lines_removed = 0;
        let mut file_types: Vec<String> = Vec::new();
        let mut has_breaking_changes = false;

        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                files_changed += 1;
                
                let ext = Path::new(path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                if !file_types.contains(&ext) {
                    file_types.push(ext.clone());
                }

                // Check for breaking changes indicators in filename
                let path_lower = path.to_lowercase();
                if path_lower.contains("breaking") {
                    has_breaking_changes = true;
                }

                // Estimate lines changed based on status
                match entry.status() {
                    git2::Status::INDEX_NEW | git2::Status::WT_NEW => {
                        lines_added += self.estimate_lines(path);
                    },
                    git2::Status::INDEX_DELETED | git2::Status::WT_DELETED => {
                        lines_removed += 10; // Estimate
                    },
                    git2::Status::INDEX_MODIFIED | git2::Status::WT_MODIFIED => {
                        lines_added += self.estimate_lines(path) / 2;
                        lines_removed += self.estimate_lines(path) / 3;
                    },
                    _ => {}
                }
            }
        }

        let primary_type = self.determine_commit_type(&file_types, files_changed);
        let summary = self.generate_summary(&primary_type, files_changed, lines_added, lines_removed);

        DiffAnalysis {
            files_changed,
            lines_added,
            lines_removed,
            primary_type,
            file_types,
            has_breaking_changes,
            summary,
        }
    }

    fn estimate_lines(&self, path: &str) -> usize {
        let full_path = self.repo_path.join(path);
        if let Ok(content) = fs::read_to_string(&full_path) {
            content.lines().count()
        } else {
            20 // Default estimate
        }
    }

    fn determine_commit_type(&self, file_types: &[String], files_changed: usize) -> String {
        let mut feat_count = 0;
        let mut fix_count = 0;
        let mut refactor_count = 0;
        let mut test_count = 0;
        let mut docs_count = 0;
        let mut chore_count = 0;

        for ext in file_types {
            match ext.as_str() {
                "rs" => {
                    if files_changed <= 3 {
                        feat_count += 1;
                    } else {
                        refactor_count += 1;
                    }
                },
                "test" | "spec" => test_count += 1,
                "md" | "txt" | "rst" => docs_count += 1,
                "toml" | "json" | "yaml" | "yml" => chore_count += 1,
                _ => {
                    if files_changed <= 2 {
                        fix_count += 1;
                    } else {
                        feat_count += 1;
                    }
                },
            }
        }

        let counts = [
            ("feat", feat_count),
            ("fix", fix_count),
            ("refactor", refactor_count),
            ("test", test_count),
            ("docs", docs_count),
            ("chore", chore_count),
        ];

        counts.iter()
            .max_by_key(|&(_, count)| count)
            .map(|(t, _)| t.to_string())
            .unwrap_or_else(|| "feat".to_string())
    }

    fn generate_summary(&self, r#type: &str, files: usize, added: usize, removed: usize) -> String {
        let action = match r#type {
            "feat" => "Add",
            "fix" => "Fix",
            "refactor" => "Refactor",
            "test" => "Update tests for",
            "docs" => "Update documentation for",
            "chore" => "Update",
            _ => "Change",
        };

        let scope = if files == 1 {
            "single module".to_string()
        } else if files <= 3 {
            format!("{} modules", files)
        } else {
            "multiple components".to_string()
        };

        format!(
            "{} {} ({} lines changed)",
            action, scope, added + removed
        )
    }

    pub fn commit(&self, message: &str) -> Result<git2::Oid, AnalyzerError> {
        let mut index = self.repo.index()?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let head = self.repo.head()?;
        let parent = head.peel_to_commit()?;

        let signature = git2::Signature::now(
            "git-commit-gen",
            "git-commit-gen@localhost"
        )?;

        let oid = self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent]
        )?;

        Ok(oid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_repo() -> TempDir {
        let tmp = TempDir::new().unwrap();
        git2::Repository::init(tmp.path()).unwrap();
        tmp
    }

    #[test]
    fn test_not_a_repo() {
        let tmp = TempDir::new().unwrap();
        let result = DiffAnalyzer::new(tmp.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_repo() {
        let tmp = setup_test_repo();
        let result = DiffAnalyzer::new(tmp.path());
        assert!(result.is_ok());
    }
}
