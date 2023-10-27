#[derive(Debug, PartialEq)]
pub struct LinearFunction {
    slope: f64,
    intercept: f64,
    xtrans: f64,
    xscale: f64,
    ytrans: f64,
    yscale: f64,
}

impl LinearFunction {
    pub fn apply(&self, x: f64) -> f64 {
        linear_equation(
            self.slope * self.yscale,
            (x - self.xtrans) / self.xscale,
            self.intercept * self.yscale + self.ytrans,
        )
    }

    pub fn apply_inverse(&self, y: f64) -> f64 {
        inverse_linear(
            self.slope * self.yscale,
            y * self.xscale,
            self.xscale * (self.intercept * self.yscale + self.ytrans),
        ) + self.xtrans
    }

    pub fn function<'a>(&'a self) -> Box<dyn Fn(f64) -> f64 + 'a> {
        Box::new(|x| {
            linear_equation(
                self.slope * self.yscale,
                (x - self.xtrans) / self.xscale,
                self.intercept * self.yscale + self.ytrans,
            )
        })
    }

    pub fn inverse<'a>(&'a self) -> Box<dyn Fn(f64) -> f64 + 'a> {
        Box::new(|y| {
            inverse_linear(
                self.slope * self.yscale,
                y * self.xscale,
                self.xscale * (self.intercept * self.yscale + self.ytrans),
            ) + self.xtrans
        })
    }

    pub fn new(slope: f64, intercept: f64) -> LinearFunction {
        LinearFunction {
            slope,
            intercept,
            xtrans: 0.0,
            xscale: 1.0,
            ytrans: 0.0,
            yscale: 1.0,
        }
    }

    pub fn with_transform(
        &self,
        (xtrans, xscale, ytrans, yscale): (f64, f64, f64, f64),
    ) -> LinearFunction {
        LinearFunction {
            slope: self.slope,
            intercept: self.intercept,
            xtrans,
            xscale,
            ytrans,
            yscale,
        }
    }
}

pub fn linear_equation(a: f64, x: f64, b: f64) -> f64 {
    a * x + b
}

fn inverse_linear(a: f64, y: f64, b: f64) -> f64 {
    (y - b) / a
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn with_transform_applies_a_transform_to_the_linear_function() {
        let basic_line = LinearFunction::new(1.0, 2.0);

        let transformed = basic_line.with_transform((2.0, 2.0, -1.0, 0.5));

        let y1 = transformed.apply(2.0);
        let y2 = transformed.apply(5.0);
        let y3 = transformed.apply(7.0);

        assert_eq!(y1, 0.0);
        assert_eq!(y2, 0.75);
        assert_eq!(y3, 1.25);
    }

    #[test]
    fn with_transform_applies_a_transform_to_the_linear_function_including_inverse() {
        let basic_line = LinearFunction::new(1.0, 2.0);

        let transformed = basic_line.with_transform((2.0, 2.0, -1.0, 0.5));

        let x1 = transformed.apply_inverse(0.0);
        let x2 = transformed.apply_inverse(1.0);
        let x3 = transformed.apply_inverse(-2.0);

        assert_eq!(x1, 2.0);
        assert_eq!(x2, 6.0);
        assert_eq!(x3, -6.0);
    }

    #[test]
    fn new_returns_linearfunction_object() {
        let expected = LinearFunction {
            slope: 1.0,
            intercept: 2.0,
            xtrans: 0.0,
            xscale: 1.0,
            ytrans: 0.0,
            yscale: 1.0,
        };

        let actual = LinearFunction::new(1.0, 2.0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn linear_equation_delivers_expected_results() {
        let y1 = linear_equation(2.0, 4.0, 2.0);
        let y2 = linear_equation(2.0, 5.0, 2.0);

        assert_eq!(y1, 10.0);
        assert_eq!(y2, 12.0);
    }

    #[test]
    fn linear_equation_halfx_plus_zero_provides_accurate_results() {
        let eq = |x| linear_equation(0.5, x, 0.0);

        assert_eq!(eq(2.0), 1.0);
        assert_eq!(eq(4.0), 2.0);
    }

    #[test]
    fn inverse_linear_delivers_expected_results() {
        let x1 = inverse_linear(2.0, 10.0, 2.0);
        let x2 = inverse_linear(2.0, 12.0, 2.0);

        assert_eq!(x1, 4.0);
        assert_eq!(x2, 5.0);
    }

    #[test]
    fn apply_returns_expected_linear_results() {
        let linear_equation = LinearFunction::new(2.0, 2.0);

        let y1 = linear_equation.apply(4.0);
        let y2 = linear_equation.apply(5.0);

        assert_eq!(y1, 10.0);
        assert_eq!(y2, 12.0);
    }

    #[test]
    fn inverse_apply_returns_expected_x_for_y() {
        let linear_equation = LinearFunction::new(2.0, 2.0);

        let x1 = linear_equation.apply_inverse(10.0);
        let x2 = linear_equation.apply_inverse(12.0);

        assert_eq!(x1, 4.0);
        assert_eq!(x2, 5.0);
    }

    #[test]
    fn function_returns_filled_in_linear_function_to_be_passed_around() {
        let linear_equation = LinearFunction::new(2.0, 2.0);

        let function = linear_equation.function();
        let y1 = function(4.0);
        let y2 = function(5.0);

        assert_eq!(y1, 10.0);
        assert_eq!(y2, 12.0);
    }

    #[test]
    fn inverse_returns_filled_in_inverse_function_to_be_passed_around() {
        let linear_equation = LinearFunction::new(2.0, 2.0);

        let function = linear_equation.inverse();
        let x1 = function(10.0);
        let x2 = function(12.0);

        assert_eq!(x1, 4.0);
        assert_eq!(x2, 5.0);
    }
}
