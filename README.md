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
envie = "0.2.0"
```

Then run:

```bash
cargo build
```

## 📚 Usage Guide for Envie (v0.2.0)
Here’s a quick guide on how to use the new and improved Envie v0.2.0 to manage environment variables in your Rust project.

### ✍️ Example Usage
Here's a simple usage example that demonstrates all the core features of Envie v0.2.0.
```rust
use envie::Envie;

fn main() {
    // 1️⃣ Load the .env file or exit with an error message if it fails
    let mut env = Envie::load().expect("Failed to load .env file");

    // 2️⃣ Retrieve the DATABASE_URL value, or use "default_url" as a fallback
    let database_url = env.get("DATABASE_URL").unwrap_or_else(|| "default_url".to_string());
    println!("Database URL: {}", database_url);

    // 3️⃣ Retrieve the DEBUG_MODE value as a boolean, defaulting to false if not found
    let debug_mode = env.get_bool("DEBUG_MODE").unwrap_or_else(|_| false);
    println!("Debug Mode: {}", debug_mode);

    // 4️⃣ Retrieve an integer variable (PORT), defaulting to 8080 if it can't be parsed
    let port = env.get_int("PORT").unwrap_or(8080);
    println!("Server will run on port: {}", port);

    // 5️⃣ Add or update an environment variable
    env.set("NEW_VARIABLE", "12345").expect("Failed to set NEW_VARIABLE");
    println!("NEW_VARIABLE set to 12345");

    // 6️⃣ Print all currently loaded environment variables
    println!("\nAll environment variables:");
    for (key, value) in env.get_all() {
        println!("{} = {}", key, value);
    }

    // 7️⃣ Remove a specific environment variable
    env.remove("NEW_VARIABLE").expect("Failed to remove NEW_VARIABLE");
    println!("\nNEW_VARIABLE removed");

    // 8️⃣ Reload the .env file to confirm changes
    let updated_env = Envie::load().expect("Failed to reload .env file");
    println!("\nUpdated environment variables:");
    for (key, value) in updated_env.get_all() {
        println!("{} = {}", key, value);
    }

    // 9️⃣ (Optional) Set a system environment variable (applies to current Rust process only)
    unsafe {
        env.set_system_env("SYSTEM_VARIABLE", "active").expect("Failed to set system environment variable");
    }
    println!("SYSTEM_VARIABLE set to 'active'");
}

```
### 📄 Example `.env` file
Here is an example of a `.env` file that you can create in the root of your Rust project.
```env
DATABASE_URL=postgres://user:password@localhost:5432/mydb
DEBUG_MODE=true
PORT=3000
APP_NAME=EnvieApp
```
> 📘 Note: The `.env` file should be placed in the root of your project (same level as `Cargo.toml`).
> You can customize it with your own variables.

### 🔥 Key Features
| Feature         | Description                                                | Example                                          |
|-----------------|------------------------------------------------------------|--------------------------------------------------|
| Load .env file  | Automatically load and parse `.env` file into memory.      | `Envie::load()`                                  |
| Get value       | Retrieve the value of an environment variable.             | `env.get("DATABASE_URL")`                        |
| Get as bool     | Get a variable as a boolean (`true`, `false`, `1`, `0`).   | `env.get_bool("DEBUG_MODE")`                     |
| Get as integer  | Get a variable as an integer.                              | `env.get_int("PORT")`                            |
| Set variable    | Add or update an environment variable.                     | `env.set("NEW_VARIABLE", "12345")`               |
| Remove variable | Remove an environment variable from the file.              | `env.remove("NEW_VARIABLE")`                     |
| List all vars   | Get a list of all loaded variables.                        | `env.get_all()`                                  |
| Set system env  | Set a system environment variable (current process only).  | `env.set_system_env("SYSTEM_VARIABLE", "value")` |

### 🛠️ How to Use
1. Install Envie
Add the following line to your `Cargo.toml` under `[dependencies]`.
```
envie = "0.2.0"
```
2. Create a .env file
Create a `.env` file in the root of your project and define your environment variables like this:
```
DATABASE_URL=postgres://user:password@localhost:5432/mydb
DEBUG_MODE=true
PORT=3000
```

3. Use Envie in your Rust code
```rust
use envie::Envie;

fn main() {
    let env = Envie::load().expect("Failed to load .env file");
    let database_url = env.get("DATABASE_URL").unwrap_or_else(|| "default_url".to_string());
    println!("Database URL: {}", database_url);
}
```

### 🧪 Advanced Usage

#### 1️⃣ Get a Variable with Default Value

```rust
let api_url = env.get("API_URL").unwrap_or_else(|| "https://default.api.com".to_string());
println!("API URL: {}", api_url);
```
If the key `API_URL` does not exist in the `.env` file, the default value `https://default.api.com` will be used.

#### 2️⃣ Get a Boolean Value

```rust
let is_production = env.get_bool("IS_PRODUCTION").unwrap_or_else(|_| false);
println!("Is production: {}", is_production);
```
The following values will be interpreted as `true` or `false`:

* True: `true`, `1`
* False: `false`, `0`

#### 3️⃣ Get an Integer Value
```rust
let port = env.get_int("PORT").unwrap_or(8080);
println!("Server running on port: {}", port);
```
This will parse the value of `PORT` from the environment as an integer. If the value is not a valid integer, it will fall back to `8080`.

#### 4️⃣ Add/Update an Environment Variable
```rust
env.set("NEW_KEY", "some_value").expect("Failed to set NEW_KEY");
```
This will add a new key-value pair to the `.env` file. If `NEW_KEY` already exists, it will be updated.

#### 5️⃣ Remove an Environment Variable
```rust
env.remove("OLD_KEY").expect("Failed to remove OLD_KEY");
```
This will remove the `OLD_KEY` entry from the `.env` file.

#### 6️⃣ List All Environment Variables
```rust
for (key, value) in env.get_all() {
    println!("{} = {}", key, value);
}
```
This will print all environment variables loaded from the `.env` file as well as any environment variables set in the operating system.

#### 7️⃣ Set System Environment Variable
```rust
unsafe {
    env.set_system_env("SYSTEM_VAR", "value").expect("Failed to set SYSTEM_VAR");
}
```
This will add a variable to the **system environment** (only for the current Rust process).
Be cautious, as `unsafe` code requires special attention to avoid side effects.

## 📈 Changelog for v0.2.0
Here are some of the new features and changes introduced in version 0.2.0:

* New Method: get_int — Retrieve environment variables as integers.
* New Method: set_system_env — Set a system environment variable for the current process.
* Improved I/O: Safer handling of .env file reads/writes.
* Bug Fixes: Better path handling for .env files in various environments.
* Performance: Reduced file I/O by caching loaded variables.

## ⚠️ Important Notes
* Thread-safety: Be cautious with `unsafe` methods like `set_system_env`.
* Path issues: Make sure the .env file is in the correct directory relative to the Rust project.
* File permissions: Ensure the process has read/write permissions to `.env`.

## ❤️ Support
If you encounter any issues or want to request new features, feel free to open an issue on the GitHub repository. Feedback is always welcome!

## Why Envie?
Envie makes managing environment variables simple and intuitive while maintaining Rust's type safety and performance standards. Whether you’re working on small projects or large-scale applications, Envie ensures your configuration is accessible and reliable.

## License
This project is licensed under the MPL-2.0 License. See the `LICENSE` file for details.