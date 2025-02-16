use super::Processor;

/// A processor that processes YAML structured data.
/// Implements [`crate::processors::Processor`] trait.
pub struct YAMLProcessor;
impl Processor for YAMLProcessor {
    fn process_data(data: &str) -> crate::values::VizValues {
        let docs = yaml::YamlLoader::load_from_str(data).unwrap();
        let yaml_doc = docs.into_iter().next().unwrap();
        crate::values::VizValues::from_yaml(yaml_doc)
    }
}