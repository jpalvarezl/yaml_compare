use files::TargetFiles;

pub(crate) mod files;

pub fn load_files()-> Result<TargetFiles, Box<dyn std::error::Error>> {
    let left = load_yaml_file("input/left.yaml")?;
    let right = load_yaml_file("input/right.yaml")?;

    Ok(TargetFiles::new(left, right))
}


fn load_yaml_file(file_path: &str) -> Result<serde_yaml::Value, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let file_yaml: serde_yaml::Value = serde_yaml::from_reader(file)?;
    Ok(file_yaml)
}
