#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;

#[get("/")]
fn index() -> &'static str {
    return "yeen"
}

#[get("/yeen")]
async fn pic() -> Option<NamedFile> {
    NamedFile::open("resources/yeen.jpg").await.ok()
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index, pic]).launch().await;
}
