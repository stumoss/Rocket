#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::request::FromForm;

#[derive(PartialEq, Debug, FromForm)]
struct Form {  }

fn main() {
    // Same number of arguments: simple case.
    let task = Form::from_form_string("");
    assert_eq!(task, Ok(Form { }));
}
