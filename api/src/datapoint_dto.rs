use domain::datapoint::Datapoint;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DatapointDTO {
    timestamp: String,
    data: String,
    tags: Vec<String>,
    key: u64,
}

impl From<Datapoint> for DatapointDTO {
    fn from(datapoint: Datapoint) -> DatapointDTO {
        let datapoint_timestamp = datapoint.get_datetime().to_rfc2822();
        DatapointDTO {
            timestamp: datapoint_timestamp,
            data: datapoint.get_data().to_owned(),
            tags: datapoint.get_tags().to_owned(),
            key: datapoint.get_key(),
        }
    }
}

pub fn dto_vec_from(datapoints: Vec<Datapoint>) -> Vec<DatapointDTO> {
    let mut collector = Vec::new();
    for datapoint in datapoints {
        collector.push(DatapointDTO::from(datapoint));
    }
    collector
}
