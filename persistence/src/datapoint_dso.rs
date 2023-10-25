use chrono::prelude::*;
use domain::datapoint::Datapoint;

struct DatapointDSO {
    datetime: i64,
    data: String,
    tags: Vec<String>,
    key: u64,
}

impl DatapointDSO {
    pub fn vec_into<U, T: Into<U>>(vec: Vec<T>) -> Vec<U> {
        vec.into_iter().map(|elem| elem.into()).collect()
    }
}

impl From<Datapoint> for DatapointDSO {
    fn from(datapoint: Datapoint) -> Self {
        DatapointDSO {
            datetime: datapoint.get_datetime().timestamp(),
            data: datapoint.get_data().to_owned(),
            tags: datapoint.get_tags().to_owned(),
            key: datapoint.get_key(),
        }
    }
}

impl Into<Datapoint> for DatapointDSO {
    fn into(self) -> Datapoint {
        Datapoint::new(
            Local.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(self.datetime, 0).unwrap()),
            self.data,
            self.tags,
            self.key,
        )
    }
}

#[cfg(test)]
pub mod tests {
    use domain::datapoint::create_datapoint;

    use super::*;

    #[test]
    fn datapoint_vec_can_convert_into_dso_vec() {
        let datapoints = vec![
            create_datapoint("Awesome +tag +D:2022-02-10 +T:15-20-30"),
            create_datapoint("Stuff +another +D:2022-02-10 +T:15-20-31"),
        ];

        let datapoint_dsos: Vec<DatapointDSO> = DatapointDSO::vec_into(datapoints);

        assert_eq!(datapoint_dsos[0].data, "Awesome".to_string());
        assert_eq!(datapoint_dsos[1].data, "Stuff".to_string());
    }

    #[test]
    fn dso_vec_can_convert_into_datapoint_vec() {
        let datapoint_dsos = vec![
            DatapointDSO {
                datetime: 1698218241,
                data: "Stuff".to_string(),
                tags: vec!["tag".to_string()],
                key: 5,
            },
            DatapointDSO {
                datetime: 1698218242,
                data: "More".to_string(),
                tags: vec!["another".to_string()],
                key: 6,
            },
        ];

        let datapoints: Vec<Datapoint> = DatapointDSO::vec_into(datapoint_dsos);

        assert_eq!(datapoints[0].get_data(), &"Stuff".to_string());
        assert_eq!(datapoints[1].get_data(), &"More".to_string());
    }

    #[test]
    fn datapoint_dso_can_convert_into_datapoint() {
        let datapoint_dso = DatapointDSO {
            datetime: 1698216313,
            data: "Some stuff".to_string(),
            tags: vec!["tag".to_string()],
            key: 4,
        };

        let datapoint: Datapoint = datapoint_dso.into();

        assert_eq!(
            datapoint.get_datetime(),
            &Utc.with_ymd_and_hms(2023, 10, 25, 6, 45, 13).unwrap()
        );
        assert_eq!(datapoint.get_data(), &"Some stuff".to_string());
        assert_eq!(datapoint.get_tags(), &vec!["tag".to_string()]);
        assert_eq!(datapoint.get_key(), 4);
    }

    #[test]
    fn datapoint_can_be_converted_into_dso() {
        let datapoint = Datapoint::new(
            Utc.with_ymd_and_hms(2023, 10, 25, 6, 45, 13)
                .unwrap()
                .into(),
            "Some stuff".to_string(),
            vec!["tag".to_string()],
            4,
        );

        let datapoint_dso = DatapointDSO::from(datapoint);

        assert_eq!(datapoint_dso.datetime, 1698216313);
        assert_eq!(datapoint_dso.data, "Some stuff".to_string());
        assert_eq!(datapoint_dso.tags, vec!["tag".to_string()]);
        assert_eq!(datapoint_dso.key, 4);
    }
}
