mod bisection_method;
mod matrix;
mod newton_raphson_method;

use std::rc::Rc;

fn main() {
    // let f: Rc<dyn Fn(f64) -> f64> = Rc::new(|x: f64| -> f64 {
    //     x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.
    // });
    // let _ = newton_raphson_method::newton_raphson_method(f.clone(), 0.6);
    // let _ = bisection_method::bisection_method(0f64..1.2f64, 1e-10, f.clone());
    // println!(
    //     "{}",
    //     if let Ok(v) = newton_raphson_method::newton_raphson_method(f.clone(), -1.) {
    //         v
    //     } else {
    //         0.0
    //     }
    // );
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
    // for i in (9972)..(10016) {
    //     println!("{}, {},", (i as f64) / 10000., f((i as f64) / 10000.));
    // }
    // if let Err(s) = newton_raphson_method::newton_raphson_method(f.clone(), 0.6) {
    //     println!("{}", s);
    // };
}
