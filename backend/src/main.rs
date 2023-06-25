use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;

//Import the data structure we just made
use common::Markup;
//Used to read files
use std::fs;
//Json for sending to the front end
use rocket::serde::json::Json;

//Import the rocket macros
#[macro_use]
extern crate rocket;

// Return the index file as a Rocket NamedFile
async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../ui/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

//Create a route for any url that is a path from the /
#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../ui/dist").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

#[get("/data/<path..>")]
async fn data(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./data/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}
#[get("/markup/<path..>")]
fn get_markup(path: PathBuf) -> Json<Markup> {
    let path = PathBuf::from("data/markup/").join(path);
    match fs::read_to_string(path) {
        Ok(markup) => Json(Markup {
            markup: String::from(markup),
        }),
        Err(e) => Json(Markup {
            markup: String::from(format!("Error: {}", e)),
        }),
    }
}
// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[get("/resume")]
async fn resume() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../ui/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, static_files, data, resume])
        .mount("/api", routes![get_markup])
}