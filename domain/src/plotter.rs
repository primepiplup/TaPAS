use std::io::Error;

use crate::datapoint::{create_datapoint, Datapoint};
use chrono::{DateTime, Local, TimeZone};
use plotters::prelude::*;

pub fn basic_plot(
    data: &Vec<Datapoint>,
    parsed_query: Vec<Vec<String>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let num_data = match get_numeric_data(data) {
        Some(result) => result,
        None => return Err(Box::new(Error::new(std::io::ErrorKind::NotFound, "test"))),
    };

    let datetimes = get_datetimes(data);
    let (lower_date, upper_date): (DateTime<Local>, DateTime<Local>) = get_daterange(data);
    let (lower_num, upper_num): (f32, f32) = apply_margin(get_upper_lower(&num_data));
    let as_date: bool = plot_as_dates((lower_date, upper_date));
    let datapoints: Vec<(DateTime<Local>, f32)> = datetimes.into_iter().zip(num_data).collect();

    let filename = generate_filename(Local::now());
    let location: String = format!("generated/{}", filename);
    let plot_title: String = generate_plot_title(parsed_query);

    let root = BitMapBackend::new(&location, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(plot_title, ("sans-serif", 35).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(lower_date..upper_date, lower_num..upper_num)?;

    chart
        .configure_mesh()
        .x_label_formatter(&|datetime| format_datetime(datetime, as_date))
        .draw()?;

    chart
        .draw_series(datapoints.iter().map(|coord| Circle::new(*coord, 5, &BLUE)))
        .unwrap();

    root.present()?;

    Ok(filename.to_owned())
}

fn get_numeric_data(data: &Vec<Datapoint>) -> Option<Vec<f32>> {
    if data.len() < 1 {
        return None;
    }

    let mut collector = Vec::new();
    for datapoint in data {
        let num = match datapoint.get_as_numeric() {
            Ok(num) => num,
            Err(_) => return None,
        };
        collector.push(num);
    }

    Some(collector)
}

fn get_datetimes(data: &Vec<Datapoint>) -> Vec<DateTime<Local>> {
    data.into_iter()
        .map(|datapoint| datapoint.get_datetime().to_owned())
        .collect()
}

fn get_daterange(data: &Vec<Datapoint>) -> (DateTime<Local>, DateTime<Local>) {
    if data.len() > 1 {
        let lower = data[0].get_datetime().to_owned();
        let upper = data[data.len() - 1].get_datetime().to_owned();
        (lower, upper)
    } else if data.len() == 1 {
        (
            data[0].get_datetime().to_owned(),
            data[0].get_datetime().to_owned(),
        )
    } else {
        let time = Local.with_ymd_and_hms(1999, 8, 26, 1, 2, 3).unwrap();
        (time, time)
    }
}

fn generate_filename(timestamp: DateTime<Local>) -> String {
    format!("{}.png", timestamp.format("%Y%m%d%H%M%S"))
}

fn generate_plot_title(parsed_query: Vec<Vec<String>>) -> String {
    let tags: Vec<String> = parsed_query
        .into_iter()
        .map(|elem| elem[0].clone())
        .collect();
    format!("Plot for: {}", tags.join(", "))
}

fn format_datetime(datetime: &DateTime<Local>, as_date: bool) -> String {
    if as_date {
        format!("{}", datetime.format("%F"))
    } else {
        format!("{}", datetime.format("%T"))
    }
}

fn plot_as_dates((early, late): (DateTime<Local>, DateTime<Local>)) -> bool {
    let difference = late - early;
    difference.num_days() > 2
}

fn get_upper_lower<T: Copy + PartialOrd>(points: &Vec<T>) -> (T, T) {
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

fn apply_margin((lower, upper): (f32, f32)) -> (f32, f32) {
    let margin = (upper - lower) / 10.0;
    (lower - margin, upper + margin)
}

#[cfg(test)]
mod test {
    use chrono::Duration;

    use super::*;

    #[test]
    fn generate_filename_function_takes_datetime_and_generates_filename_based_on_time() {
        let datetime = Local.with_ymd_and_hms(2023, 10, 16, 10, 6, 32).unwrap();

        let filename = generate_filename(datetime);

        assert_eq!(filename, "20231016100632.png".to_string());
    }

    #[test]
    fn generate_plot_title_takes_all_elements_of_vector_and_returns_title() {
        let parsed = vec![
            vec!["something".to_string(), "value".to_string()],
            vec!["tag".to_string()],
            vec!["else".to_string()],
        ];

        let title = generate_plot_title(parsed);

        assert_eq!(title, "Plot for: something, tag, else")
    }

    #[test]
    fn plot_as_dates_returns_true_if_dates_more_than_two_days_apart() {
        let datetime = Local.with_ymd_and_hms(2023, 10, 16, 10, 6, 32).unwrap();
        let datetime_2 = datetime + Duration::days(3);

        assert!(plot_as_dates((datetime, datetime_2)));
    }

    #[test]
    fn plot_as_dates_returns_false_if_dates_less_than_two_days_apart() {
        let datetime = Local.with_ymd_and_hms(2023, 10, 16, 10, 6, 32).unwrap();
        let datetime_2 = datetime + Duration::days(1);

        assert!(!plot_as_dates((datetime, datetime_2)));
    }

    #[test]
    fn format_datetime_with_as_date_true_returns_date_formatted_string() {
        let datetime = Local.with_ymd_and_hms(2023, 10, 16, 10, 6, 32).unwrap();

        let formatted = format_datetime(&datetime, true);

        assert_eq!(formatted, "2023-10-16".to_owned());
    }

    #[test]
    fn format_datetime_with_as_date_false_returns_time_formatted_string() {
        let datetime = Local.with_ymd_and_hms(2023, 10, 16, 10, 6, 32).unwrap();

        let formatted = format_datetime(&datetime, false);

        assert_eq!(formatted, "10:06:32".to_owned());
    }

    #[test]
    fn apply_margin_function_applies_10_percent_margin() {
        let bounds = (40.0, 80.0);

        let margin_bounds = apply_margin(bounds);

        assert_eq!(margin_bounds, (36.0, 84.0));
    }

    #[test]
    fn empty_input_into_plot_returns_error() {
        let datapoints: Vec<Datapoint> = Vec::new();

        let output = basic_plot(&datapoints, Vec::new());

        assert_eq!(output.ok(), None);
    }

    #[test]
    fn empty_input_into_get_numeric_data_returns_none() {
        let datapoints: Vec<Datapoint> = Vec::new();

        let output = get_numeric_data(&datapoints);

        assert_eq!(output, None);
    }

    #[test]
    fn get_numeric_data_for_datapoints_without_numbers_returns_none() {
        let mut datapoints: Vec<Datapoint> = Vec::new();
        datapoints.push(create_datapoint("things we don't care about"));
        datapoints.push(create_datapoint("how was your day! "));
        datapoints.push(create_datapoint("whoa cool idea +million-dollar-idea!"));

        let actual = get_numeric_data(&datapoints);

        assert_eq!(actual, None);
    }

    #[test]
    fn get_numeric_data_returns_float_values_from_datapoints() {
        let mut datapoints: Vec<Datapoint> = Vec::new();
        datapoints.push(create_datapoint("30.4kg +weight"));
        datapoints.push(create_datapoint("4 reps +reps"));
        datapoints.push(create_datapoint("20loc written today +work"));
        let expected = Vec::from([30.4, 4.0, 20.0]);

        let actual = get_numeric_data(&datapoints).unwrap();

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

        let actual = get_numeric_data(&datapoints).unwrap();

        assert_eq!(actual.len(), 3);
    }

    #[test]
    fn get_upper_lower_returns_min_and_max_of_number_array() {
        let numbers: Vec<f32> = Vec::from([5.0, 800.0, 50.0, 45.0, 3.0, 1101.0, 32.0]);

        let (lower, upper) = get_upper_lower(&numbers);

        assert_eq!(lower, 3.0);
        assert_eq!(upper, 1101.0);
    }

    #[test]
    fn get_daterange_returns_first_and_last_date_for_two_or_more_datapoints() {
        let mut datapoints: Vec<Datapoint> = Vec::new();
        datapoints.push(create_datapoint("stuff"));
        datapoints.push(create_datapoint("more stuff"));
        datapoints.push(create_datapoint("even more stuff"));
        let expected_lower = datapoints[0].get_datetime().to_owned();
        let expected_upper = datapoints[2].get_datetime().to_owned();

        let (lower, upper) = get_daterange(&datapoints);

        assert_eq!(lower, expected_lower);
        assert_eq!(upper, expected_upper);
    }

    #[test]
    fn get_dates_returns_vector_of_dates_from_datapoints() {
        let mut datapoints: Vec<Datapoint> = Vec::new();
        datapoints.push(create_datapoint("stuff"));
        datapoints.push(create_datapoint("even more stuff"));
        let date_one = datapoints[0].get_datetime().to_owned();
        let date_two = datapoints[1].get_datetime().to_owned();

        let gotten_dates = get_datetimes(&datapoints);

        assert_eq!(gotten_dates[0], date_one);
        assert_eq!(gotten_dates[1], date_two);
    }

    #[test]
    fn get_daterange_returns_same_date_for_single_datapoint() {
        let mut datapoints: Vec<Datapoint> = Vec::new();
        datapoints.push(create_datapoint("stuff"));
        let expected = datapoints[0].get_datetime().to_owned();

        let (lower, upper) = get_daterange(&datapoints);

        assert_eq!(lower, expected);
        assert_eq!(upper, expected);
    }

    #[test]
    fn get_daterange_returns_0_0_when_no_data_passed() {
        let time = Local.with_ymd_and_hms(1999, 8, 26, 1, 2, 3).unwrap();
        let (lower, upper) = get_daterange(&Vec::new());

        assert_eq!(lower, time);
        assert_eq!(upper, time);
    }
}
