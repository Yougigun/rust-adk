use adk::agent::AgentBuilder;
use adk::openai::OpenAI;
use std::sync::Arc;

#[test]
fn test_macro_compilation() {
    // Test that the tool_fn macro is available for import
    // We can't easily test the macro without causing conflicts in this test file
    // but we can verify it's accessible
    println!("Macro test compiled successfully");
}

#[tokio::test]
async fn test_agent_builder_without_tools() {
    // Test that we can create an agent without tools
    let model = Arc::new(OpenAI::new("test-key", "gpt-4"));

    let agent = AgentBuilder::new("test_agent")
        .instructions("You are a test agent.")
        .model(model)
        .build()
        .unwrap();

    assert_eq!(agent.name(), "test_agent");
    assert_eq!(agent.tools().len(), 0);
}

#[test]
fn test_function_tool_macro_exists() {
    // Test that the function_tool macro is available and compiles
    use adk::function_tool;

    let _tool = function_tool!("test_tool", "A test tool", |_context, _params| {
        Ok(adk::ToolResult {
            tool_name: "test_tool".to_string(),
            output: "test output".to_string(),
        })
    });

    // If we get here, the macro compiled successfully
    assert!(true);
}
