//noinspection RsMainFunctionNotFound
#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
use rocket::{Rocket};

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
use rocket::figment::{Figment, providers::{Toml, Env, Format}};
use rocket::fairing::{ AdHoc };

#[derive(RustEmbed)]
#[folder = "../client/dist/pcb-decoupage"]
struct Asset;

#[derive(RustEmbed)]
#[folder = "./config"]
struct ConfigFolder;


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

#[derive(Responder)]
enum StrOrVecResponse {
    Str(String),
    Vec(Vec<u8>),
    StrAndContent(String, ContentType),
    VecAndContent(Vec<u8>, ContentType),
}

#[get("/<file..>")]
fn dist<'r>(file: PathBuf) -> Result<StrOrVecResponse, Status> {
  let filename = file.display().to_string();
  Asset::get(&filename).map_or_else(
    || Err(Status::NotFound),
    |d| {
      let ext_path = file.as_path();
      let ext_type = ext_path.extension().ok_or_else(|| Status::new(400))?;
      let ext =   OsStr::to_str(ext_type).ok_or_else(|| Status::new(400))?;
      let try_content_type = ContentType::from_extension(ext);

      match String::from_utf8(d.data.as_ref().to_vec()) {
        Ok(v) => {
            match try_content_type {
                Some(c) => Ok(StrOrVecResponse::StrAndContent(v, c)),
                None => Ok(StrOrVecResponse::Str(v))
            }
        },
        Err(_) => {
            match try_content_type {
                Some(c) => Ok(StrOrVecResponse::VecAndContent(d.data.as_ref().to_vec(), c)),
                None => Ok(StrOrVecResponse::Vec(d.data.as_ref().to_vec()))
            }
        },
    }
    },
  )
}

fn default_launch_browser() -> bool {
    true
}

//noinspection RsMainFunctionNotFound
#[rocket::launch]
fn rocket() -> Rocket<rocket::Build> {

  let config_str=ConfigFolder::get("PCBDecoupage.toml").map_or_else(
    || Err("missing toml"),
    |d| Ok(String::from_utf8(d.data.as_ref().to_vec()).unwrap()),
  ).unwrap();
  let figment = Figment::from(rocket::Config::default())
  .merge(Toml::string(&config_str).nested())
  .merge(Env::prefixed("PCB_DECOUPAGE_").global());


  let rocket = rocket::custom(figment);
  

  let figment = rocket.figment();

  #[derive(Deserialize)]
  struct Config {
      #[serde(default = "default_launch_browser")]
      launch_browser: bool,
  }

  let config: Config = figment.extract().expect("config");
  
  rocket
  .mount("/", routes![index, dist])
  .register("/", catchers![not_found])
  .attach(AdHoc::on_liftoff("Liftoff Message", |rkt| Box::pin(async move {
    if config.launch_browser {
      open::that(format!("http://{}:{}",rkt.config().address, rkt.config().port)).ok();
    }
  })))

}