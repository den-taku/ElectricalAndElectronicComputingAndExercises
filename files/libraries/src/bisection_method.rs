#![allow(dead_code)]

// pub mod bisection_method {
pub use std::ops::Range;
pub use std::rc::Rc;

pub fn bisection_method(
    range: Range<f64>,
    e: f64,
    f: Rc<dyn Fn(f64) -> f64>,
    expected_value: f64,
) -> (f64, Vec<(f64, f64)>) {
    let data: Vec<(f64, f64)> = Vec::new();
    bisection_method_inner(range, e, f, 1, expected_value, data)
}

fn bisection_method_inner(
    mut range: Range<f64>,
    e: f64,
    f: Rc<dyn Fn(f64) -> f64>,
    times: usize,
    expected_value: f64,
    mut data: Vec<(f64, f64)>,
) -> (f64, Vec<(f64, f64)>) {
    let x_new = (range.end + range.start) / 2.;
    if f(x_new) * f(range.start) >= 0. {
        range.start = x_new;
    } else {
        range.end = x_new;
    }
    // println!("{}, {}", times, x_new); // for plot |true value - approximate solution|
    data.push((times as f64, (x_new - expected_value).abs()));
    if range.end - range.start <= e {
        (x_new, data)
    } else {
        bisection_method_inner(range, e, f, times + 1, expected_value, data)
    }
}
// }
// definition for f64, TODO: make this generic

#[cfg(test)]
mod tests_bisection_method {
    use crate::bisection_method::*;

    #[test]
    fn tests_bisection_method() {
        let f = Rc::new(|x: f64| {
            x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.
        });
        assert_eq!(
            (bisection_method(-2f64..0f64, 1e-3, f.clone(), 1.414213566237)).0,
            -1.4150390625
        );
        assert_eq!(
            (bisection_method(-2f64..0f64, 1e-4, f.clone(), 1.414213566237)).0,
            -1.41424560546875
        );
        assert_eq!(
            (bisection_method(-2f64..0f64, 1e-5, f.clone(), 1.414213566237)).0,
            -1.4142074584960938
        );
    }
}
