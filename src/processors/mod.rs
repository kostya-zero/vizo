use anyhow::{Error, Result};

use crate::values::VizValue;

/// A processor for JSON.
pub mod json;

/// A processor for TOML.
pub mod toml;

/// A processor for YAML.
pub mod yaml;

/// A base trait for processors.
pub trait Processor {
    fn process_data(data: &str) -> Result<VizValue, Error>;
}
