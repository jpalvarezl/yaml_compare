use crate::Result;

pub mod openai_yaml;

pub fn load_yaml_file(file_path: &str) -> Result<serde_yaml::Value> {
    let file = std::fs::File::open(file_path)?;
    let file_yaml: serde_yaml::Value = serde_yaml::from_reader(file)?;
    Ok(file_yaml)
}
