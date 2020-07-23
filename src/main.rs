#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

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

fn main() {
  rocket::ignite()
    .mount("/", routes![index])
    .attach(Template::fairing())
    .launch();
}
