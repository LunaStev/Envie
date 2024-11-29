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
envie = "0.1"
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
let env = Envie::load().expect("Failed to load .env file");

    let database_url = env.get("DATABASE_URL").unwrap_or_else(|| "default_url".to_string());
    let debug_mode: bool = env.get_bool("DEBUG_MODE").unwrap_or(false);

    println!("Database URL: {}", database_url);
    println!("Debug Mode: {}", debug_mode);
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