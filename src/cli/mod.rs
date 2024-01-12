use serde_yaml::Mapping;
use skim::SkimItem;

pub mod input;

struct OpenAPIItem {
    search_key: String,
    inner: Mapping,
}

impl SkimItem for OpenAPIItem {
    fn text(&self) -> std::borrow::Cow<str> {
        self.search_key.as_str().into()
    }

    fn output(&self) -> std::borrow::Cow<str> {
        self.search_key.as_str().into()
    }
}
