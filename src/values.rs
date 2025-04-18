use indexmap::IndexMap;
use serde::Deserialize;

/// A global values for Viz.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VizValue {
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
    Array(Vec<VizValue>),
    /// Represents object.
    Object(IndexMap<String, VizValue>),
}

impl VizValue {
    pub fn from_yaml(value: yaml::Yaml) -> Self {
        match value {
            yaml::Yaml::Null => VizValue::Null,
            yaml::Yaml::Boolean(b) => VizValue::Bool(b),
            yaml::Yaml::Integer(i) => VizValue::Number(i),
            yaml::Yaml::Real(s) => {
                s.parse::<f64>()
                 .map(|f| VizValue::Number(f as i64))
                 .unwrap_or(VizValue::Null)
            },
            yaml::Yaml::String(s) => VizValue::String(s),
            yaml::Yaml::Array(seq) => {
                VizValue::Array(seq.into_iter().map(VizValue::from_yaml).collect())
            },
            yaml::Yaml::Hash(map) => {
                let mut object = IndexMap::new();
                for (k, v) in map {
                    let key = match k {
                        yaml::Yaml::String(s) => s,
                        other => other.as_str().unwrap_or("").to_string(),
                    };
                    object.insert(key, VizValue::from_yaml(v));
                }
                VizValue::Object(object)
            },
            _ => VizValue::Null,
        }
    }

    pub fn from_serde_json(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => VizValue::Null,
            serde_json::Value::Bool(b) => VizValue::Bool(b),
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    VizValue::Number(n.as_i64().unwrap())
                } else {
                    VizValue::Float(n.as_f64().unwrap())
                }
            },
            serde_json::Value::String(s) => VizValue::String(s),
            serde_json::Value::Array(vec) => {
                VizValue::Array(vec.into_iter().map(VizValue::from_serde_json).collect())
            },
            serde_json::Value::Object(map) => {
                let mut object = IndexMap::new();
                for (k, v) in map {
                    object.insert(k, VizValue::from_serde_json(v));
                }
                VizValue::Object(object)
            },
        }
    }
}
