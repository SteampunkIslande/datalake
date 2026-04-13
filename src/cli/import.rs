use clap::Parser;

use crate::cli::Cli;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Import {}

impl Import {
    pub fn run(&self, _cli: &Cli) -> anyhow::Result<()> {
        Ok(())
    }
}
