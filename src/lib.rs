mod workflow;
pub use workflow::*;
mod value;
pub use value::*;
mod workflow_engine;
pub use workflow_engine::*;
mod context;
pub use context::*;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashMap;

//     #[tokio::test]
//     async fn test_workflow_engine() -> anyhow::Result<()> {
//         // Initialize the workflow engine
//         let mut engine = WorkflowEngine::new("manifest.json")?;

//         // Load the add component
//         engine.load_component("add").await?;

//         // Create test inputs
//         let mut inputs = HashMap::new();
//         inputs.insert("first".to_string(), serde_json::json!(5));
//         inputs.insert("second".to_string(), serde_json::json!(3));

//         // Execute the workflow
//         let result = engine.execute_workflow("simple_add", inputs).await?;

//         assert_eq!(result, serde_json::json!(8));
//         Ok(())
//     }
// }
