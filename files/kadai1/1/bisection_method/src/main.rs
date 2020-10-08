use std::ops::Range;

// definition for f64, TODO: make this generic
fn bisection_method<F>(mut range: Range<f64>, e: f64, f: F) -> f64
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

fn main() {
    let f = |x: f64| x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.;
    println!("f(x) = x^5 - 3 x^4 + x^3 + 5 x^2 - 6 x + 2 -> {}", bisection_method(-2f64..0f64, 1e-3, f));
    println!("f(x) = x^5 - 3 x^4 + x^3 + 5 x^2 - 6 x + 2 -> {}", bisection_method(-2f64..0f64, 1e-4, f));
    println!("f(x) = x^5 - 3 x^4 + x^3 + 5 x^2 - 6 x + 2 -> {}", bisection_method(-2f64..0f64, 1e-5, f));
}
