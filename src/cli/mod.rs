use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod import;
pub use import::Import;

mod serve;
pub use serve::Serve;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long)]
    pub datalake_path: PathBuf,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Imports a raw parquet file (from vcf2parquet)
    Import(Import),
    /// Starts a rocket server to ask and enrich the datalake
    Serve(Serve),
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Import(import) => import.run(self),
            Commands::Serve(serve) => serve.run(self),
        }
    }
}
