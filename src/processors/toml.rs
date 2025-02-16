use super::Processor;

/// A processor that processes TOML structured data.
/// Implements [`crate::processors::Processor`] trait.
pub struct TOMLProcessor;
impl Processor for TOMLProcessor {
    fn process_data(data: &str) -> crate::values::VizValues {
        toml::from_str(data).unwrap()
    }
}
