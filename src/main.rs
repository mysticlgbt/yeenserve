#[macro_use]
extern crate rocket;

use std::env;
use std::path::Path;

use rand::Rng;
use rocket::{Request, response, Response, State};
use rocket::response::{content, Responder};
use rocket::response::status::NotFound;

mod backend;

// Contains config for the application.
struct YeenserveConfig {
    backend: Box<dyn backend::base::Backend>,
}

static DEFAULT_PATH: &'static str = "resources/";

struct Image {
    data: Vec<u8>,
    content_type: &'static str,
}

impl<'r> Responder<'r, 'static> for Image {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.data.respond_to(req)?)
            .raw_header("Content-Type", self.content_type)
            .ok()
    }
}

#[get("/")]
async fn root(config: &State<YeenserveConfig>) -> Result<content::RawHtml<String>, NotFound<String>> {
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
    let name: &String = &pictures[random_num as usize % pictures_len];

    let html = String::from(format!("<img style=\"display: block; user-select: none; margin: auto;
        background-color: rgb(230, 230, 230); width: 100%\"
    src=\"/pics/{}\" />", name));

    return Ok(content::RawHtml(html));
}

#[get("/pics/<path>")]
async fn pics(path: &str, config: &State<YeenserveConfig>) -> Result<Image, NotFound<String>> {
    let data = config.backend.get_file_contents(path);
    return if data.is_ok() {
        let path = Path::new(path);
        let ext = path.extension().unwrap().to_str().unwrap();
        let ext = match ext {
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "png" => "image/png",
            _ => return Err(NotFound("Extension missing.".to_string()))
        };

        Ok(Image { data: data.unwrap(), content_type: ext })
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

    let backend_type = std::env::var("YEENSERVE_BACKEND");
    let backend = match backend_type.unwrap_or("file".to_string()).as_str() {
        "file" => crate::backend::file::create(path),
        "s3" => crate::backend::s3::create(),
        _ => panic!("invalid backend type"),
    };
    if backend.is_err() {
        panic!("failed to initialize backend");
    }

    return YeenserveConfig {
        backend: backend.unwrap()
    };
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().manage({
        build_config()
    }).mount("/", routes![
        root,
        pics
    ]).launch().await.expect("Rocket launch");
}
