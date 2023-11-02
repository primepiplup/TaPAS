use crate::datapoint::Datapoint;
use chrono::prelude::*;

pub fn generate_filename(timestamp: DateTime<Local>) -> String {
    format!("{}.png", timestamp.format("%Y%m%d%H%M%S"))
}

pub fn get_upper_lower<T: Copy + PartialOrd>(points: &Vec<T>) -> (T, T) {
    let mut lower: T = points[0];
    let mut upper: T = points[0];
    for point in points {
        if point < &lower {
            lower = *point;
        }
        if point > &upper {
            upper = *point;
        }
    }
    (lower, upper)
}

pub fn get_numeric_data(data: &Vec<Datapoint>) -> Option<(Vec<DateTime<Local>>, Vec<f64>)> {
    let mut number_collector = Vec::new();
    let mut date_collector = Vec::new();
    for datapoint in data {
        match datapoint.get_as_numeric() {
            Ok(num) => {
                number_collector.push(num);
                date_collector.push(datapoint.get_datetime().to_owned());
            }
            Err(_) => (),
        };
    }

    if date_collector.len() < 1 {
        return None;
    }

    Some((date_collector, number_collector))
}

pub fn apply_margin((lower, upper): (f64, f64)) -> (f64, f64) {
    let margin = (upper - lower) / 10.0;
    (lower - margin, upper + margin)
}

#[cfg(test)]
mod test {
    use crate::datapoint::create_datapoint;

    use super::*;

    #[test]
    fn apply_margin_function_applies_10_percent_margin() {
        let bounds = (40.0, 80.0);

        let margin_bounds = apply_margin(bounds);

        assert_eq!(margin_bounds, (36.0, 84.0));
    }

    #[test]
    fn generate_filename_function_takes_datetime_and_generates_filename_based_on_time() {
        let datetime = Local.with_ymd_and_hms(2023, 10, 16, 10, 6, 32).unwrap();

        let filename = generate_filename(datetime);

        assert_eq!(filename, "20231016100632.png".to_string());
    }

    #[test]
    fn get_upper_lower_returns_min_and_max_of_number_array() {
        let numbers: Vec<f64> = Vec::from([5.0, 800.0, 50.0, 45.0, 3.0, 1101.0, 32.0]);

        let (lower, upper) = get_upper_lower(&numbers);

        assert_eq!(lower, 3.0);
        assert_eq!(upper, 1101.0);
    }

    #[test]
    fn empty_input_into_get_numeric_data_returns_none() {
        let datapoints: Vec<Datapoint> = Vec::new();

        let output = get_numeric_data(&datapoints);

        assert_eq!(output, None);
    }

    #[test]
    fn get_numeric_data_for_datapoints_without_numbers_returns_empty_vector_for_numbers() {
        let mut datapoints: Vec<Datapoint> = Vec::new();
        datapoints.push(create_datapoint("things we don't care about"));
        datapoints.push(create_datapoint("how was your day! "));
        datapoints.push(create_datapoint("whoa cool idea +million-dollar-idea!"));

        let (_, actual) = get_numeric_data(&datapoints).unwrap();

        assert_eq!(actual, Vec::new());
    }

    #[test]
    fn get_numeric_data_for_datapoints_with_some_numbers_returns_only_numbers() {
        let mut datapoints: Vec<Datapoint> = Vec::new();
        datapoints.push(create_datapoint("things we don't care about"));
        datapoints.push(create_datapoint("how was your day! "));
        datapoints.push(create_datapoint("40 dollas idea +million-dollar-idea!"));

        let (_, actual) = get_numeric_data(&datapoints).unwrap();

        assert_eq!(actual, Vec::from([40.0]));
    }

    #[test]
    fn get_numeric_data_returns_float_values_from_datapoints() {
        let mut datapoints: Vec<Datapoint> = Vec::new();
        datapoints.push(create_datapoint("30.4kg +weight"));
        datapoints.push(create_datapoint("4 reps +reps"));
        datapoints.push(create_datapoint("20loc written today +work"));
        let expected = Vec::from([30.4, 4.0, 20.0]);

        let (_, actual) = get_numeric_data(&datapoints).unwrap();

        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn get_numeric_data_returns_a_vector_of_floats() {
        let mut datapoints: Vec<Datapoint> = Vec::new();
        datapoints.push(create_datapoint("30.4kg +weight"));
        datapoints.push(create_datapoint("4 reps +reps"));
        datapoints.push(create_datapoint("20loc written today +work"));

        let (_, actual) = get_numeric_data(&datapoints).unwrap();

        assert_eq!(actual.len(), 3);
    }
}
