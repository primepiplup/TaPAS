use crate::linearfunction::{linear_equation, LinearFunction};
use crate::stats::preprocess::date_data_processing;
use crate::stats::stats::{average, sum_of_squares};
use chrono::{DateTime, Local};

pub fn linear_regression(data: Vec<(DateTime<Local>, f64)>, steps: usize) -> LinearFunction {
    let (data, transform) = date_data_processing(data);

    let mut b = average(&data.clone().into_iter().map(|(_, y)| y).collect());
    let mut a = 0.0;

    let mut counter = 0;
    while counter < steps {
        counter += 1;
        a = gradient_descent(&|a_optim, x| linear_equation(a_optim, x, b), a, &data);
        b = gradient_descent(&|b_optim, x| linear_equation(a, x, b_optim), b, &data);
    }

    LinearFunction::new(a, b).with_transform(transform)
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

fn find_linear_parameters((x1, y1): &(f64, f64), (x2, y2): &(f64, f64)) -> (f64, f64) {
    let a = (y2 - y1) / (x2 - x1);
    let b = y1 - a * x1;
    (a, b)
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;
    use plotters::prelude::LogScalable;

    use super::*;

    #[test]
    fn linear_regression_provides_an_accurate_fit_for_datapoints_on_a_line() {
        let raw_data = vec![
            (
                Local.with_ymd_and_hms(2023, 10, 15, 15, 20, 2).unwrap(),
                2.0,
            ),
            (
                Local.with_ymd_and_hms(2023, 10, 15, 15, 20, 4).unwrap(),
                3.0,
            ),
            (
                Local.with_ymd_and_hms(2023, 10, 15, 15, 20, 6).unwrap(),
                4.0,
            ),
        ];

        let linear_function = linear_regression(raw_data, 10);
        let fitted_line = linear_function.function();

        let res1 = fitted_line(
            Local
                .with_ymd_and_hms(2023, 10, 15, 15, 20, 2)
                .unwrap()
                .timestamp()
                .as_f64(),
        ) - 2.0;

        let res2 = fitted_line(
            Local
                .with_ymd_and_hms(2023, 10, 15, 15, 20, 4)
                .unwrap()
                .timestamp()
                .as_f64(),
        ) - 3.0;

        let res3 = fitted_line(
            Local
                .with_ymd_and_hms(2023, 10, 15, 15, 20, 6)
                .unwrap()
                .timestamp()
                .as_f64(),
        ) - 4.0;

        assert!(res1 < 0.01 && res1 > -0.01);
        assert!(res2 < 0.01 && res2 > -0.01);
        assert!(res3 < 0.01 && res3 > -0.01);
    }

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
}
