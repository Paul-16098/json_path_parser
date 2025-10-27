# json_path_parser

this is a Rust library designed for JSON path retrieval and error handling. It provides a robust API for extracting values from JSON data based on specified paths, while ensuring structured error handling.

## Technology Stack

The project leverages the following technologies:

- **Rust**: Programming language
- **nom**: For parsing operations
- **serde** and **serde_json**: For JSON serialization and deserialization
- **thiserror**: For structured error handling

## Project Architecture

The library is structured into the following key components:

1. **`src/lib.rs`**:
   - Contains the main library logic.
   - Exposes the `get_json_value` function for retrieving values from JSON based on a path.
   - Re-exports error types (`AppError`, `AppResult`) from `src/erroe.rs`.
2. **`src/erroe.rs`**:
   - Defines the `AppError` enum for structured error handling.
   - Provides the `AppResult` type alias for consistent result handling.

## Getting Started

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   ```

2. Navigate to the project directory:

   ```bash
   cd xxx
   ```

3. Build the project:

   ```bash
   cargo build
   ```

### Running Tests

Run the test suite with:

```bash
cargo test
```

## Project Structure

The project follows this folder organization:

- `src/lib.rs`: Main library logic.
- `src/erroe.rs`: Error handling definitions.
- `Cargo.toml`: Dependency and project configuration.

## Key Features

- Retrieve JSON values based on paths.
- Handle errors gracefully for invalid paths or data type mismatches.
- Retrieve array lengths in JSON using the `#` symbol in paths.

## Development Workflow

- **Building**: Use `cargo build` to compile the project.
- **Testing**: Use `cargo test` to run the test suite.

## Coding Standards

- Use `AppError` for all error cases.
- Convert external errors (e.g., `nom` errors) into `AppError` variants.
- Place all tests in `#[cfg(test)]` modules.
- Use `serde_json::json!` for constructing test data.

## Testing

The library includes test cases demonstrating its usage:

- Retrieve the length of an array in JSON:

  ```rust
  get_json_value("a.b.#", json_data);
  ```

- Handle errors gracefully when paths are invalid or data types mismatch.

## Contributing

- Follow the existing patterns for error handling and testing.
- Ensure all new functions are covered by unit tests.
- Maintain compatibility with the existing API structure.

For further clarification, refer to the test cases in `src/lib.rs`.
