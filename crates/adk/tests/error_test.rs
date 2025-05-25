use adk::error::{AgentError, AgentResult};

#[test]
fn test_model_error() {
    let error = AgentError::ModelError("Model failed".to_string());
    assert_eq!(error.to_string(), "Model error: Model failed");
}

#[test]
fn test_tool_error() {
    let error = AgentError::ToolError("Tool execution failed".to_string());
    assert_eq!(
        error.to_string(),
        "Tool execution error: Tool execution failed"
    );
}

#[test]
fn test_invalid_input_error() {
    let error = AgentError::InvalidInput("Invalid parameter".to_string());
    assert_eq!(error.to_string(), "Invalid input: Invalid parameter");
}

#[test]
fn test_context_error() {
    let error = AgentError::ContextError("Context is invalid".to_string());
    assert_eq!(error.to_string(), "Context error: Context is invalid");
}

#[test]
fn test_configuration_error() {
    let error = AgentError::ConfigurationError("Missing API key".to_string());
    assert_eq!(error.to_string(), "Configuration error: Missing API key");
}

#[test]
fn test_internal_error() {
    let error = AgentError::InternalError("Something went wrong".to_string());
    assert_eq!(error.to_string(), "Internal error: Something went wrong");
}

#[test]
fn test_serialization_error_conversion() {
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json");
    assert!(json_error.is_err());

    let agent_error: AgentError = json_error.unwrap_err().into();
    match agent_error {
        AgentError::SerializationError(_) => {
            // Expected
        }
        _ => panic!("Expected SerializationError"),
    }
}

#[test]
fn test_agent_result_ok() {
    let result: AgentResult<String> = Ok("success".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[test]
fn test_agent_result_err() {
    let result: AgentResult<String> = Err(AgentError::ModelError("failed".to_string()));
    assert!(result.is_err());

    match result.unwrap_err() {
        AgentError::ModelError(msg) => assert_eq!(msg, "failed"),
        _ => panic!("Expected ModelError"),
    }
}

#[test]
fn test_error_debug_formatting() {
    let error = AgentError::ToolError("debug test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("ToolError"));
    assert!(debug_str.contains("debug test"));
}

#[test]
fn test_error_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AgentError>();
}

#[test]
fn test_error_from_serde_json() {
    let json_str = r#"{"invalid": json}"#;
    let json_result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(json_str);

    assert!(json_result.is_err());

    let agent_error: AgentError = json_result.unwrap_err().into();
    match agent_error {
        AgentError::SerializationError(_) => {
            // This is expected
        }
        _ => panic!("Expected SerializationError"),
    }
}

#[test]
fn test_error_chain() {
    // Test that we can chain different error types
    let result: AgentResult<()> = Err(AgentError::ModelError("model failed".to_string()));

    let mapped_result: AgentResult<String> = result.map(|_| "success".to_string());

    assert!(mapped_result.is_err());
    match mapped_result.unwrap_err() {
        AgentError::ModelError(msg) => assert_eq!(msg, "model failed"),
        _ => panic!("Expected ModelError"),
    }
}

#[test]
fn test_error_variants_exhaustive() {
    // Test that we can create all error variants
    let errors = vec![
        AgentError::ModelError("test".to_string()),
        AgentError::ToolError("test".to_string()),
        AgentError::InvalidInput("test".to_string()),
        AgentError::ContextError("test".to_string()),
        AgentError::ConfigurationError("test".to_string()),
        AgentError::InternalError("test".to_string()),
        AgentError::SerializationError(
            serde_json::from_str::<serde_json::Value>("invalid").unwrap_err(),
        ),
    ];

    assert_eq!(errors.len(), 7);

    // Each error should be debuggable
    for error in errors {
        let debug_str = format!("{:?}", error);
        assert!(!debug_str.is_empty());
    }
}
