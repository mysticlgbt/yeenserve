#[macro_use]
extern crate rocket;

use std::env;
use std::fs;
use std::path::Path;

use rand::Rng;
use rocket::State;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;

// Contains config for the application.
struct YeenserveConfig {
    path: String,
}

// List of approved extensions.
static EXTENSIONS: &'static [&str] = &["jpg", "jpeg", "png"];

#[get("/")]
async fn root(config: &State<YeenserveConfig>) -> Result<NamedFile, NotFound<String>> {
    // Read all file entries from the pictures path.
    let all_entries = fs::read_dir(config.path.as_str());
    if all_entries.is_err() {
        return Result::Err(NotFound(all_entries.err().unwrap().to_string()))
    }
    let all_entries = all_entries.unwrap();

    // Filter to contain only files with extensions contained in EXTENSIONS.
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

    // Collect all of the files.
    let collected_entries: Result<Vec<fs::DirEntry>, _> = filtered_entries.collect();
    let entries = collected_entries.unwrap();

    if entries.len() == 0 {
        return Result::Err(NotFound(String::from("Pictures directory empty.")))
    }

    // Generate a random number, and index the list of files we've collected.
    let random_num: u32 = { rand::thread_rng().gen::<u32>() };
    let path: &fs::DirEntry = &entries[random_num as usize % entries.len()];

    // Return the selected file to the web server.
    let file = NamedFile::open(path.path().to_str().unwrap()).await.ok();
    return if file.is_some() {
        Result::Ok(file.unwrap())
    } else {
        Result::Err(NotFound(String::from("File not found.")))
    }
}

fn build_config() -> YeenserveConfig {
    let mut path = String::from("resources/");
    let path_env = env::var("YEENSERVE_PATH");
    if path_env.is_ok() {
        path = path_env.unwrap()
    }

    // Validate that the pictures path exists.
    if !Path::new(path.as_str()).is_dir() {
        panic!("Path {} is not a directory!", path.as_str());
    }

    return YeenserveConfig {
        path
    }
}

#[rocket::main]
async fn main() {
    rocket::build().manage({
        build_config()
    }).mount("/", routes![root]).launch().await;
}
