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

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

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
