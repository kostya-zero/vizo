use anyhow::{Context, Error};
use serde_json::Value;
use anyhow::Result;

use super::Processor;

/// A processor that processes JSON structured data.
/// Implements [`crate::processors::Processor`] trait.
pub struct JSONProcessor;
impl Processor for JSONProcessor {
    fn process_data(data: &str) -> Result<crate::values::VizValue, Error> {
        let values: Value = serde_json::from_str::<Value>(data).context("Failed to parse JSON data.")?;

        Ok(crate::values::VizValue::from_serde_json(values))
    }
}
