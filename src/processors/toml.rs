use crate::values::VizValues;

use super::Processor;
use anyhow::{Context, Result};

/// A processor that processes TOML structured data.
/// Implements [`crate::processors::Processor`] trait.
pub struct TOMLProcessor;
impl Processor for TOMLProcessor {
    fn process_data(data: &str) -> Result<VizValues, anyhow::Error> {
        let values: VizValues = toml::from_str(data).context("Failed to parse TOML data.")?;
        Ok(values)
    }
}
