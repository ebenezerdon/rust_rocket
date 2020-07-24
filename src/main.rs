#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;
use rocket::Request;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/")]
fn index() -> Template {
  #[derive(Serialize)]
  struct Context {
    first_name: String,
    last_name: String
  }

  let context = Context {
    first_name: String::from("Ebenezer"),
    last_name: String::from("Don")
  };

  Template::render("home", context)
}

#[get("/api")]
fn api() -> content::Json<&'static str> {
  content::Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
  rocket::ignite()
    .register(catchers![not_found])
    .mount("/", routes![index, api])
    .attach(Template::fairing())
    .launch();
}
