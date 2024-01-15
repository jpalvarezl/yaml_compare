use openapiv3::OpenAPI;

use crate::Result;

pub fn get_paths(yaml: &serde_yaml::Mapping) -> Vec<String> {
    println!("{:#?}", yaml.capacity() );
    let mut paths = Vec::new();
    for (key, value) in yaml {
        println!("{:#?}: {:#?}", key, value.as_mapping().is_some());
        let key = key.as_str().unwrap();
        if value.as_str().is_none() {
            continue;
        }
        let value = value.as_str().unwrap();
        paths.push(format!("{}: {}", key, value));
    }
    paths
}


pub fn deserialize_opena_api_yaml() -> Result<OpenAPI> {
    let data = include_str!("../../input/openai.yaml");
    let openai: OpenAPI = serde_yaml::from_str::<OpenAPI>(data)?;
    Ok(openai)
}
