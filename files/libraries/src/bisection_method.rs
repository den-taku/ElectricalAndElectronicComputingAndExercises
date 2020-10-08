mod bisection_method {
    pub use std::ops::Range;

    pub fn bisection_method<F>(mut range: Range<f64>, e: f64, f: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let x_new = (range.end + range.start) / 2.;
        if f(x_new) * f(range.start) >= 0. {
            range.start = x_new;
        } else {
            range.end = x_new;
        }
        if range.end - range.start <= e {
            x_new
        } else {
            bisection_method(range, e, f)
        }
    }
}
// definition for f64, TODO: make this generic

#[cfg(test)]
mod tests_bisection_method {
    use crate::bisection_method::bisection_method::*;

    #[test]
    fn tests_bisection_method() {
        let f = |x: f64| x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.;
        assert_eq!(bisection_method(-2f64..0f64, 1e-3, f), -1.4150390625);
        assert_eq!(bisection_method(-2f64..0f64, 1e-4, f), -1.41424560546875);
        assert_eq!(bisection_method(-2f64..0f64, 1e-5, f), -1.4142074584960938);
    }
}
