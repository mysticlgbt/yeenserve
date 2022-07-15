#[macro_use]
extern crate rocket;

use std::cell::Cell;
use std::env;
use std::fs;

use rand::Rng;
use rocket::{Config, State};
use rocket::fs::NamedFile;

struct YeenserveConfig {
    path: String,
}

static EXTENSIONS: &'static [&str] = &["jpg", "jpeg", "png"];

#[get("/")]
async fn pic(config: &State<YeenserveConfig>) -> Option<NamedFile> {
    let all_entries = fs::read_dir(config.path.as_str()).unwrap();
    let filtered_entries = all_entries.filter(|p| {
        let entry = p.as_ref().unwrap();
        let path = entry.path();
        let ext = path.extension();
        if ext.is_none() {
            return false;
        }
        let ext_str = ext.unwrap().to_str().unwrap();
        let is_valid_ext = EXTENSIONS.contains(&ext_str);
        p.is_ok() && entry.file_type().unwrap().is_file() && is_valid_ext
    });
    let collected_entries: Result<Vec<fs::DirEntry>, _> = filtered_entries.collect();
    let entries = collected_entries.unwrap();

    let f: u32 = { rand::thread_rng().gen::<u32>() };
    let path: &fs::DirEntry = &entries[f as usize % entries.len()];

    NamedFile::open(path.path().to_str().unwrap()).await.ok()
}

fn build_config() -> YeenserveConfig {
    let mut path = String::from("resources/");
    let path_env = std::env::var("YEENSERVE_PATH");
    if path_env.is_ok() {
        path = path_env.unwrap()
    }

    return YeenserveConfig {
        path
    }
}

#[rocket::main]
async fn main() {
    rocket::build().manage({
        build_config()
    }).mount("/", routes![pic]).launch().await;
}
