use std::path::PathBuf;

use clap::Parser;
use thiserror::Error;
use waw::{Workflow, WorkflowEngine};

/// A CLI tool for executing workflows
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the workflow manifest file
    #[arg(short, long)]
    workflow: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let args = Args::parse();
    let workflow = Workflow::new(&args.workflow)?;
    let mut engine = WorkflowEngine::new()?;
    engine.load_components(&workflow).await?;
    let res = engine.execute_workflow(&workflow).await?;
    println!("Result: {:?}", res);
    Ok(())
}

#[derive(Debug, Error)]
pub enum MainError {
    #[error(transparent)]
    WorkflowError(#[from] waw::WorkflowError),
    #[error(transparent)]
    WorkflowEngineError(#[from] waw::WorkflowEngineError),
}
