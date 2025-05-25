use adk::ToolResult;
use adk::agent::{Agent, AgentBuilder};
use adk::error::AgentError;
use adk::openai::Model;
use adk::tool::Tool;
use adk::types::{Context, RunContext};
use async_trait::async_trait;
use std::sync::Arc;

// Mock model for testing
#[derive(Debug)]
struct MockModel {
    response: String,
}

impl MockModel {
    fn new(response: &str) -> Self {
        Self {
            response: response.to_string(),
        }
    }
}

#[async_trait]
impl Model for MockModel {
    async fn generate_response(
        &self,
        _context: &mut RunContext,
        _tools: &[&dyn Tool],
    ) -> Result<String, AgentError> {
        Ok(self.response.clone())
    }
}

// Mock tool for testing
#[derive(Debug)]
struct MockTool {
    name: String,
}

impl MockTool {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Tool for MockTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "A mock tool for testing"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    }

    async fn execute(
        &self,
        _context: &mut RunContext,
        _params: &str,
    ) -> Result<ToolResult, AgentError> {
        Ok(ToolResult {
            tool_name: self.name.clone(),
            output: "mock output".to_string(),
        })
    }
}

#[test]
fn test_agent_creation() {
    let model = Arc::new(MockModel::new("test response"));
    let tool: Arc<dyn Tool> = Arc::new(MockTool::new("test_tool"));
    let tools = vec![tool.clone()];

    let agent = Agent::new(
        "test_agent",
        Some("Test instructions".to_string()),
        model,
        tools,
    );

    assert_eq!(agent.name(), "test_agent");
    assert_eq!(agent.instructions(), Some("Test instructions"));
    assert_eq!(agent.tools().len(), 1);
    assert_eq!(agent.tools()[0].name(), "test_tool");
}

#[test]
fn test_agent_creation_without_instructions() {
    let model = Arc::new(MockModel::new("test response"));
    let agent = Agent::new("test_agent", None, model, vec![]);

    assert_eq!(agent.name(), "test_agent");
    assert_eq!(agent.instructions(), None);
    assert_eq!(agent.tools().len(), 0);
}

#[tokio::test]
async fn test_agent_run() {
    let model = Arc::new(MockModel::new("Hello, World!"));
    let agent = Agent::new(
        "test_agent",
        Some("You are helpful".to_string()),
        model,
        vec![],
    );

    let context = Context::new();
    let result = agent.run("Hello", context).await.unwrap();

    assert_eq!(result, "Hello, World!");
}

#[tokio::test]
async fn test_agent_run_with_tools() {
    let model = Arc::new(MockModel::new("Tool response"));
    let tool: Arc<dyn Tool> = Arc::new(MockTool::new("test_tool"));
    let agent = Agent::new("test_agent", None, model, vec![tool]);

    let context = Context::new();
    let result = agent.run("Use the tool", context).await.unwrap();

    assert_eq!(result, "Tool response");
}

#[test]
fn test_agent_builder_basic() {
    let model = Arc::new(MockModel::new("test response"));

    let agent = AgentBuilder::new("builder_agent")
        .model(model)
        .build()
        .unwrap();

    assert_eq!(agent.name(), "builder_agent");
    assert_eq!(agent.instructions(), None);
    assert_eq!(agent.tools().len(), 0);
}

#[test]
fn test_agent_builder_with_instructions() {
    let model = Arc::new(MockModel::new("test response"));

    let agent = AgentBuilder::new("builder_agent")
        .instructions("You are a test agent")
        .model(model)
        .build()
        .unwrap();

    assert_eq!(agent.name(), "builder_agent");
    assert_eq!(agent.instructions(), Some("You are a test agent"));
}

#[test]
fn test_agent_builder_with_tools() {
    let model = Arc::new(MockModel::new("test response"));
    let tool1: Arc<dyn Tool> = Arc::new(MockTool::new("tool1"));
    let tool2: Arc<dyn Tool> = Arc::new(MockTool::new("tool2"));

    let agent = AgentBuilder::new("builder_agent")
        .model(model)
        .add_tool(tool1)
        .add_tool(tool2)
        .build()
        .unwrap();

    assert_eq!(agent.tools().len(), 2);
    assert_eq!(agent.tools()[0].name(), "tool1");
    assert_eq!(agent.tools()[1].name(), "tool2");
}

#[test]
fn test_agent_builder_missing_model() {
    let result = AgentBuilder::new("builder_agent")
        .instructions("Test instructions")
        .build();

    assert!(result.is_err());
    let error = result.err().unwrap();
    match error {
        AgentError::ConfigurationError(msg) => {
            assert_eq!(msg, "Model not set");
        }
        _ => panic!("Expected ConfigurationError"),
    }
}

#[test]
fn test_agent_builder_fluent_interface() {
    let model = Arc::new(MockModel::new("test response"));
    let tool: Arc<dyn Tool> = Arc::new(MockTool::new("test_tool"));

    let agent = AgentBuilder::new("fluent_agent")
        .instructions("Fluent instructions")
        .model(model)
        .add_tool(tool)
        .build()
        .unwrap();

    assert_eq!(agent.name(), "fluent_agent");
    assert_eq!(agent.instructions(), Some("Fluent instructions"));
    assert_eq!(agent.tools().len(), 1);
    assert_eq!(agent.tools()[0].name(), "test_tool");
}
