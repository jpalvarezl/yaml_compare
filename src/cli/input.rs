use skim::prelude::*;
use std::io::Cursor;

use crate::Result;

pub fn select_one_from(rx_item: Receiver<Arc<dyn SkimItem>>) {

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .expect("Build CLI layout failed");
    
    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
}
