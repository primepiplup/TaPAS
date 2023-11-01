use crate::numericaldata::NumericalData;
use crate::queryresult::QueryResult;
use chrono::{DateTime, Local};
use plotters::prelude::LogScalable;

pub fn data_processing(raw_data: Vec<(f64, f64)>) -> (Vec<(f64, f64)>, (f64, f64, f64, f64)) {
    /* returns (data, (x_translation, x_scaling, y_translation, y_scaling))*/
    let mut xmin = raw_data[0].0;
    let mut xmax = raw_data[0].0;
    let mut ymin = raw_data[0].1;
    let mut ymax = raw_data[0].1;
    for (x, y) in raw_data.clone() {
        if ymin > y {
            ymin = y;
        }

        if xmin > x {
            xmin = x;
        }

        if y > ymax {
            ymax = y;
        }

        if x > xmax {
            xmax = x;
        }
    }

    let yscale = ymax - ymin;
    let xscale = xmax - xmin;

    (
        raw_data
            .into_iter()
            .map(|(x, y)| ((x - xmin) / xscale, (y - ymin) / yscale))
            .collect(),
        (xmin, xscale, ymin, yscale),
    )
}

pub fn date_data_processing(
    raw_data: Vec<(DateTime<Local>, f64)>,
) -> (Vec<(f64, f64)>, (f64, f64, f64, f64)) {
    /* returns (data, (x_translation, x_scaling, y_translation, y_scaling))*/
    let numeric_data: Vec<(f64, f64)> = raw_data
        .into_iter()
        .map(|(datetime, number)| (datetime.timestamp().as_f64(), number))
        .collect();

    data_processing(numeric_data)
}

pub fn into_categorical(datasets: &Vec<QueryResult>) -> Vec<NumericalData> {
    let mut collector: Vec<NumericalData> = Vec::new();
    for dataset in datasets {
        let numericdata = NumericalData::from(dataset);
        if !numericdata.is_empty() {
            collector.push(numericdata);
        }
    }
    return collector;
}

#[cfg(test)]
mod test {
    use crate::datastore::Datastore;
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn into_categorical_takes_a_vector_of_retrieved_datasets_and_queries_and_returns_titled_data_vectors(
    ) {
        let mut collector: Vec<QueryResult> = Vec::new();
        let datastore = Datastore::new();
        datastore.add_datapoint("6 hours +coffee");
        datastore.add_datapoint("7 hours +coffee");
        datastore.add_datapoint("7 hours +tea");
        datastore.add_datapoint("8 hours +tea");
        collector.push(datastore.query("coffee"));
        collector.push(datastore.query("tea"));

        let titled_data = into_categorical(&collector);

        assert_eq!(titled_data[0].get_data()[0], 6.0);
        assert_eq!(titled_data[0].get_data()[1], 7.0);
        assert_eq!(titled_data[1].get_data()[0], 7.0);
        assert_eq!(titled_data[1].get_data()[1], 8.0);
        assert_eq!(titled_data[0].get_title(), "coffee".to_string());
        assert_eq!(titled_data[1].get_title(), "tea".to_string());
    }

    #[test]
    fn data_processing_transforms_x_axis_to_be_based_on_zero_and_normalized_to_max() {
        let raw_data = vec![
            (
                Local.with_ymd_and_hms(2023, 10, 15, 15, 20, 21).unwrap(),
                2.0,
            ),
            (
                Local.with_ymd_and_hms(2023, 10, 15, 15, 20, 20).unwrap(),
                1.0,
            ),
            (
                Local.with_ymd_and_hms(2023, 10, 15, 15, 20, 24).unwrap(),
                2.0,
            ),
        ];

        let (data, _) = date_data_processing(raw_data);

        assert_eq!(data[0].0, 0.25);
        assert_eq!(data[1].0, 0.0);
        assert_eq!(data[2].0, 1.0);
    }

    #[test]
    fn data_processing_transforms_y_axis_to_be_based_on_zero_and_normalized_to_max() {
        let raw_data = vec![
            (
                Local.with_ymd_and_hms(2023, 10, 15, 15, 20, 20).unwrap(),
                1.0,
            ),
            (
                Local.with_ymd_and_hms(2023, 10, 15, 15, 20, 21).unwrap(),
                2.0,
            ),
            (
                Local.with_ymd_and_hms(2023, 10, 15, 15, 20, 24).unwrap(),
                5.0,
            ),
        ];

        let (data, _) = date_data_processing(raw_data);

        assert_eq!(data[0].1, 0.0);
        assert_eq!(data[1].1, 0.25);
        assert_eq!(data[2].1, 1.0);
    }
}
