use super::Processor;
use crate::values::VizValue;
use anyhow::Result;

/// A processor that processes JSON structured data.
/// Implements [`crate::processors::Processor`] trait.
pub struct JSONProcessor;
impl Processor for JSONProcessor {
    fn process_data(data: &str) -> Result<VizValue> {
        let values = serde_json::from_str(data)?;
        Ok(values)
    }
}
