# Integration Tests

This directory contains integration tests for the ADK (Agent Development Kit) library.

## Overview

The integration tests verify that the ADK library components work correctly together in realistic scenarios. These tests complement the unit tests found in each crate.

## Test Structure

- `tests/basic_agent_test.rs` - Tests core Agent and Tool functionality
- `tests/macro_test.rs` - Tests the `tool_fn` macro and code generation

## Running the Tests

To run all integration tests:

```bash
cargo test --package integration-tests
```

To run a specific test file:

```bash
cargo test --package integration-tests basic_agent_test
cargo test --package integration-tests macro_test
```

To run with output:

```bash
cargo test --package integration-tests -- --nocapture
```

## Test Coverage

The integration tests cover:

1. **Tool Creation**: Manual tool creation by implementing the `Tool` trait
2. **Tool Execution**: Testing tool parameter validation and execution
3. **Agent Management**: Creating agents with different models and adding tools
4. **Macro Functionality**: Testing the `tool_fn` macro for automatic tool generation
5. **Error Handling**: Testing error scenarios like division by zero

## Adding New Tests

When adding new integration tests:

1. Create a new test file in the `tests/` directory
2. Follow the naming convention `*_test.rs`
3. Use `#[tokio::test]` for async tests and `#[test]` for sync tests
4. Test realistic usage scenarios rather than isolated units
5. Include both positive and negative test cases 