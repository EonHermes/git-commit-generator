use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_type: String,
    pub max_subject_length: usize,
    pub include_scope: bool,
    pub auto_detect_breaking: bool,
    pub team_conventions: TeamConventions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamConventions {
    pub allowed_types: Vec<String>,
    pub required_scopes: Vec<String>,
    pub custom_rules: Vec<CustomRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomRule {
    pub name: String,
    pub pattern: String,
    pub message_template: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_type: "feat".to_string(),
            max_subject_length: 72,
            include_scope: true,
            auto_detect_breaking: true,
            team_conventions: TeamConventions {
                allowed_types: vec![
                    "feat".to_string(),
                    "fix".to_string(),
                    "docs".to_string(),
                    "style".to_string(),
                    "refactor".to_string(),
                    "perf".to_string(),
                    "test".to_string(),
                    "build".to_string(),
                    "ci".to_string(),
                    "chore".to_string(),
                ],
                required_scopes: vec![],
                custom_rules: vec![],
            },
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let path = path.as_ref();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(path)?;
        let content = serde_json::to_string_pretty(self)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn find_in_repo(repo_path: &Path) -> Option<std::path::PathBuf> {
        let config_file = repo_path.join(".git-commit-gen.json");
        
        if config_file.exists() {
            return Some(config_file);
        }

        // Check parent directories
        let mut current = repo_path.parent()?;
        while let Some(parent) = current.parent() {
            let git_dir = current.join(".git");
            if git_dir.exists() {
                let config_file = current.join(".git-commit-gen.json");
                if config_file.exists() {
                    return Some(config_file);
                }
            }
            current = parent;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_type, "feat");
        assert_eq!(config.max_subject_length, 72);
    }

    #[test]
    fn test_save_and_load() {
        let tmp = TempDir::new().unwrap();
        let config_path = tmp.path().join("config.json");

        let config = Config::default();
        config.save(&config_path).unwrap();

        let loaded = Config::load(&config_path).unwrap();
        assert_eq!(loaded.default_type, config.default_type);
    }

    #[test]
    fn test_load_nonexistent() {
        let tmp = TempDir::new().unwrap();
        let non_existent = tmp.path().join("nonexistent.json");
        
        let config = Config::load(&non_existent).unwrap();
        assert_eq!(config.default_type, "feat"); // Should return default
    }
}
