mod bisection_method;
mod matrix;
mod newton_raphson_method;

use std::rc::Rc;

fn main() {
    let f: Rc<Box<dyn Fn(f64) -> f64>> = Rc::new(Box::new(|x: f64| -> f64 {
        x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.
    }));
    println!(
        "{}",
        if let Ok(v) =
            newton_raphson_method::newton_raphson_method::newton_raphson_method(f.clone(), -1.)
        {
            v
        } else {
            0.0
        }
    );
    // This call induce stack overflow
    // println!(
    //     "{}",
    //     if let Ok(v) =
    //         newton_raphson_method::newton_raphson_method::newton_raphson_method(f.clone(), 0.6)
    //     {
    //         v
    //     } else {
    //         0.0
    //     }
    // );
}
