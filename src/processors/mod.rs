use crate::values::VizValues;

/// A processor for JSON.
pub mod json;

/// A processor for TOML.
pub mod toml;

/// A base trait for processors.
pub trait Processor {
    fn process_data(data: &str) -> VizValues;
}