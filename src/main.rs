extern crate skim;

pub(crate) mod cli;
pub(crate) mod yaml;

pub fn main() {
    let files = yaml::load_files().expect("Failed to open yaml files");
    cli::input::select_one_from("aaaa\nbbbb\ncccc");
}
