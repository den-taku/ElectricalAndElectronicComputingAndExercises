mod bisection_method;
mod matrix;
mod newton_raphson_method;

fn main() {
    let f = Box::new(|x: f64| -> f64 {
        x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.
    });
    println!(
        "{}",
        if let Ok(v) = newton_raphson_method::newton_raphson_method::newton_raphson_method(f, -1.) {v} else {0.0}
    );
}
