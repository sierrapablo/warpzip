# Project Style Guide

This document outlines the coding style and best practices to follow when contributing to **WarpZip**. Following these conventions ensures that the project remains consistent, clean, and maintainable.

## General Guidelines

1. **Use Rust's Official Style**: Follow the [Rust style guide](https://doc.rust-lang.org/nightly/style-guide/) for indentation, line breaks, and general formatting.
2. **Indentation**: Use 4 spaces for indentation (do not use tabs).
3. **Line Length**: Try to limit lines to 100 characters. If a line exceeds this length, break it into multiple lines as needed.
4. **Function Naming**: Use `snake_case` for function names (e.g., `my_function()`).
5. **Variable Naming**: Use `snake_case` for variables, and be descriptive (e.g., `compression_level`).
6. **Constants**: Use `UPPERCASE_SNAKE_CASE` for constants (e.g., `MAX_BUFFER_SIZE`).
7. **Enums and Structs**: Use `CamelCase` for enum and struct names (e.g., `CompressionMethod`).
8. **Documentation**: Use comments to explain "why" something is being done, not "what". Write documentation for all public functions and types using `///`.
9. **Error Handling**: Use `Result<T, E>` for functions that can fail. Prefer returning `Err()` with a descriptive error message.
10. **Avoid Unnecessary Comments**: Do not add comments for obvious code. Use comments to explain complex logic or non-obvious decisions.

## Code Structure

1. **Modules**: Break down the code into logical modules. For example, keep compression methods in a separate module like `compression.rs`.
2. **Error Handling**: Handle errors gracefully and avoid panic where possible.
3. **Tests**: Add unit tests for all public functions. Use `#[cfg(test)]` to mark test modules and `#[test]` for individual tests.

## Commit Messages

Please refer to the [Contributing](README.md#contributing) section for guidelines on commit messages and PR practices.

---

By following these guidelines, you help maintain the quality and consistency of the project.
