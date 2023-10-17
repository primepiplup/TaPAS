mod datapoint_dto;

use crate::datapoint_dto::{dto_vec_from, DatapointDTO};
use domain::datastore::Datastore;
use domain::plotter::basic_plot;
use rocket::fs::{relative, FileServer};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Form<'a> {
    #[serde(rename = "fieldInput")]
    value: &'a str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Image {
    filename: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Tag {
    tag: String,
}

#[post("/input", format = "application/json", data = "<form_input>")]
fn input(form_input: Json<Form<'_>>, datastorage: &State<Datastore>) -> () {
    datastorage.add_datapoint(form_input.value);
}

#[post("/query", format = "application/json", data = "<form_input>")]
fn query(form_input: Json<Form<'_>>, datastorage: &State<Datastore>) -> Json<Vec<DatapointDTO>> {
    let (datapoints, _parsed) = datastorage.query(form_input.value);
    Json(dto_vec_from(datapoints))
}

#[post("/plot", format = "application/json", data = "<form_input>")]
fn plot(form_input: Json<Form<'_>>, datastorage: &State<Datastore>) -> Json<Image> {
    let (datapoints, parsed) = datastorage.query(form_input.value);
    let filename = basic_plot(&datapoints, parsed).expect("Plot broke...");
    Json(Image { filename })
}

#[get("/tags")]
fn tags(datastorage: &State<Datastore>) -> Json<Vec<Tag>> {
    let tags = datastorage.retrieve_taglist();
    let mut tag_objects: Vec<Tag> = Vec::new();
    for tag in tags {
        tag_objects.push(Tag { tag });
    }
    Json(tag_objects)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![input, query, plot, tags])
        .mount("/plot", FileServer::from(relative!("../generated")))
        .manage(Datastore::new())
}
