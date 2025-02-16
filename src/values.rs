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
    /// Represents null field.
    Null,
    /// Represents boolean data.
    Bool(bool),
    /// Represents array.
    Array(Vec<VizValues>),
    /// Represents object.
    Object(IndexMap<String, VizValues>),
}

impl VizValues {
    pub fn from_yaml(value: yaml::Yaml) -> Self {
        match value {
            yaml::Yaml::Null => VizValues::Null,
            yaml::Yaml::Boolean(b) => VizValues::Bool(b),
            yaml::Yaml::Integer(i) => VizValues::Number(i),
            yaml::Yaml::Real(s) => {
                s.parse::<f64>()
                 .map(|f| VizValues::Number(f as i64))
                 .unwrap_or(VizValues::Null)
            },
            yaml::Yaml::String(s) => VizValues::String(s),
            yaml::Yaml::Array(seq) => {
                VizValues::Array(seq.into_iter().map(VizValues::from_yaml).collect())
            },
            yaml::Yaml::Hash(map) => {
                let mut object = IndexMap::new();
                for (k, v) in map {
                    let key = match k {
                        yaml::Yaml::String(s) => s,
                        other => other.as_str().unwrap_or("").to_string(),
                    };
                    object.insert(key, VizValues::from_yaml(v));
                }
                VizValues::Object(object)
            },
            _ => VizValues::Null,
        }
    }
}
