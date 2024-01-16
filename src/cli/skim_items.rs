use openapiv3::{PathItem, OpenAPI};
use skim::SkimItem;

use crate::Result;

#[derive(Debug)]
pub struct YamlPath {
    pub search_key: String,
    path: PathItem,
}

impl SkimItem for YamlPath {
    fn text(&self) -> std::borrow::Cow<str> {
        self.search_key.as_str().into()
    }

    fn output(&self) -> std::borrow::Cow<str> {
        self.search_key.as_str().into()
    }
}

pub fn get_path_skim_items(openai: OpenAPI) -> Vec<YamlPath> {
    let paths = openai.paths;
    let mut path_items: Vec<YamlPath> = Vec::new();
    for (key, value) in paths {
        path_items.push(YamlPath {
            search_key: key,
            path: value.as_item().expect("Failed to get path item").clone(),
        });
    }
    path_items
}
