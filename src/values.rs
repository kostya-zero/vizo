use indexmap::IndexMap;
use serde::Deserialize;

/// A global values for Viz.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VizValues {
    /// Represents string.
    String(String),
    /// Represents integer.
    Number(i64),
    // Integer(i64),
    /// Represents null field.
    Null,
    // Float(f64),
    /// Represents boolean data.
    Bool(bool),
    // Boolean(bool),
    /// Represents array.
    Array(Vec<VizValues>),
    // Table(IndexMap<String, VizValues>),
    /// Represents object.
    Object(IndexMap<String, VizValues>),
}
