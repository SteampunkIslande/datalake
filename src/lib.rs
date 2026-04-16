pub mod backend;

use std::path::PathBuf;

pub use backend::import::*;

#[derive(Debug)]
pub struct DatalakeConfig {
    pub datalake_path: PathBuf,
}
