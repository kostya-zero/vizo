use crate::values::VizValue;

use super::Processor;
use anyhow::Result;

/// A processor that processes TOML structured data.
/// Implements [`crate::processors::Processor`] trait.
pub struct TOMLProcessor;
impl Processor for TOMLProcessor {
    fn process_data(data: &str) -> Result<VizValue> {
        let values: VizValue = toml::from_str(data)?;
        Ok(values)
    }
}
