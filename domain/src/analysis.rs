use chrono::{DateTime, Local};
use plotters::prelude::LogScalable;

pub fn linear_regression(data: Vec<(f64, f64)>) -> Box<dyn Fn(f64) -> f64> {
    let mut b = average(&data.clone().into_iter().map(|(x, y)| y).collect());
    let mut a = 0.0;
    let flat_line = fill_in_linear_equation(a, b);
    let flat_s_squared = sum_of_squares(&data, flat_line);

    Box::new(|x| linear_equation(0.5, x, 0.0))
}

pub fn data_processing(
    raw_data: Vec<(DateTime<Local>, f64)>,
) -> (Vec<(f64, f64)>, (f64, f64, f64, f64)) {
    let first_datetime = raw_data[0].0.timestamp().as_f64();
    let last_datetime = raw_data[raw_data.len() - 1].0.timestamp().as_f64() - first_datetime;
    let x_zeroed_normalized: Vec<(f64, f64)> = raw_data
        .into_iter()
        .map(|(datetime, number)| {
            (
                (datetime.timestamp().as_f64() - first_datetime) / last_datetime,
                number,
            )
        })
        .collect();

    let mut ymin = 0.0;
    let mut ymax = 0.0;
    for (_, y) in x_zeroed_normalized.clone() {
        if ymin > y {
            ymin = y;
        }

        if y > ymax {
            ymax = y;
        }
    }

    ymax = ymax - ymin;

    (
        x_zeroed_normalized
            .into_iter()
            .map(|(datetime, number)| (datetime, (number - ymin) / ymax))
            .collect(),
        (first_datetime, last_datetime, ymin, ymax),
    )
}

fn gradient_descent<'a, OF>(
    optimizing_function: &'a OF,
    initial_parameter: f64,
    data: &Vec<(f64, f64)>,
) -> f64
where
    OF: Fn(f64, f64) -> f64 + 'a,
{
    let mut step_size = 0.5;
    let mut step_direction = 1.0;

    let mut parameter = initial_parameter;

    let mut counter = 0;
    while counter < 100000 {
        counter += 1;

        parameter += step_size * step_direction;

        let slope = get_local_slope(optimizing_function, parameter, data);

        if step_direction < 0.0 && slope < 0.0 || step_direction > 0.0 && slope > 0.0 {
            step_direction *= -1.0;
        }

        step_size *= 0.9;
    }

    return parameter;
}

fn get_local_slope<'a, OF>(
    optimizing_function: &'a OF,
    parameter: f64,
    data: &Vec<(f64, f64)>,
) -> f64
where
    OF: Fn(f64, f64) -> f64 + 'a,
{
    let delta = 0.00001;

    let function = optimizing_function.to_owned();
    let function_left = Box::new(move |free| function(parameter, free));
    let cost_left = sum_of_squares(data, function_left);

    let delta_parameter = parameter + delta;
    let function = optimizing_function.to_owned();
    let function_right = Box::new(move |free| function(delta_parameter, free));
    let cost_right = sum_of_squares(data, function_right);

    let (slope, _) =
        find_linear_parameters(&(parameter, cost_left), &(delta_parameter, cost_right));
    return slope;
}

fn average(nums: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    let mut counter = 0.0;
    for num in nums {
        sum += num;
        counter += 1.0;
    }
    sum / counter
}

fn sum_of_squares<'a>(
    data: &Vec<(f64, f64)>,
    predictive_function: Box<dyn Fn(f64) -> f64 + 'a>,
) -> f64 {
    let mut sum = 0.0;
    for (x, y) in data {
        let diff = y - predictive_function(*x);
        let square = diff * diff;
        sum += square;
    }
    return sum;
}

fn fill_in_linear_equation(a: f64, b: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| linear_equation(a, x, b))
}

fn linear_equation(a: f64, x: f64, b: f64) -> f64 {
    a * x + b
}

fn find_linear_parameters((x1, y1): &(f64, f64), (x2, y2): &(f64, f64)) -> (f64, f64) {
    let a = (y2 - y1) / (x2 - x1);
    let b = y1 - a * x1;
    (a, b)
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn gradient_descent_for_gradient_of_linear_datapoints_finds_correct_slope<'a>() {
        let data = vec![(2.0, 1.0), (5.0, 3.0)];
        let expected = 0.666;
        let intercept = -0.333;
        let initial_flat_slope = 0.0;
        let intercepted_linear: Box<dyn Fn(f64, f64) -> f64 + 'a> =
            Box::new(move |a, x| linear_equation(a, x, intercept));

        let found_slope = gradient_descent(&intercepted_linear, initial_flat_slope, &data);

        let diff = found_slope - expected;

        assert!(diff < 0.01 && diff > -0.01);
    }

    #[test]
    fn average_gives_average_of_float_vector() {
        let nums = vec![3.0, 2.0, 1.0, 4.0, 5.0];

        let avg = average(&nums);

        assert_eq!(avg, 3.0);
    }

    #[test]
    fn data_processing_transforms_x_axis_to_be_based_on_zero_and_normalized_to_max() {
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
                2.0,
            ),
        ];

        let (data, _) = data_processing(raw_data);

        assert_eq!(data[0].0, 0.0);
        assert_eq!(data[1].0, 0.25);
        assert_eq!(data[2].0, 1.0);
    }

    #[test]
    fn linear_equation_halfx_plus_zero_provides_accurate_results() {
        let eq = |x| linear_equation(0.5, x, 0.0);

        assert_eq!(eq(2.0), 1.0);
        assert_eq!(eq(4.0), 2.0);
    }

    #[test]
    fn sum_of_squares_is_accurate_for_basic_linear_equation() {
        let data = vec![(4.0, 1.0), (2.0, 2.0)];
        let eq = Box::new(|x| linear_equation(0.5, x, 0.0));

        let s_square = sum_of_squares(&data, eq);

        assert_eq!(s_square, 2.0);
    }
}
