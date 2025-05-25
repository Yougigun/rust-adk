# Contributing to Rust ADK

Thank you for your interest in contributing to the Rust Agent Development Kit (ADK)! This guide will help you get started with contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Create a new branch for your changes
4. Make your changes
5. Test your changes
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Cargo

### Building the Project

```bash
cargo build
```

### Running Tests

We have comprehensive test coverage with both unit and integration tests:

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run tests for a specific crate
cargo test -p adk
```

## Commit Message Convention

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages. This helps us maintain a clean and readable commit history.

### Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat:` - A new feature
- `fix:` - A bug fix
- `test:` - Adding missing tests or correcting existing tests
- `docs:` - Documentation only changes
- `refactor:` - A code change that neither fixes a bug nor adds a feature
- `perf:` - A code change that improves performance
- `chore:` - Changes to the build process or auxiliary tools
- `ci:` - Changes to CI configuration files and scripts
- `style:` - Changes that do not affect the meaning of the code (formatting, etc.)

### Examples

```bash
feat: add new tool execution middleware
fix: resolve memory leak in agent runtime
test: add integration tests for macro functionality
docs: update README with usage examples
refactor: simplify error handling in core module
```

### Scope (Optional)

You can add a scope to provide additional context:

```bash
feat(agent): add support for custom model providers
fix(macros): resolve compilation error with async tools
test(integration): add end-to-end workflow tests
```

## Testing Guidelines

### Unit Tests

- Located in `crates/adk/tests/` directory
- Test individual modules and functions in isolation
- Use mock objects when testing components with external dependencies
- Follow the naming convention: `<module>_test.rs`

### Integration Tests

- Located in `integration/tests/` directory
- Test end-to-end workflows and cross-module interactions
- Verify that different components work together correctly

### Writing Good Tests

1. **Descriptive names**: Test function names should clearly describe what is being tested
2. **Arrange-Act-Assert**: Structure tests with clear setup, execution, and verification phases
3. **Test edge cases**: Include tests for error conditions and boundary cases
4. **Use mocks**: Isolate units under test by mocking external dependencies
5. **Async testing**: Use `#[tokio::test]` for testing async functionality

### Example Test Structure

```rust
#[tokio::test]
async fn test_agent_executes_tool_successfully() {
    // Arrange
    let mock_tool = Arc::new(MockTool::new("test_tool"));
    let agent = AgentBuilder::new()
        .model("test-model")
        .add_tool(mock_tool.clone())
        .build()
        .unwrap();

    // Act
    let result = agent.execute_tool("test_tool", &json!({})).await;

    // Assert
    assert!(result.is_ok());
    assert_eq!(mock_tool.call_count(), 1);
}
```

## Code Style

- Follow Rust standard formatting with `cargo fmt`
- Use `cargo clippy` to catch common mistakes
- Add documentation comments for public APIs
- Keep functions focused and single-purpose

## Pull Request Process

1. **Branch naming**: Use descriptive branch names (e.g., `feat/add-custom-models`, `fix/memory-leak`)
2. **PR title**: Follow conventional commit format for the title
3. **Description**: Provide a clear description of what your PR does
4. **Tests**: Ensure all tests pass and add tests for new functionality
5. **Documentation**: Update documentation if you're changing public APIs

### PR Title Examples

```
feat: add support for custom model providers
fix: resolve memory leak in agent execution
test: add comprehensive test suite with 66 tests
docs: improve getting started guide
```

## Questions?

If you have questions about contributing, feel free to:
- Open an issue for discussion
- Check existing issues and discussions
- Reach out to the maintainers

Thank you for contributing to Rust ADK! 