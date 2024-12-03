# Envie
Envie is a lightweight and user-friendly library for managing environment variables in Rust. It allows you to load and parse .env files effortlessly and provides type-safe access to environment variables for seamless integration into your applications.

## Key Features
* Automatically loads .env files
* Retrieves environment variables by key
* Supports type-safe conversions (e.g., boolean, integers)
* Fallback to system environment variables

## Installation
Add Envie to your `Cargo.toml` file:

```toml
envie = "0.1.2"
```

Then run:

```bash
cargo build
```

## Usage
Here’s a quick example of how to use Envie:

**Load environment variables**

```rust
use envie::Envie;

fn main() {
    // Load the .env file and exit if it fails
    let mut env = Envie::load().expect("Failed to load .env file");

    // Get the DATABASE_URL value, or use "default_url" if it's not found
    let database_url = env.get("DATABASE_URL").unwrap_or_else(|| "default_url".to_string());
    println!("Database URL: {}", database_url);

    // Get the DEBUG_MODE value, or set it to false if it's not found
    let debug_mode = env.get_bool("DEBUG_MODE").unwrap_or_else(|_| false);
    println!("Debug Mode: {}", debug_mode);

    // Add or update a new environment variable
    env.set("NEW_VARIABLE", "12345").expect("Failed to set NEW_VARIABLE");
    println!("NEW_VARIABLE set to 12345");

    // Print all environment variables
    let all_vars = env.get_all();
    println!("All environment variables:");
    for (key, value) in all_vars {
        println!("{} = {}", key, value);
    }

    // Delete a specific environment variable
    env.remove("NEW_VARIABLE").expect("Failed to remove NEW_VARIABLE");
    println!("NEW_VARIABLE removed");

    // Reload the .env file to reflect the changes
    let env = Envie::load().expect("Failed to load .env file after removal");
    let all_vars = env.get_all();
    println!("All environment variables after removal:");
    for (key, value) in all_vars {
        println!("{} = {}", key, value);
    }
}
```
### Example .env file
```env
DATABASE_URL=postgres://user:password@localhost:5432/mydb
DEBUG_MODE=true
```

## Why Envie?
Envie makes managing environment variables simple and intuitive while maintaining Rust's type safety and performance standards. Whether you’re working on small projects or large-scale applications, Envie ensures your configuration is accessible and reliable.

## License
This project is licensed under the MPL-2.0 License. See the `LICENSE` file for details.