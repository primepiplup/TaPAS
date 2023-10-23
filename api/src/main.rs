mod datapoint_dto;

use crate::datapoint_dto::{dto_vec_from, DatapointDTO};
use domain::datastore::Datastore;
use domain::plotter::scatterplot;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::response::status;
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PlotRequest<'a> {
    #[serde(rename = "fieldInput")]
    value: &'a str,
    #[serde(rename = "withRegression")]
    with_regression: bool,
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
fn plot(
    form_input: Json<PlotRequest<'_>>,
    datastorage: &State<Datastore>,
) -> status::Custom<Json<Image>> {
    let (datapoints, parsed) = datastorage.query(form_input.value);
    match scatterplot(&datapoints, parsed, form_input.with_regression) {
        Ok(filename) => status::Custom(Status::Ok, Json(Image { filename })),
        Err(err) => status::Custom(
            Status::InternalServerError,
            Json(Image {
                filename: "nodice".to_string(),
            }),
        ),
    }
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
