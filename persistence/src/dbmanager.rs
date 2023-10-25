use crate::datapoint_dso::DatapointDSO;
use domain::datapoint::Datapoint;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;

pub struct DBManager {
    pool: MySqlPool,
}

impl DBManager {
    pub async fn new() -> DBManager {
        let url = match env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => panic!("DATABASE_URL environment variable not configured."),
        };
        let pool = MySqlPoolOptions::new().connect(&url).await.unwrap();
        DBManager { pool }
    }

    pub async fn insert_datapoint(&self, datapoint: Datapoint) -> bool {
        let dso: DatapointDSO = datapoint.into();
        match sqlx::query(
            "INSERT INTO datapoints(data, tags, datetime, data_key) VALUES (?, ?, ?, ?)",
        )
        .bind(dso.get_data())
        .bind(dso.get_stringified_tags())
        .bind(dso.get_datetime())
        .bind(dso.get_key())
        .execute(&self.pool)
        .await
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
