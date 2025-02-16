use super::Processor;

/// A processor that processes JSON structured data.
/// Implements [`crate::processors::Processor`] trait.
pub struct JSONProcessor;
impl Processor for JSONProcessor {
    fn process_data(data: &str) -> crate::values::VizValues {
        serde_json::from_str(data).unwrap()
    }
}
