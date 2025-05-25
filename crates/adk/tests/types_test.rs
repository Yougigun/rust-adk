use adk::types::{Context, Message, RunContext};
use serde_json::json;

#[test]
fn test_context_creation() {
    let context = Context::new();
    assert!(context.data.is_empty());
}

#[test]
fn test_context_with_data() {
    let context = Context::new()
        .with_data("key1", "value1")
        .with_data("key2", 42)
        .with_data("key3", true);

    assert_eq!(context.data.len(), 3);
    assert_eq!(context.data["key1"], json!("value1"));
    assert_eq!(context.data["key2"], json!(42));
    assert_eq!(context.data["key3"], json!(true));
}

#[test]
fn test_context_serialization() {
    let context = Context::new()
        .with_data("name", "test")
        .with_data("count", 5);

    let serialized = serde_json::to_string(&context).unwrap();
    let deserialized: Context = serde_json::from_str(&serialized).unwrap();

    assert_eq!(context.data, deserialized.data);
}

#[test]
fn test_message_creation() {
    let message = Message {
        role: "user".to_string(),
        content: "Hello".to_string(),
        tool_name: None,
    };

    assert_eq!(message.role, "user");
    assert_eq!(message.content, "Hello");
    assert_eq!(message.tool_name, None);
}

#[test]
fn test_message_with_tool_name() {
    let message = Message {
        role: "tool".to_string(),
        content: "Tool output".to_string(),
        tool_name: Some("calculator".to_string()),
    };

    assert_eq!(message.role, "tool");
    assert_eq!(message.content, "Tool output");
    assert_eq!(message.tool_name, Some("calculator".to_string()));
}

#[test]
fn test_message_serialization() {
    let message = Message {
        role: "assistant".to_string(),
        content: "How can I help?".to_string(),
        tool_name: None,
    };

    let serialized = serde_json::to_string(&message).unwrap();
    let deserialized: Message = serde_json::from_str(&serialized).unwrap();

    assert_eq!(message.role, deserialized.role);
    assert_eq!(message.content, deserialized.content);
    assert_eq!(message.tool_name, deserialized.tool_name);
}

#[test]
fn test_run_context_creation() {
    let context = Context::new();
    let run_context = RunContext::new(context);

    assert!(run_context.messages.is_empty());
    assert!(run_context.context.data.is_empty());
}

#[test]
fn test_run_context_add_message() {
    let context = Context::new();
    let mut run_context = RunContext::new(context);

    run_context.add_message("user", "Hello");
    run_context.add_message("assistant", "Hi there!");

    assert_eq!(run_context.messages.len(), 2);
    assert_eq!(run_context.messages[0].role, "user");
    assert_eq!(run_context.messages[0].content, "Hello");
    assert_eq!(run_context.messages[0].tool_name, None);

    assert_eq!(run_context.messages[1].role, "assistant");
    assert_eq!(run_context.messages[1].content, "Hi there!");
    assert_eq!(run_context.messages[1].tool_name, None);
}

#[test]
fn test_run_context_add_tool_message() {
    let context = Context::new();
    let mut run_context = RunContext::new(context);

    run_context.add_tool_message("calculator", "Result: 42");

    assert_eq!(run_context.messages.len(), 1);
    assert_eq!(run_context.messages[0].role, "tool");
    assert_eq!(run_context.messages[0].content, "Result: 42");
    assert_eq!(
        run_context.messages[0].tool_name,
        Some("calculator".to_string())
    );
}

#[test]
fn test_run_context_with_initial_data() {
    let context = Context::new()
        .with_data("session_id", "abc123")
        .with_data("user_id", 456);

    let run_context = RunContext::new(context);

    assert_eq!(run_context.context.data["session_id"], json!("abc123"));
    assert_eq!(run_context.context.data["user_id"], json!(456));
}

#[test]
fn test_run_context_message_ordering() {
    let context = Context::new();
    let mut run_context = RunContext::new(context);

    run_context.add_message("system", "You are helpful");
    run_context.add_message("user", "What's 2+2?");
    run_context.add_tool_message("calculator", "4");
    run_context.add_message("assistant", "The answer is 4");

    assert_eq!(run_context.messages.len(), 4);
    assert_eq!(run_context.messages[0].role, "system");
    assert_eq!(run_context.messages[1].role, "user");
    assert_eq!(run_context.messages[2].role, "tool");
    assert_eq!(run_context.messages[3].role, "assistant");
}

#[test]
fn test_context_default() {
    let context = Context::default();
    assert!(context.data.is_empty());
}

#[test]
fn test_context_clone() {
    let context = Context::new().with_data("test", "value");

    let cloned = context.clone();
    assert_eq!(context.data, cloned.data);
}

#[test]
fn test_run_context_clone() {
    let context = Context::new().with_data("test", "value");
    let mut run_context = RunContext::new(context);
    run_context.add_message("user", "Hello");

    let cloned = run_context.clone();
    assert_eq!(run_context.context.data, cloned.context.data);
    assert_eq!(run_context.messages.len(), cloned.messages.len());
    assert_eq!(run_context.messages[0].content, cloned.messages[0].content);
}
