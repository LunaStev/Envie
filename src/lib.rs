use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;

pub struct Envie {
    pub variables: HashMap<String, String>,
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

    /// Get a value as an integer.
    pub fn get_int(&self, key: &str) -> Result<i32, String> {
        self.get(key)
            .ok_or(format!("Key '{}' not found", key))
            .and_then(|v| v.parse().map_err(|_| format!("Invalid integer value for key '{}'", key)))
    }

    /// Get all environment variables as a HashMap.
    pub fn get_all(&self) -> HashMap<String, String> {
        self.variables.clone()
    }

    /// Set a value for a given key and update the .env file.
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        self.variables.insert(key.to_string(), value.to_string());
        let content = fs::read_to_string(".env").unwrap_or_default();
        let mut updated_content = String::new();
        let mut found = false;

        for line in content.lines() {
            if line.starts_with(&format!("{}=", key)) {
                updated_content.push_str(&format!("{}={}\n", key, value));
                found = true;
            } else {
                updated_content.push_str(line);
                updated_content.push('\n');
            }
        }

        if !found {
            updated_content.push_str(&format!("{}={}\n", key, value));
        }

        fs::write(".env", updated_content).map_err(|_| "Failed to write to .env file")?;
        Ok(())
    }

    /// Remove a key-value pair and update the .env file.
    pub fn remove(&mut self, key: &str) -> Result<(), String> {
        self.variables.remove(key);

        let content = fs::read_to_string(".env").unwrap_or_default();
        let mut updated_content = String::new();

        for line in content.lines() {
            if !line.starts_with(&format!("{}=", key)) {
                updated_content.push_str(line);
                updated_content.push('\n');
            }
        }

        fs::write(".env", updated_content).map_err(|_| "Failed to write to .env file")?;
        Ok(())
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
