use adk::tool::{FunctionTool, Tool, ToolResult};
use adk::types::{Context, RunContext};
use adk::{AgentError, function_tool};
use async_trait::async_trait;

// Simple test tool
#[derive(Debug)]
struct TestTool;

#[async_trait]
impl Tool for TestTool {
    fn name(&self) -> &str {
        "test_tool"
    }

    fn description(&self) -> &str {
        "A simple test tool"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "input": {"type": "string"}
            },
            "required": ["input"]
        })
    }

    async fn execute(
        &self,
        _context: &mut RunContext,
        params: &str,
    ) -> Result<ToolResult, AgentError> {
        let params: serde_json::Value = serde_json::from_str(params)?;
        let input = params["input"].as_str().unwrap_or("default");

        Ok(ToolResult {
            tool_name: self.name().to_string(),
            output: format!("Processed: {}", input),
        })
    }
}

#[test]
fn test_tool_properties() {
    let tool = TestTool;

    assert_eq!(tool.name(), "test_tool");
    assert_eq!(tool.description(), "A simple test tool");

    let schema = tool.parameters_schema();
    assert_eq!(schema["type"], "object");
    assert!(schema["properties"].is_object());
    assert!(schema["required"].is_array());
}

#[tokio::test]
async fn test_tool_execution() {
    let tool = TestTool;
    let mut context = RunContext::new(Context::new());

    let params = r#"{"input": "hello world"}"#;
    let result = tool.execute(&mut context, params).await.unwrap();

    assert_eq!(result.tool_name, "test_tool");
    assert_eq!(result.output, "Processed: hello world");
}

#[tokio::test]
async fn test_tool_execution_invalid_json() {
    let tool = TestTool;
    let mut context = RunContext::new(Context::new());

    let params = "invalid json";
    let result = tool.execute(&mut context, params).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_tool_execution_missing_parameter() {
    let tool = TestTool;
    let mut context = RunContext::new(Context::new());

    let params = r#"{}"#;
    let result = tool.execute(&mut context, params).await.unwrap();

    // Should use default value
    assert_eq!(result.tool_name, "test_tool");
    assert_eq!(result.output, "Processed: default");
}

#[test]
fn test_tool_result_creation() {
    let result = ToolResult {
        tool_name: "my_tool".to_string(),
        output: "some output".to_string(),
    };

    assert_eq!(result.tool_name, "my_tool");
    assert_eq!(result.output, "some output");
}

#[test]
fn test_tool_result_clone() {
    let result = ToolResult {
        tool_name: "my_tool".to_string(),
        output: "some output".to_string(),
    };

    let cloned = result.clone();
    assert_eq!(result.tool_name, cloned.tool_name);
    assert_eq!(result.output, cloned.output);
}

#[test]
fn test_function_tool_creation() {
    let tool = function_tool!("echo_tool", "Echoes the input", |_context, params| {
        Ok(ToolResult {
            tool_name: "echo_tool".to_string(),
            output: format!("Echo: {}", params),
        })
    });

    assert_eq!(tool.name(), "echo_tool");
    assert_eq!(tool.description(), "Echoes the input");
}

#[tokio::test]
async fn test_function_tool_execution() {
    let tool = function_tool!("double_tool", "Doubles a number", |_context, params| {
        let num: i32 = params.parse().unwrap_or(0);
        Ok(ToolResult {
            tool_name: "double_tool".to_string(),
            output: (num * 2).to_string(),
        })
    });

    let mut context = RunContext::new(Context::new());
    let result = tool.execute(&mut context, "21").await.unwrap();

    assert_eq!(result.tool_name, "double_tool");
    assert_eq!(result.output, "42");
}

#[test]
fn test_function_tool_with_schema() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "value": {"type": "number"}
        },
        "required": ["value"]
    });

    let tool = function_tool!(
        "square_tool",
        "Squares a number",
        schema,
        |_context, params| {
            let params: serde_json::Value = serde_json::from_str(params)?;
            let value = params["value"].as_f64().unwrap_or(0.0);
            Ok(ToolResult {
                tool_name: "square_tool".to_string(),
                output: (value * value).to_string(),
            })
        }
    );

    assert_eq!(tool.name(), "square_tool");
    assert_eq!(tool.description(), "Squares a number");
    assert_eq!(tool.parameters_schema()["type"], "object");
}

#[tokio::test]
async fn test_function_tool_with_schema_execution() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "value": {"type": "number"}
        },
        "required": ["value"]
    });

    let tool = function_tool!(
        "square_tool",
        "Squares a number",
        schema,
        |_context, params| {
            let params: serde_json::Value = serde_json::from_str(params)?;
            let value = params["value"].as_f64().unwrap_or(0.0);
            Ok(ToolResult {
                tool_name: "square_tool".to_string(),
                output: (value * value).to_string(),
            })
        }
    );

    let mut context = RunContext::new(Context::new());
    let result = tool
        .execute(&mut context, r#"{"value": 5.0}"#)
        .await
        .unwrap();

    assert_eq!(result.tool_name, "square_tool");
    assert_eq!(result.output, "25");
}

#[test]
fn test_function_tool_new() {
    let tool = FunctionTool::new(
        "custom_tool",
        "Custom description",
        serde_json::json!({"type": "object"}),
        Box::new(|_context, _params| {
            Ok(ToolResult {
                tool_name: "custom_tool".to_string(),
                output: "custom output".to_string(),
            })
        }),
    );

    assert_eq!(tool.name(), "custom_tool");
    assert_eq!(tool.description(), "Custom description");
    assert_eq!(
        tool.parameters_schema(),
        serde_json::json!({"type": "object"})
    );
}

#[tokio::test]
async fn test_function_tool_new_execution() {
    let tool = FunctionTool::new(
        "custom_tool",
        "Custom description",
        serde_json::json!({"type": "object"}),
        Box::new(|_context, params| {
            Ok(ToolResult {
                tool_name: "custom_tool".to_string(),
                output: format!("Received: {}", params),
            })
        }),
    );

    let mut context = RunContext::new(Context::new());
    let result = tool.execute(&mut context, "test input").await.unwrap();

    assert_eq!(result.tool_name, "custom_tool");
    assert_eq!(result.output, "Received: test input");
}
