extern crate skim;

pub(crate) mod cli;
pub(crate) mod yaml;

use std::{path, sync::Arc};
use skim::prelude::*;
use openapiv3::OpenAPI;

type Result<T> = anyhow::Result<T, Box<dyn std::error::Error>>;

pub fn main() {
    let openai = yaml::openai_yaml::deserialize_opena_api_yaml()
        .expect("Failed to deserialize yaml file");

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    let path_items = cli::skim_items::get_path_skim_items(openai);

    for item in path_items {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item);

    cli::input::select_one_from(rx_item);
}
