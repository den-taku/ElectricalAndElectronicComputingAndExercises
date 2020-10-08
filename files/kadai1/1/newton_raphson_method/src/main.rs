mod newton_raphson_method {
    fn newton_raphson_method(f: Box<dyn Fn(f64) -> f64>) -> Option<f64> {
        if !f.is_converge() {
            return None;
        }
        // TODO: implement nrm using defferntail_f
        Some(0.0)
    }

    fn differential_f(f: Box<dyn Fn(f64) -> f64>) -> Box<dyn Fn(f64) -> f64> {
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

fn main() {
    println!("Hello, world!");
}
