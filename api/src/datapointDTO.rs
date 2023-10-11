use domain::datapoint::Datapoint;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DatapointDTO {
    data: String,
    tags: Vec<String>,
}

pub fn from(datapoint: Datapoint) -> DatapointDTO {
    DatapointDTO {
        data: datapoint.get_data().to_owned(),
        tags: datapoint.get_tags().to_owned(),
    }
}

pub fn vec_from(datapoints: Vec<Datapoint>) -> Vec<DatapointDTO> {
    let mut collector = Vec::new();
    for datapoint in datapoints {
        collector.push(from(datapoint));
    }
    collector
}
