extern crate skim;

pub(crate) mod cli;
pub(crate) mod yaml;

use openapiv3::OpenAPI;

type Result<T> = anyhow::Result<T, Box<dyn std::error::Error>>;

pub fn main() {
    let openai = yaml::openai_yaml::deserialize_opena_api_yaml()
        .expect("Failed to deserialize yaml file");

    println!("{:#?}",openai.info.terms_of_service);
    // let yaml_file = yaml::load_yaml_file("input/openai.yaml").expect("Failed to open yaml file");
    // if let Some(yaml_file) = yaml_file.as_mapping() {
    //     let paths = yaml::openai_yaml::get_paths(yaml_file);
    //     println!("{:#?}", paths);
    //     // cli::input::select_one_from(paths);
    // }
    // println!("{:#?}", openai);
    // cli::input::select_one_from("aaaa\nbbbb\ncccc");
}
