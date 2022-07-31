#[macro_use]
extern crate rocket;

use std::env;
use std::path::Path;

use rand::Rng;
use rocket::response::status::NotFound;
use rocket::State;

mod backend;

// Contains config for the application.
struct YeenserveConfig {
    backend: Box<dyn backend::base::Backend>
}

static DEFAULT_PATH: &'static str = "resources/";

#[get("/")]
async fn root(config: &State<YeenserveConfig>) -> Result<String, NotFound<String>> {
    // Load list of pictures.
    let pictures = config.backend.list_files();
    if pictures.is_err() {
        return Err(NotFound(String::from(pictures.err().unwrap().to_string())));
    }
    let pictures = pictures.unwrap();
    let pictures_len = pictures.len();

    // If there are no pictures, return a 404.
    if pictures_len == 0 {
        return Err(NotFound("Pictures directory empty.".to_string()));
    }

    // Generate a random number, and index the list of files we've collected.
    let random_num: u32 = { rand::thread_rng().gen::<u32>() };
    let path: &String = &pictures[random_num as usize % pictures_len];

    // Return the selected file to the web server.
    //let file = NamedFile::open(path.path().to_str().unwrap()).await.ok();
    return if true {
        Ok(path.clone())
    } else {
        Err(NotFound("File not found.".to_string()))
    };
}

fn build_config() -> YeenserveConfig {
    let mut path = String::from(DEFAULT_PATH);
    let path_env = env::var("YEENSERVE_PATH");
    if path_env.is_ok() {
        path = path_env.unwrap()
    }

    // Validate that the pictures path exists.
    if !Path::new(path.as_str()).is_dir() {
        panic!("Path {} is not a directory!", path.as_str());
    }

    let be = crate::backend::file::create(path);

    return YeenserveConfig {
        backend: be
    };
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().manage({
        build_config()
    }).mount("/", routes![root]).launch().await.expect("Rocket launch");
}
