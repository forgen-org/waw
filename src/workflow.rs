use std::path::PathBuf;

use serde::Deserialize;
use thiserror::Error;

use crate::value::Value;

#[derive(Debug, Deserialize)]
pub struct Workflow(pub Vec<WorkflowStep>);

#[derive(Debug, Deserialize)]
pub struct WorkflowStep {
    pub function: String,
    pub inputs: Vec<Value>,
    pub wasm: String,
}

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}

impl Workflow {
    pub fn new(path: &PathBuf) -> Result<Self, WorkflowError> {
        let source = std::fs::read_to_string(path)?;
        let is_json = path.extension().map_or(false, |ext| ext == "json");
        Ok(match is_json {
            true => serde_json::from_str(&source)?,
            false => serde_yaml::from_str(&source)?,
        })
    }
}
