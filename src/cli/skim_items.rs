use openapiv3::{OpenAPI, PathItem};
use skim::{ItemPreview, PreviewContext, SkimItem};

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

    // should probably do something with the output here. Copy to clip board?
    // fn output(&self) -> std::borrow::Cow<str> {
    //     self.search_key.as_str().into()
    // }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        let preview = serde_yaml::to_string(&self.path).expect("Failed to serialize path item");
        return ItemPreview::Text(preview);
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
