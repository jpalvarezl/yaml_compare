use openapiv3::OpenAPI;

use crate::Result;

pub fn deserialize_opena_api_yaml() -> Result<OpenAPI> {
    let data = include_str!("../../input/openai.yaml");
    let openai: OpenAPI = serde_yaml::from_str::<OpenAPI>(data)?;
    Ok(openai)
}
