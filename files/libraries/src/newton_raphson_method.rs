mod newton_raphson_method {
    pub fn newton_raphson_method(f: Box<dyn Fn(f64) -> f64>) -> Option<f64> {
        if !f.is_converge() {
            return None;
        }
        // TODO: implement nrm using defferntail_f
        Some(0.0)
    }

    pub fn differential_f(f: Box<dyn Fn(f64) -> f64>) -> Box<dyn Fn(f64) -> f64> {
        let dx = 0.1e-10;
        let f_ = move |x: f64| -> f64 { (f(x + dx) - f(x)) / dx };
        Box::new(f_)
    }

    trait CheckConverge {
        fn is_converge(&self) -> bool;
    }

    impl CheckConverge for Box<dyn Fn(f64) -> f64> {
        fn is_converge(&self) -> bool {
            false // TODO: implement using differntail
        }
    }
}

#[cfg(test)]
mod tests_newton_raphson_method {
    use crate::newton_raphson_method::newton_raphson_method::*;
    #[test]
    fn test_newton_raphson_method_differential_f() {
        let dummy_function = Box::new(| x: f64| -> f64 {x * x});
        differential_f(dummy_function)(3.0);
    }
}