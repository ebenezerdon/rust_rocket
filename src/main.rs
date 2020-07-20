#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/")]
fn index() -> Template {
  #[derive(Serialize, Debug)]
  struct Context {
    first_name: String,
    last_name: String
  }

  let context = Context {
    first_name: String::from("Ebenezer"),
    last_name: String::from("Don")
  };

  Template::render("index", context)
}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
