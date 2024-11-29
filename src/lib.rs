// src/lib.rs
use std::collections::HashMap;
use std::env;
use std::fs;

pub struct Envie {
    variables: HashMap<String, String>,
}

impl Envie {
    /// Load .env file and parse it into a new Envie instance.
    pub fn load() -> Result<Self, String> {
        let content = fs::read_to_string(".env")
            .map_err(|_| "Failed to read .env file. Make sure it exists in the current directory.")?;
        let variables = Self::parse(&content);
        Ok(Self { variables })
    }

    /// Get a value by key.
    pub fn get(&self, key: &str) -> Option<String> {
        self.variables
            .get(key)
            .cloned()
            .or_else(|| env::var(key).ok())
    }

    /// Get a value as a boolean.
    pub fn get_bool(&self, key: &str) -> Result<bool, String> {
        self.get(key)
            .map(|v| v.to_lowercase())
            .ok_or(format!("Key '{}' not found", key))
            .and_then(|v| match v.as_str() {
                "true" | "1" => Ok(true),
                "false" | "0" => Ok(false),
                _ => Err(format!("Invalid boolean value for key '{}'", key)),
            })
    }

    /// Parse the content of a .env file into a HashMap.
    fn parse(content: &str) -> HashMap<String, String> {
        content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    None
                } else {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                    } else {
                        None
                    }
                }
            })
            .collect()
    }
}
