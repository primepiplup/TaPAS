mod datapoint_dto;

use crate::datapoint_dto::{dto_vec_from, DatapointDTO};
use chrono::{Local, NaiveDateTime};
use domain::analysis::linear_regression;
use domain::datastore::Datastore;
use domain::plotter::scatterplot;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket_db_pools::{sqlx, Database};

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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PredictionForm<'a> {
    #[serde(rename = "fieldInput")]
    query: &'a str,
    #[serde(rename = "targetGoal")]
    goal: f64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Image {
    filename: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Prediction {
    prediction: String,
    #[serde(rename = "willIntercept")]
    will_intercept: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Tag {
    tag: String,
}

#[derive(Database)]
#[database("mysql_storage")]
struct Storage(sqlx::MySqlPool);

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

#[post("/predict", format = "application/json", data = "<form_input>")]
fn predict(
    form_input: Json<PredictionForm<'_>>,
    datastorage: &State<Datastore>,
) -> status::Custom<Json<Prediction>> {
    let (datapoints, _) = datastorage.query(form_input.query);
    let data = datapoints
        .into_iter()
        .map(|datapoint| {
            (
                datapoint.get_datetime().to_owned(),
                datapoint.get_as_numeric().unwrap(),
            )
        })
        .collect();
    let linear_function = linear_regression(data, 50);
    let prediction: i64 = linear_function.apply_inverse(form_input.goal) as i64;
    let predicted_datetime = NaiveDateTime::from_timestamp_opt(prediction, 0).unwrap();

    let mut will_intercept = true;

    if predicted_datetime < Local::now().naive_local() {
        will_intercept = false;
    }

    status::Custom(
        Status::Ok,
        Json(Prediction {
            prediction: predicted_datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            will_intercept, // Need to create a way to check whether an intercept will even occur beyond "now"
        }),
    )
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
        .mount("/api", routes![input, query, plot, tags, predict])
        .mount("/plot", FileServer::from(relative!("../generated")))
        .manage(Datastore::new())
        .attach(Storage::init())
}
