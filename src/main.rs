#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::{Request, response::content};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(FromForm)]
struct Book {
  title: String,
  author: String,
  isbn: String
}

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

#[get("/hello")]
fn hello() -> content::Json<&'static str> {
  content::Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    print!("{}", req);
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

// #[post("/book", data = "<book_form>")]
// fn new_book(book_form: Form<Book>) -> content::Json<&'static str> {
//   let book: Book = book_form.into_inner();

//   print!("{}", book.title);
//   content::Json("{
//     'stuff' 'yolo'
//   }")
// }

#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> Flash<Redirect> {
  let book: Book = book_form.into_inner();

  if book.title.is_empty() {
    Flash::error(Redirect::to("/book"), "Title is required!")
  } else {
    Flash::error(Redirect::to("/book"), "Server error.")
  }
}

fn main() {
  rocket::ignite()
    .register(catchers![not_found])
    .mount("/", routes![index])
    .mount("/api", routes![hello, new_book])
    .attach(Template::fairing())
    .launch();
}
