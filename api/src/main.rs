use domain::datapoint;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Form<'a> {
    #[serde(rename = "fieldInput")]
    value: &'a str,
}

#[post("/input", format = "application/json", data = "<form_input>")]
fn input(form_input: Json<Form<'_>>) -> () {
    println!("{:?}", datapoint::create_datapoint(form_input.value));
}

#[post("/query", format = "application/json", data = "<form_input>")]
fn query(form_input: Json<Form<'_>>) -> () /*Json<Vec<&str>>*/ {
    println!("{:?}", datapoint::create_datapoint(form_input.value));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![input, query])
}
