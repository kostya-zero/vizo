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
    /// Represents float.
    Float(f64),
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

    pub fn from_serde_json(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => VizValues::Null,
            serde_json::Value::Bool(b) => VizValues::Bool(b),
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    VizValues::Number(n.as_i64().unwrap())
                } else {
                    VizValues::Float(n.as_f64().unwrap())
                }
            },
            serde_json::Value::String(s) => VizValues::String(s),
            serde_json::Value::Array(vec) => {
                VizValues::Array(vec.into_iter().map(VizValues::from_serde_json).collect())
            },
            serde_json::Value::Object(map) => {
                let mut object = IndexMap::new();
                for (k, v) in map {
                    object.insert(k, VizValues::from_serde_json(v));
                }
                VizValues::Object(object)
            },
        }
    }
}
