pub fn sum_of_squares<'a>(
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

pub fn average(nums: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    let mut counter = 0.0;
    for num in nums {
        sum += num;
        counter += 1.0;
    }
    sum / counter
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::linearfunction::linear_equation;

    #[test]
    fn average_gives_average_of_float_vector() {
        let nums = vec![3.0, 2.0, 1.0, 4.0, 5.0];

        let avg = average(&nums);

        assert_eq!(avg, 3.0);
    }

    #[test]
    fn sum_of_squares_is_accurate_for_basic_linear_equation() {
        let data = vec![(4.0, 1.0), (2.0, 2.0)];
        let eq = Box::new(|x| linear_equation(0.5, x, 0.0));

        let s_square = sum_of_squares(&data, eq);

        assert_eq!(s_square, 2.0);
    }
}
