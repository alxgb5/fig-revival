//! Fig Local Provider - Offline autocomplete without AWS/Bedrock
//!
//! This provider generates shell completions locally without any cloud dependencies.

use anyhow::Result;

/// Local completion suggestion
#[derive(Debug, Clone)]
pub struct LocalSuggestion {
    pub text: String,
    pub description: Option<String>,
}

/// Local provider for generating shell completions
pub struct LocalProvider {
    history: Vec<String>,
}

impl LocalProvider {
    /// Create a new local provider
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    /// Generate completion suggestions for a given buffer
    ///
    /// This is a simple implementation that:
    /// 1. Returns common directory suggestions for 'cd'
    /// 2. Returns history-based suggestions for other commands
    pub fn generate_suggestions(&self, buffer: &str) -> Result<Vec<LocalSuggestion>> {
        let buffer = buffer.trim();

        // Handle 'cd' command with directory suggestions
        if buffer.starts_with("cd ") || buffer == "cd" {
            return Ok(vec![
                LocalSuggestion {
                    text: "~/".to_string(),
                    description: Some("Home directory".to_string()),
                },
                LocalSuggestion {
                    text: "../".to_string(),
                    description: Some("Parent directory".to_string()),
                },
                LocalSuggestion {
                    text: "./".to_string(),
                    description: Some("Current directory".to_string()),
                },
            ]);
        }

        // Handle 'git' command suggestions
        if buffer.starts_with("git ") {
            let subcommands = vec![
                ("status", "Show working tree status"),
                ("add", "Add file contents to index"),
                ("commit", "Record changes to repository"),
                ("push", "Update remote refs"),
                ("pull", "Fetch and integrate"),
                ("checkout", "Switch branches"),
                ("branch", "List, create, or delete branches"),
                ("log", "Show commit logs"),
                ("diff", "Show changes"),
                ("merge", "Join development histories"),
            ];

            return Ok(subcommands
                .into_iter()
                .map(|(text, desc)| LocalSuggestion {
                    text: text.to_string(),
                    description: Some(desc.to_string()),
                })
                .collect());
        }

        // Handle 'npm' command suggestions
        if buffer.starts_with("npm ") {
            let subcommands = vec![
                ("install", "Install a package"),
                ("start", "Start a package"),
                ("test", "Test a package"),
                ("run", "Run arbitrary package scripts"),
                ("build", "Build a package"),
                ("init", "Create a package.json"),
                ("publish", "Publish a package"),
                ("update", "Update packages"),
            ];

            return Ok(subcommands
                .into_iter()
                .map(|(text, desc)| LocalSuggestion {
                    text: text.to_string(),
                    description: Some(desc.to_string()),
                })
                .collect());
        }

        // Default: no suggestions yet
        Ok(vec![])
    }

    /// Add command to history (for future history-based suggestions)
    pub fn add_to_history(&mut self, command: String) {
        self.history.push(command);
    }
}

impl Default for LocalProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cd_suggestions() {
        let provider = LocalProvider::new();
        let suggestions = provider.generate_suggestions("cd ").unwrap();
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.text.contains("~/")));
    }

    #[test]
    fn test_git_suggestions() {
        let provider = LocalProvider::new();
        let suggestions = provider.generate_suggestions("git ").unwrap();
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.text.contains("status")));
    }
}
