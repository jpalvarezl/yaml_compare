pub fn load_files()-> Result<(), Box<dyn std::error::Error>> {
    let left = load_yaml_file("input/left.yaml")?;
    let right = load_yaml_file("input/right.yaml")?;

    Ok(())
}


fn load_yaml_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open(file_path)?;
    let d: String = serde_yaml::from_reader(f)?;
    println!("Read YAML string: {}", d);
    Ok(())
}
