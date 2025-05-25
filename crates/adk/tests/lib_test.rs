use adk::prelude::*;
use adk::{AgentError, Tool, ToolResult};

#[test]
fn test_prelude_exports() {
    // Test that all expected items are available from prelude
    let _error = AgentError::ModelError("test".to_string());
    let _context = Context::new();
}

#[test]
fn test_main_exports() {
    // Test that main exports are available
    use adk::{AgentError, Tool, ToolResult};

    // This test mainly verifies compilation and availability of types
    assert!(true);
}

#[test]
fn test_module_structure() {
    // Test that modules are properly structured
    use adk::agent::AgentBuilder;
    use adk::error::AgentError;
    use adk::openai::OpenAI;
    use adk::tool::FunctionTool;
    use adk::types::{Context, RunContext};

    // This test mainly verifies compilation and module organization
    assert!(true);
}

#[test]
fn test_macro_availability() {
    // Test that macros are available
    use adk::function_tool;

    // Create a simple tool using the macro
    let _tool = function_tool!("test", "test tool", |_context, _params| {
        Ok(ToolResult {
            tool_name: "test".to_string(),
            output: "test".to_string(),
        })
    });

    assert!(true);
}

#[test]
fn test_re_exports() {
    // Test that re-exports work correctly
    use adk::{Agent, AgentError, Model, Tool, ToolResult};

    // Verify that types are accessible
    fn _type_check() -> Option<Box<dyn Tool>> {
        None
    }

    assert!(true);
}

#[test]
fn test_prelude_convenience() {
    // Test that prelude includes commonly used items
    // Without explicit imports, these should be available
    let _context = Context::new();
    let _run_context = RunContext::new(_context);

    // Test that tool_fn is in prelude
    // Note: Can't easily test macro compilation here without conflicts
    assert!(true);
}

#[test]
fn test_error_integration() {
    // Test that errors integrate well with the type system
    let result: Result<String, AgentError> = Err(AgentError::ModelError("test".to_string()));

    assert!(result.is_err());

    let error = result.unwrap_err();
    match error {
        AgentError::ModelError(msg) => assert_eq!(msg, "test"),
        _ => panic!("Unexpected error type"),
    }
}

#[test]
fn test_tool_result_integration() {
    // Test that ToolResult integrates well
    let result = ToolResult {
        tool_name: "integration_test".to_string(),
        output: "test output".to_string(),
    };

    assert_eq!(result.tool_name, "integration_test");
    assert_eq!(result.output, "test output");

    // Test cloning
    let cloned = result.clone();
    assert_eq!(result.tool_name, cloned.tool_name);
}

#[test]
fn test_context_integration() {
    // Test that Context and RunContext work together
    let context = Context::new()
        .with_data("test_key", "test_value")
        .with_data("number", 42);

    let mut run_context = RunContext::new(context);
    run_context.add_message("user", "Hello");
    run_context.add_tool_message("test_tool", "Tool response");

    assert_eq!(run_context.messages.len(), 2);
    assert_eq!(run_context.context.data.len(), 2);
}
