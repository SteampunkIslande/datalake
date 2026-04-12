use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct Import {}

impl Import {
    pub fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
