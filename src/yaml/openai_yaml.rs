pub fn get_paths(yaml: &serde_yaml::Mapping) -> Vec<String> {
    let mut paths = Vec::new();
    for (key, value) in yaml {
        let key = key.as_str().unwrap();
        let value = value.as_str().unwrap();
        paths.push(format!("{}: {}", key, value));
    }
    paths
}
