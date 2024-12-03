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
        if let Some(value) = self.variables.get(key) {
            Some(value.clone())
        } else {
            env::var(key).ok()
        }
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

    /// Set and apply the variable to the system environment
    pub unsafe fn set_system_env(&mut self, key: &str, value: &str) -> Result<(), String> {
        self.set(key, value)?;
        env::set_var(key, value);
        Ok(())
    }

    /// Parse the content of a .env file into a HashMap.
    fn parse(content: &str) -> HashMap<String, String> {
        content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }

                let (key, value) = line.split_once('=')
                    .map(|(k, v)| (k.trim(), v.trim()))
                    .unwrap_or((line, ""));

                Some((key.to_string(), value.to_string()))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let content = "KEY1=VALUE1\nKEY2=VALUE2\n";
        let variables = Envie::parse(content);
        assert_eq!(variables.get("KEY1"), Some(&"VALUE1".to_string()));
        assert_eq!(variables.get("KEY2"), Some(&"VALUE2".to_string()));
    }

    #[test]
    fn test_get() {
        let env = Envie { variables: HashMap::new() };
        unsafe { env::set_var("TEST_KEY", "test_value"); }
        assert_eq!(env.get("TEST_KEY"), Some("test_value".to_string()));
    }
}
