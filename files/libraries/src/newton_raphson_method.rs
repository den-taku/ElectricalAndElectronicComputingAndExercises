// pub mod newton_raphson_method {
pub use std::rc::Rc;
pub use std::result::Result;

// f must be declared as dyn Fn trait object.
pub fn newton_raphson_method(f: Rc<dyn Fn(f64) -> f64>, init: f64) -> Result<f64, f64> {
    let threshold = 0.1e-10;
    // let f = Rc::new(f);
    let f_dir = differential_f(f.clone()); // f is consumed here.
    if !f.is_converge() {
        return Err(f64::NEG_INFINITY);
    }
    // TODO: implement nrm using defferntail_f
    newton_method(newton_transform(f, f_dir), init, threshold, 1, 1000000)
}

fn differential_f(f: Rc<dyn Fn(f64) -> f64>) -> Rc<dyn Fn(f64) -> f64> {
    let dx = 0.1e-10;
    let f_dir = move |x: f64| -> f64 { (f(x + dx) - f(x)) / dx };
    Rc::new(f_dir)
}

fn newton_transform(
    f: Rc<dyn Fn(f64) -> f64>,
    f_dir: Rc<dyn Fn(f64) -> f64>,
) -> Rc<dyn Fn(f64) -> f64> {
    Rc::new(move |x: f64| -> f64 { x - f(x) / f_dir(x) })
}

fn newton_method(
    f: Rc<dyn Fn(f64) -> f64>,
    guess: f64,
    threshold: f64,
    times: usize,
    limit: usize,
) -> Result<f64, f64> {
    let next = f(guess);
    if limit == times + 1 {
        return Err(next);
    }
    if (next - guess).abs() <= threshold {
        Ok(next)
    } else {
        newton_method(f, next, threshold, times + 1, limit)
    }
}

trait CheckConverge {
    fn is_converge(&self) -> bool;
}

impl CheckConverge for Rc<dyn Fn(f64) -> f64> {
    fn is_converge(&self) -> bool {
        true // TODO: implement using differntail
    }
}
// }

#[cfg(test)]
mod tests_newton_raphson_method {
    use crate::newton_raphson_method::*;
    // #[test]
    // fn test_newton_raphson_method_differential_f() {
    //     let dummy_function = Rc::new(Box::new(|x: f64| -> f64 { x * x }));
    //     differential_f(dummy_function)(3.0);
    // }

    #[test]
    fn test_newton_raphson_method_newton_raphson_method() {
        let f: Rc<dyn Fn(f64) -> f64> = Rc::new(|x: f64| -> f64 {
            x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.
        });
        assert_eq!(newton_raphson_method(f, -1.), Ok(-1.4142135623730951));
    }
}
