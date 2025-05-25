use adk::agent::AgentBuilder;
use adk::openai::OpenAI;
use adk::prelude::*;
use adk::{AgentError, Tool, ToolResult};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct CalculatorTool;

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Performs basic arithmetic operations"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["add", "subtract", "multiply", "divide"]
                },
                "a": {"type": "number"},
                "b": {"type": "number"}
            },
            "required": ["operation", "a", "b"]
        })
    }

    async fn execute(
        &self,
        _context: &mut RunContext,
        params: &str,
    ) -> Result<ToolResult, AgentError> {
        let params: serde_json::Value = serde_json::from_str(params)?;

        let operation = params["operation"]
            .as_str()
            .ok_or_else(|| AgentError::InvalidInput("Missing operation".into()))?;
        let a = params["a"]
            .as_f64()
            .ok_or_else(|| AgentError::InvalidInput("Invalid first number".into()))?;
        let b = params["b"]
            .as_f64()
            .ok_or_else(|| AgentError::InvalidInput("Invalid second number".into()))?;

        let result = match operation {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Err(AgentError::InvalidInput("Division by zero".into()));
                }
                a / b
            }
            _ => return Err(AgentError::InvalidInput("Invalid operation".into())),
        };

        Ok(ToolResult {
            tool_name: self.name().to_string(),
            output: result.to_string(),
        })
    }
}

#[tokio::test]
async fn test_tool_creation_and_execution() {
    let calculator = CalculatorTool;

    // Test tool properties
    assert_eq!(calculator.name(), "calculator");
    assert_eq!(
        calculator.description(),
        "Performs basic arithmetic operations"
    );

    // Test parameter schema
    let params = calculator.parameters_schema();
    assert!(params.is_object());

    // Create a mock context for testing
    let mut context = RunContext::new(Context::new());

    // Test tool execution - addition
    let add_params = r#"{"operation": "add", "a": 5.0, "b": 3.0}"#;
    let result = calculator.execute(&mut context, add_params).await.unwrap();
    assert_eq!(result.tool_name, "calculator");
    assert_eq!(result.output, "8");

    // Test tool execution - division
    let div_params = r#"{"operation": "divide", "a": 10.0, "b": 2.0}"#;
    let result = calculator.execute(&mut context, div_params).await.unwrap();
    assert_eq!(result.tool_name, "calculator");
    assert_eq!(result.output, "5");

    // Test error handling - division by zero
    let div_zero_params = r#"{"operation": "divide", "a": 10.0, "b": 0.0}"#;
    let result = calculator.execute(&mut context, div_zero_params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_agent_builder() {
    // Create a mock OpenAI model (this won't actually call the API)
    let model = Arc::new(OpenAI::new("test-key", "gpt-4"));

    // Create the calculator tool
    let calculator = Arc::new(CalculatorTool);

    // Test agent creation using builder pattern
    let agent = AgentBuilder::new("test_agent")
        .instructions("You are a helpful test agent.")
        .model(model)
        .add_tool(calculator)
        .build()
        .unwrap();

    // Test that the agent was created successfully
    assert_eq!(agent.name(), "test_agent");
    assert_eq!(agent.instructions(), Some("You are a helpful test agent."));
    assert_eq!(agent.tools().len(), 1);
    assert_eq!(agent.tools()[0].name(), "calculator");
}

#[test]
fn test_tool_schema_validation() {
    let calculator = CalculatorTool;
    let schema = calculator.parameters_schema();

    // Test schema structure
    assert_eq!(schema["type"], "object");
    assert!(schema["properties"].is_object());
    assert!(schema["required"].is_array());

    // Test that required fields are present
    let required = schema["required"].as_array().unwrap();
    assert!(required.contains(&serde_json::Value::String("operation".to_string())));
    assert!(required.contains(&serde_json::Value::String("a".to_string())));
    assert!(required.contains(&serde_json::Value::String("b".to_string())));
}

#[test]
fn test_error_types() {
    // Test that our error types can be created and used
    use adk::AgentError;

    let _error1 = AgentError::ModelError("test error".to_string());
    let _error2 = AgentError::ToolError("tool error".to_string());
    let _error3 = AgentError::InvalidInput("invalid input".to_string());
    let _error4 = AgentError::ContextError("context error".to_string());
    let _error5 = AgentError::ConfigurationError("config error".to_string());
}

#[test]
fn test_tool_result_creation() {
    let result = ToolResult {
        tool_name: "test_tool".to_string(),
        output: "test output".to_string(),
    };

    assert_eq!(result.tool_name, "test_tool");
    assert_eq!(result.output, "test output");
}
