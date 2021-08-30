#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};
// use rocket::response;
use rust_embed::RustEmbed;

use std::ffi::OsStr;
// use std::io::Cursor;
use std::path::PathBuf;
use std::string::String;
use rocket::response::content;
// use rocket::response::content::Html;
use rocket::Request;
use rocket::serde::Deserialize;

#[derive(RustEmbed)]
#[folder = "../client/dist/pcb-decoupage"]
struct Asset;


#[catch(404)]
fn not_found(request: &Request<'_>) -> content::Html<String> {
    let html = match request.format() {
        Some(ref mt) if !(mt.is_xml() || mt.is_html()) => {
            format!("<p>'{}' requests are not supported.</p>", mt)
        }
        _ => format!("<p>Sorry, '{}' is an invalid path!",
                 request.uri())
    };

    content::Html(html)
}

#[get("/")]
fn index<'r>() -> Result<content::Html<String>,Status> {
  Asset::get("index.html").map_or_else(
    || Err(Status::NotFound),
    |d| Ok(content::Html(String::from_utf8(d.data.as_ref().to_vec()).unwrap())),
  )
}

#[get("/<file..>")]
fn dist<'r>(file: PathBuf) -> Result<(ContentType, String), Status> {
  let filename = file.display().to_string();
  Asset::get(&filename).map_or_else(
    || Err(Status::NotFound),
    |d| {
      let ext = file
        .as_path()
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| Status::new(400))?;
      let content_type = ContentType::from_extension(ext).ok_or_else(|| Status::new(400))?;
      Ok((content_type, String::from_utf8(d.data.as_ref().to_vec()).unwrap()))
    },
  )
}

#[rocket::launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    

    let figment = rocket.figment();

    #[derive(Deserialize)]
    struct Config {
        launch_browser: bool,
        address: String,
        port: u16,
    }

    let config: Config = figment.extract().expect("config");
    
    if config.launch_browser {
        open::that(format!("http://{}:{}",config.address, config.port)).ok();
    }
    
    rocket
    .mount("/", routes![index, dist])
    .register("/", catchers![not_found])

}