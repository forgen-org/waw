use std::path::PathBuf;

use clap::Parser;
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
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let workflow = Workflow::new(&args.workflow)?;
    let engine = WorkflowEngine::new()?;
    let res = engine.execute_workflow(&workflow).await?;
    println!("Result: {:?}", res);
    Ok(())
}
