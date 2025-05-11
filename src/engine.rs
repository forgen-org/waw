use std::collections::HashMap;

use anyhow::Result;
use wasmtime::{
    component::{Component, Linker, Val},
    Config, Engine, Store,
};

use crate::Workflow;

pub struct WorkflowEngine {
    engine: Engine,
    components: HashMap<String, Component>,
}

impl WorkflowEngine {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;
        Ok(Self {
            engine,
            components: HashMap::new(),
        })
    }

    pub async fn load_components(&mut self, workflow: &Workflow) -> Result<()> {
        let paths: Vec<String> = workflow.0.iter().map(|step| step.wasm.clone()).collect();
        for path in paths {
            if self.components.contains_key(&path) {
                continue;
            }

            let component_bytes = std::fs::read(&path)
                .map_err(|e| anyhow::anyhow!("Failed to read component file '{}': {}", path, e))?;

            let component = Component::new(&self.engine, &component_bytes)?;

            self.components.insert(path.to_string(), component);
        }
        Ok(())
    }

    pub async fn execute_workflow(&self, workflow: &Workflow) -> Result<serde_json::Value> {
        let mut store = Store::new(&self.engine, ());
        let mut last_result = None;

        for step in &workflow.0 {
            let component = self
                .components
                .get(&step.wasm)
                .ok_or_else(|| anyhow::anyhow!("Component '{}' not found", step.wasm,))?;

            // Process inputs using template variables
            // let processed_inputs: Vec<serde_json::Value> = step
            //     .inputs
            //     .iter()
            //     .map(|value| {
            //         // if template.starts_with("${workflow.input.") {
            //         //     let key = template
            //         //         .trim_start_matches("${workflow.input.")
            //         //         .trim_end_matches("}");
            //         //     inputs.get(key).cloned().unwrap_or(serde_json::Value::Null)
            //         // } else {
            //         //     serde_json::Value::String(template.to_string())
            //         // }
            //     })
            //     .collect();

            // Execute the component
            let linker = Linker::new(&self.engine);
            let instance = linker.instantiate(&mut store, &component)?;

            // Get the function we want to call
            let func = instance
                .get_func(&mut store, &step.function)
                .ok_or_else(|| anyhow::anyhow!("Function not found"))?;
            // Convert inputs to component values
            let input_vals: Vec<Val> = step.inputs.iter().map(|value| value.val()).collect();

            // Prepare result buffer
            let mut result_vals = vec![wasmtime::component::Val::S32(0)];

            // Call the function
            func.call(&mut store, &input_vals, &mut result_vals)?;

            // Convert result back to JSON
            last_result = Some(match result_vals.get(0) {
                Some(Val::S32(n)) => serde_json::Value::Number((*n).into()),
                _ => serde_json::Value::Null,
            });
        }

        Ok(last_result.unwrap_or(serde_json::Value::Null))
    }
}
