use crate::values::VizValue;

use super::Processor;
use anyhow::{Context, Result};

/// A processor that processes YAML structured data.
/// Implements [`crate::processors::Processor`] trait.
pub struct YAMLProcessor;
impl Processor for YAMLProcessor {
    fn process_data(data: &str) -> Result<VizValue, anyhow::Error> {
        let docs = yaml_rust2::YamlLoader::load_from_str(data).context("Failed to load YAML data.")?;
        let yaml_doc = docs
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No documents found in YAML data."))?;
        let values = VizValue::from_yaml(yaml_doc);
        Ok(values)
    }
}
