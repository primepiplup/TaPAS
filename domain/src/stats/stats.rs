use crate::{datapoint::Datapoint, parsedquery::ParsedQuery, stats::t_distribution::TTable};

use super::{preprocess::into_categorical, summary::Summary};

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
    if counter == 0.0 {
        return 0.0;
    }
    sum / counter
}

pub fn variance(nums: &Vec<f64>) -> f64 {
    let mean = average(nums);
    let mut s_square: f64 = 0.0;
    nums.into_iter()
        .for_each(|num| s_square += (num - mean).powi(2));
    return s_square / degrees_of_freedom(nums);
}

pub fn degrees_of_freedom(nums: &Vec<f64>) -> f64 {
    return nums.len() as f64 - 1.0;
}

pub fn weighted_average_of_variances(sample_1: &Vec<f64>, sample_2: &Vec<f64>) -> f64 {
    let dof1 = degrees_of_freedom(sample_1);
    let dof2 = degrees_of_freedom(sample_2);
    let variance1 = variance(sample_1);
    let variance2 = variance(sample_2);
    return (dof1 * variance1 + dof2 * variance2) / (dof1 + dof2);
}

pub fn pooled_two_sample_t_test(sample_1: &Vec<f64>, sample_2: &Vec<f64>) -> (f64, f64) {
    let prop_1 = 1.0 / sample_1.len() as f64;
    let prop_2 = 1.0 / sample_2.len() as f64;
    let waov = weighted_average_of_variances(sample_1, sample_2);
    let mean_1 = average(sample_1);
    let mean_2 = average(sample_2);
    let dof = degrees_of_freedom(sample_1) + degrees_of_freedom(sample_2);
    let t = (mean_1 - mean_2) / (waov * (prop_1 + prop_2)).sqrt();
    return (t, dof);
}

pub fn compare(samples: &Vec<(Vec<Datapoint>, ParsedQuery)>) -> Vec<Summary> {
    let samples = into_categorical(samples);
    if samples.len() == 2 {
        two_group_comparison(samples)
    } else {
        Vec::new()
    }
}

fn two_group_comparison(samples: Vec<(Vec<f64>, String)>) -> Vec<Summary> {
    let ttable = TTable::new();
    let (t, dof) = pooled_two_sample_t_test(&samples[0].0, &samples[1].0);
    let p = ttable.get_p_for(dof, t.abs());
    let summary1 = Summary::from(samples[0].0.clone())
        .set_name(samples[0].1.clone())
        .set_p(p);
    let summary2 = Summary::from(samples[1].0.clone())
        .set_name(samples[1].1.clone())
        .set_p(p);
    vec![summary1, summary2]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{datastore::Datastore, linearfunction::linear_equation};

    #[test]
    fn comparison_between_two_samples_provides_accurate_summaries() {
        let datastore: Datastore = Datastore::new();
        datastore.add_datapoint("40 +one");
        datastore.add_datapoint("41 +one");
        datastore.add_datapoint("39 +one");
        datastore.add_datapoint("30 +two");
        datastore.add_datapoint("31 +two");
        datastore.add_datapoint("29 +two");
        let mut collector = Vec::new();
        collector.push(datastore.query("one"));
        collector.push(datastore.query("two"));

        let summaries = compare(&collector);

        assert_eq!(summaries[0].get_mean(), 40.0);
        assert_eq!(summaries[0].get_p(), 0.001);
        assert_eq!(summaries[1].get_mean(), 30.0);
        assert_eq!(summaries[1].get_p(), 0.001);
    }

    #[test]
    fn pooled_two_sample_t_test_is_accurate() {
        let sample_1 = vec![42.1, 41.3, 42.4, 43.2, 41.8, 41.0, 41.8, 42.8, 42.3, 42.7];
        let sample_2 = vec![42.7, 43.8, 42.5, 43.1, 44.0, 43.6, 43.3, 43.5, 41.7, 44.1];

        let (t, _) = pooled_two_sample_t_test(&sample_1, &sample_2);
        println!("{}", t);
        let expect_actual_diff = t + 3.398;

        assert!(expect_actual_diff < 0.001 && expect_actual_diff > -0.001);
    }

    #[test]
    fn accurate_weighted_average_of_variances_for_t_test() {
        let sample_1 = vec![42.1, 41.3, 42.4, 43.2, 41.8, 41.0, 41.8, 42.8, 42.3, 42.7];
        let sample_2 = vec![42.7, 43.8, 42.5, 43.1, 44.0, 43.6, 43.3, 43.5, 41.7, 44.1];

        let waov = weighted_average_of_variances(&sample_1, &sample_2);
        println!("{}", waov);
        let expect_actual_diff = waov - 0.5145;

        assert!(expect_actual_diff < 0.001 && expect_actual_diff > -0.001);
    }

    #[test]
    fn degrees_of_freedom_is_samplesize_minus_one() {
        let nums = vec![46.0, 69.0, 32.0, 60.0, 52.0, 41.0];

        let d_o_f = degrees_of_freedom(&nums);

        assert_eq!(d_o_f, 5.0);
    }

    #[test]
    fn variance_can_be_accurately_determined() {
        let nums = vec![46.0, 69.0, 32.0, 60.0, 52.0, 41.0];

        let variance = variance(&nums);

        assert_eq!(variance, 177.2);
    }

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
