use crate::datapoint::Datapoint;
use crate::parsedquery::ParsedQuery;
use crate::plotter::plotcolors::PlotColors;
use crate::plotter::util::*;
use crate::stats::model_fit::linear_regression;
use chrono::{DateTime, Local, TimeZone};
use plotters::prelude::*;
use std::io::Error;

pub fn scatterplot(
    data: &Vec<Datapoint>,
    parsed_query: ParsedQuery,
    with_regression: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let (datetimes, num_data) = match get_numeric_data(data) {
        Some(result) => result,
        None => return Err(Box::new(Error::new(std::io::ErrorKind::NotFound, "test"))),
    };

    let (lower_date, upper_date): (DateTime<Local>, DateTime<Local>) = get_daterange(&datetimes);
    let (lower_num, upper_num): (f64, f64) = apply_margin(get_upper_lower(&num_data));
    let as_date: bool = plot_as_dates((lower_date, upper_date));
    let datapoints: Vec<(DateTime<Local>, f64)> = datetimes.into_iter().zip(num_data).collect();

    let filename = generate_filename(Local::now());
    let location: String = format!("generated/{}", filename);
    let plot_title: String = parsed_query.generate_plot_title();

    let plot_colors = PlotColors::new();
    let root = BitMapBackend::new(&location, (640, 480)).into_drawing_area();
    root.fill(plot_colors.background())?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            plot_title,
            ("sans-serif", 35)
                .with_color(plot_colors.textcolor())
                .into_text_style(&root),
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(lower_date..upper_date, lower_num..upper_num)?;

    chart
        .configure_mesh()
        .label_style(plot_colors.highlight())
        .axis_style(plot_colors.textcolor())
        .bold_line_style(plot_colors.highlight())
        .light_line_style(plot_colors.darklight())
        .x_label_formatter(&|datetime| format_datetime(datetime, as_date))
        .draw()?;

    chart
        .draw_series(
            datapoints
                .iter()
                .map(|coord| Circle::new(*coord, 5, plot_colors.labelstyle().clone())),
        )
        .unwrap();

    if with_regression {
        let linear_function = linear_regression(datapoints.clone(), 50);
        let fitted_line = linear_function.function();
        chart
            .draw_series(LineSeries::new(
                datapoints
                    .iter()
                    .map(|(datetime, _)| (*datetime, fitted_line(datetime.timestamp().as_f64()))),
                &BLUE,
            ))
            .unwrap();
    }

    root.present()?;

    Ok(filename.to_owned())
}

fn get_daterange(data: &Vec<DateTime<Local>>) -> (DateTime<Local>, DateTime<Local>) {
    if data.len() > 1 {
        let lower = data[0];
        let upper = data[data.len() - 1];
        (lower, upper)
    } else if data.len() == 1 {
        (data[0], data[0])
    } else {
        let time = Local.with_ymd_and_hms(1999, 8, 26, 1, 2, 3).unwrap();
        (time, time)
    }
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

#[cfg(test)]
mod test {
    use crate::datapoint::create_datapoint;
    use chrono::Duration;

    use super::*;

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
    fn empty_input_into_plot_returns_error() {
        let datapoints: Vec<Datapoint> = Vec::new();

        let output = scatterplot(&datapoints, ParsedQuery::from(Vec::new()), false);

        assert_eq!(output.ok(), None);
    }

    #[test]
    fn get_daterange_returns_first_and_last_date_for_two_or_more_datapoints() {
        let mut datapoints: Vec<DateTime<Local>> = Vec::new();
        datapoints.push(create_datapoint("stuff").get_datetime().to_owned());
        datapoints.push(create_datapoint("more stuff").get_datetime().to_owned());
        datapoints.push(
            create_datapoint("even more stuff")
                .get_datetime()
                .to_owned(),
        );
        let expected_lower = datapoints[0];
        let expected_upper = datapoints[2];

        let (lower, upper) = get_daterange(&datapoints);

        assert_eq!(lower, expected_lower);
        assert_eq!(upper, expected_upper);
    }

    #[test]
    fn get_daterange_returns_same_date_for_single_datapoint() {
        let mut datapoints: Vec<DateTime<Local>> = Vec::new();
        datapoints.push(create_datapoint("stuff").get_datetime().to_owned());
        let expected = datapoints[0];

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
