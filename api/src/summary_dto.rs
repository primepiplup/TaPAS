use domain::stats::summary::Summary;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SummaryDTO {
    name: String,
    mean: f64,
}

impl From<Summary> for SummaryDTO {
    fn from(summary: Summary) -> SummaryDTO {
        SummaryDTO {
            name: summary.get_name(),
            mean: summary.get_mean(),
        }
    }
}
