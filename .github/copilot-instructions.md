# Copilot Instructions for `json_path_parser`

## Project Overview

`json_path_parser` is a Rust library designed for JSON path retrieval and error handling. It leverages the following key dependencies:

- **nom**: For parsing operations.
- **serde** and **serde_json**: For JSON serialization and deserialization.
- **thiserror**: For structured error handling.

### Key Components

1. **`src/lib.rs`**:
   - Contains the main library logic.
   - Exposes the `get_json_value` function for retrieving values from JSON based on a path.
   - Re-exports error types (`AppError`, `AppResult`) from `src/erroe.rs`.
2. **`src/erroe.rs`**:
   - Defines the `AppError` enum for structured error handling.
   - Provides the `AppResult` type alias for consistent result handling.

### Example Usage

The library includes test cases demonstrating its usage:

- Retrieve the length of an array in JSON: `get_json_value("a.b.#", json_data)`.
- Handle errors gracefully when paths are invalid or data types mismatch.

## Developer Workflows

### Building the Project

Use Cargo to build the project:

```bash
cargo build
```

### Running Tests

Run the test suite with:

```bash
cargo test
```

### Debugging

- Use `println!` for debugging intermediate values.
- Example: Uncomment `println!` statements in `get_json_value` to trace execution.

## Project-Specific Conventions

- **Error Handling**: Use `AppError` for all error cases. Convert external errors (e.g., `nom` errors) into `AppError` variants.
- **Testing**: Place all tests in `#[cfg(test)]` modules. Use `serde_json::json!` for constructing test data.
- **JSON Path Parsing**: Paths are delimited by `.`. Use `#` to denote array length retrieval.

## Integration Points

- **Dependencies**:
  - `nom`: Used for parsing JSON paths.
  - `serde_json`: Used for JSON manipulation.
  - `thiserror`: Used for error definitions.
- **Cross-Component Communication**:
  - `lib.rs` re-exports types from `erroe.rs` to ensure consistent API exposure.

## Key Files

- `src/lib.rs`: Main library logic.
- `src/erroe.rs`: Error handling definitions.
- `Cargo.toml`: Dependency and project configuration.

## Notes for AI Agents

- Follow the existing patterns for error handling and testing.
- Ensure all new functions are covered by unit tests.
- Maintain compatibility with the existing API structure.

For further clarification, refer to the test cases in `src/lib.rs`.
