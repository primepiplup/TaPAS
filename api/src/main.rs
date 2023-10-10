use domain;
use rocket::serde::{json::Json, Deserialize};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Input<'a> {
    #[serde(rename = "fieldInput")]
    field_text: &'a str,
}

#[post("/input", format = "application/json", data = "<form_input>")]
fn input(form_input: Json<Input<'_>>) -> () {
    println!("{:?}", domain::create_datapoint(form_input.field_text));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![input])
}
