mod datapoint_dto;
mod summary_dto;

use crate::datapoint_dto::{dto_vec_from, DatapointDTO};
use crate::summary_dto::SummaryDTO;
use chrono::NaiveDateTime;
use domain::datastore::Datastore;
use domain::plotter::categorical::categorical_plot;
use domain::plotter::scatterplot::{predictionplot, scatterplot};
use domain::stats::model_fit::linear_regression;
use domain::stats::stats::compare;
use persistence::dbmanager::DBManager;
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

#[post("/input", format = "application/json", data = "<form_input>")]
async fn input(
    form_input: Json<Form<'_>>,
    datastorage: &State<Datastore>,
    dbmanager: &State<DBManager>,
) -> Status {
    let new_datapoint = datastorage.add_datapoint(form_input.value);
    match dbmanager.insert_datapoint(new_datapoint).await {
        true => Status::Ok,
        false => Status::InternalServerError,
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct UpdateForm<'a> {
    #[serde(rename = "fieldInput")]
    value: &'a str,
    key: u64,
}

#[post("/update", format = "application/json", data = "<form_input>")]
async fn update(
    form_input: Json<UpdateForm<'_>>,
    datastorage: &State<Datastore>,
    dbmanager: &State<DBManager>,
) -> status::Custom<Json<DatapointDTO>> {
    let updated_datapoint = datastorage.update_datapoint(form_input.value, form_input.key);
    dbmanager.update_datapoint(updated_datapoint.clone()).await;
    status::Custom(Status::Ok, Json(DatapointDTO::from(updated_datapoint)))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EditRequest<'a> {
    tags: &'a str,
    add: bool,
    keys: Vec<u64>,
}

#[post("/batchedit", format = "application/json", data = "<edit_request>")]
async fn batchedit(
    edit_request: Json<EditRequest<'_>>,
    datastorage: &State<Datastore>,
    dbmanager: &State<DBManager>,
) -> status::Custom<Json<Vec<DatapointDTO>>> {
    let datapoints = datastorage.batch_operation(
        edit_request.tags,
        edit_request.keys.clone(),
        edit_request.add,
    );
    for datapoint in datapoints.clone() {
        if !dbmanager.update_datapoint(datapoint).await {
            return status::Custom(Status::InternalServerError, Json(dto_vec_from(datapoints)));
        }
    }

    return status::Custom(Status::Ok, Json(dto_vec_from(datapoints)));
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct DeleteKey {
    value: u64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct DeleteConfirmation {
    #[serde(rename = "datastoreDeleted")]
    datastore_deleted: bool,
    #[serde(rename = "databaseDeleted")]
    database_deleted: bool,
}

#[post("/delete", format = "application/json", data = "<key>")]
async fn delete(
    key: Json<DeleteKey>,
    datastorage: &State<Datastore>,
    dbmanager: &State<DBManager>,
) -> status::Custom<Json<DeleteConfirmation>> {
    let datastore_deleted = match datastorage.delete_datapoint(key.value) {
        Some(_) => true,
        None => false,
    };
    let database_deleted = dbmanager.delete_datapoint(key.value).await;
    if database_deleted && database_deleted {
        return status::Custom(
            Status::Ok,
            Json(DeleteConfirmation {
                datastore_deleted,
                database_deleted,
            }),
        );
    } else {
        return status::Custom(
            Status::InternalServerError,
            Json(DeleteConfirmation {
                datastore_deleted,
                database_deleted,
            }),
        );
    }
}

#[post("/query", format = "application/json", data = "<form_input>")]
fn query(form_input: Json<Form<'_>>, datastorage: &State<Datastore>) -> Json<Vec<DatapointDTO>> {
    let queryresult = datastorage.query(form_input.value);
    Json(dto_vec_from(queryresult.get_datapoints()))
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

#[post("/plot", format = "application/json", data = "<form_input>")]
fn plot(
    form_input: Json<PlotRequest<'_>>,
    datastorage: &State<Datastore>,
) -> status::Custom<Json<Image>> {
    let queryresult = datastorage.query(form_input.value);
    match scatterplot(&queryresult, form_input.with_regression) {
        Ok(filename) => status::Custom(Status::Ok, Json(Image { filename })),
        Err(_) => status::Custom(
            Status::InternalServerError,
            Json(Image {
                filename: "nodice".to_string(),
            }),
        ),
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct CompareForm<'a> {
    #[serde(borrow)]
    #[serde(rename = "fieldInputs")]
    queries: Vec<&'a str>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ComparisonResults {
    filename: String,
    summaries: Vec<SummaryDTO>,
}

#[post("/comparison", format = "application/json", data = "<form_input>")]
fn comparison(
    form_input: Json<CompareForm<'_>>,
    datastorage: &State<Datastore>,
) -> status::Custom<Json<ComparisonResults>> {
    let mut collector = Vec::new();
    for query in form_input.queries.clone() {
        collector.push(datastorage.query(query));
    }
    let filename = match categorical_plot(&collector) {
        Some(filename) => filename,
        None => {
            return status::Custom(
                Status::Ok,
                Json(ComparisonResults {
                    filename: "none".to_string(),
                    summaries: Vec::new(),
                }),
            )
        }
    };
    let summaries = compare(&collector)
        .into_iter()
        .map(|summary| SummaryDTO::from(summary))
        .collect();
    status::Custom(
        Status::Ok,
        Json(ComparisonResults {
            filename,
            summaries,
        }),
    )
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Prediction {
    filename: String,
    prediction: String,
    #[serde(rename = "willIntercept")]
    will_intercept: bool,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PredictionForm<'a> {
    #[serde(rename = "fieldInput")]
    query: &'a str,
    #[serde(rename = "targetGoal")]
    goal: f64,
}

#[post("/predict", format = "application/json", data = "<form_input>")]
fn predict(
    form_input: Json<PredictionForm<'_>>,
    datastorage: &State<Datastore>,
) -> status::Custom<Json<Prediction>> {
    let queryresult = datastorage.query(form_input.query);

    let data = match queryresult.get_date_numeric_data() {
        Some(data) => data,
        None => {
            return status::Custom(
                Status::InternalServerError,
                Json(Prediction {
                    filename: "".to_string(),
                    prediction: "float parsing failure".to_string(),
                    will_intercept: false,
                }),
            )
        }
    };
    let latest_date = data[data.len() - 1].0;
    let linear_function = linear_regression(data, 50);
    let prediction: i64 = linear_function.apply_inverse(form_input.goal) as i64;
    let predicted_datetime = NaiveDateTime::from_timestamp_opt(prediction, 0).unwrap();

    let mut will_intercept = true;

    if predicted_datetime < latest_date.naive_local() {
        will_intercept = false;
    }

    let filename: String;
    if will_intercept {
        filename = predictionplot(
            &queryresult,
            linear_function,
            form_input.goal,
            predicted_datetime,
        );
    } else {
        filename = "".to_string();
    }

    status::Custom(
        Status::Ok,
        Json(Prediction {
            filename,
            prediction: predicted_datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            will_intercept,
        }),
    )
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Tag {
    tag: String,
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
async fn rocket() -> _ {
    let dbmanager = DBManager::new().await;
    let datapoints = dbmanager.load_datapoints().await;
    let datastore = Datastore::from(datapoints);
    rocket::build()
        .mount(
            "/api",
            routes![input, query, plot, tags, predict, update, delete, batchedit, comparison],
        )
        .mount("/plot", FileServer::from(relative!("../generated")))
        .manage(datastore)
        .manage(dbmanager)
}
