# SWAPI Rust

A Rust-based API server for accessing Star Wars planets data.

## Project Structure
- `src/` - Source code
- `config/` - Environment configuration files
- `sql/` - Database scripts

### Building and Running with Docker
To build and run the project with Docker:
```sh
docker compose up -d --build
```

### Building and Running Locally with Cargo
To build the project:
```sh
cargo build
```
To run the project:
```sh
cargo run
```

### Running Tests
To execute all tests:
```sh
cargo test
```

### Formatting Code
To format the code using Cargo:
```sh
cargo fmt
```
