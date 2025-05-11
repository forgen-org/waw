use std::collections::HashMap;

use thiserror::Error;
use wasmtime::component::{Component, Val};

use crate::{Context, Workflow};

pub struct WorkflowEngine {
    context: Context,
    components: HashMap<String, Component>,
}

impl WorkflowEngine {
    pub fn new() -> Result<Self, WorkflowEngineError> {
        let context = Context::new()?;
        Ok(Self {
            context,
            components: HashMap::new(),
        })
    }

    pub async fn load_components(
        &mut self,
        workflow: &Workflow,
    ) -> Result<(), WorkflowEngineError> {
        let paths: Vec<String> = workflow.0.iter().map(|step| step.wasm.clone()).collect();
        for path in paths {
            if self.components.contains_key(&path) {
                continue;
            }

            let component_bytes = std::fs::read(&path)?;

            let component = Component::new(&self.context.engine, &component_bytes)?;

            self.components.insert(path.to_string(), component);
        }
        Ok(())
    }

    pub async fn execute_workflow(
        &mut self,
        workflow: &Workflow,
    ) -> Result<Option<String>, WorkflowEngineError> {
        let mut last_result = None;

        for step in &workflow.0 {
            let component = self
                .components
                .get(&step.wasm)
                .ok_or(WorkflowEngineError::ComponentNotFound(step.wasm.clone()))?;

            // Execute the component
            let instance = &self
                .context
                .linker
                .instantiate_async(&mut self.context.store, &component)
                .await?;

            // Get the function we want to call
            let func = instance
                .get_func(&mut self.context.store, &step.function)
                .ok_or(WorkflowEngineError::FunctionNotFound(step.function.clone()))?;

            // Convert inputs to component values
            let input_vals: Vec<Val> = step
                .inputs
                .iter()
                .map(|value| value.val())
                .collect::<Result<Vec<_>, _>>()?;

            // Prepare result buffer
            let mut result_vals = vec![wasmtime::component::Val::S32(0)];

            // Call the function
            func.call_async(&mut self.context.store, &input_vals, &mut result_vals)
                .await?;

            // Convert result back to JSON
            last_result = result_vals.get(0).cloned();
        }

        Ok(last_result.map(|val| val.to_wave().unwrap()))
    }
}

#[derive(Debug, Error)]
pub enum WorkflowEngineError {
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    #[error("Function not found: {0}")]
    FunctionNotFound(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Wasmtime(#[from] wasmtime::Error),
    #[error(transparent)]
    WasmWave(#[from] wasm_wave::parser::ParserError),
}
