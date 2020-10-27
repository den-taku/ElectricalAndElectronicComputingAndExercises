#![allow(dead_code)]

// pub mod newton_raphson_method {
pub use crate::matrix::*;
pub use std::rc::Rc;
pub use std::result::Result;

pub fn jacobian_newton_raphson_method(
    vec_f: Vec<Rc<dyn Fn(Vec<f64>) -> f64>>,
    vec_init: Vec<f64>,
) -> Result<Vec<f64>, String> {
    let n = vec_f.len();
    if n != vec_init.len() {
        panic!("`jacobian_newton_raphson_method` needs vec_f and vec_init the same size.");
    }
    let threshold = 0.1e-10;
    let jacobian = dif_jacobi(vec_f.clone());
    let init = Matrix::append(n, 1, vec_init);
    let f_n: Matrix<Rc<dyn Fn(Vec<f64>) -> f64>> = Matrix::append(n, 1, vec_f);
    jacobian_newton_method(f_n, jacobian, init, threshold, 1, 10000000)
}

// f must be declared as dyn Fn trait object.
pub fn newton_raphson_method(
    f: Rc<dyn Fn(f64) -> f64>,
    init: f64,
    expected_value: f64,
) -> Result<(f64, Vec<(f64, f64)>), String> {
    let threshold = 0.1e-10;
    // let f = Rc::new(f);
    let f_dir = differential_f(f.clone()); // f is consumed here.
    if !f.is_convergence() {
        // TODO: check convergency in newton_method()
        return Err("function is not convergence.".to_string());
    }
    let data: Vec<(f64, f64)> = Vec::new();
    newton_method(
        newton_transform(f, f_dir),
        init,
        threshold,
        1,
        1000000,
        expected_value,
        data,
    )
}

fn dif_jacobi(vec_f: Vec<Rc<dyn Fn(Vec<f64>) -> f64>>) -> Matrix<Rc<dyn Fn(Vec<f64>) -> f64>> {
    Matrix::append_line({
        let mut v = Vec::new();
        for i in 0..vec_f.len() {
            let mut u = Vec::new();
            for j in 0..vec_f.len() {
                unsafe { u.push(partial_derivative(vec_f.index(i).clone(), j)) }
            }
            v.push(u);
        }
        v
    })
}

fn differential_f(f: Rc<dyn Fn(f64) -> f64>) -> Rc<dyn Fn(f64) -> f64> {
    let dx = 0.1e-10;
    let f_dir = move |x: f64| -> f64 { (f(x + dx) - f(x)) / dx };
    Rc::new(f_dir)
}

unsafe fn partial_derivative(
    f: Rc<dyn Fn(Vec<f64>) -> f64>,
    i: usize,
) -> Rc<dyn Fn(Vec<f64>) -> f64> {
    let dx = 0.1e-10;
    let f_der = move |v: Vec<f64>| -> f64 {
        let mut v_dx = v.clone();
        v_dx[i] += dx;
        (f(v_dx) - f(v)) / dx
    };
    Rc::new(f_der)
}

fn newton_transform(
    f: Rc<dyn Fn(f64) -> f64>,
    f_dir: Rc<dyn Fn(f64) -> f64>,
) -> Rc<dyn Fn(f64) -> f64> {
    Rc::new(move |x: f64| -> f64 { x - f(x) / f_dir(x) })
}

fn jacobian_newton_method(
    vec_f: Matrix<Rc<dyn Fn(Vec<f64>) -> f64>>,
    jacobian: Matrix<Rc<dyn Fn(Vec<f64>) -> f64>>,
    v_guess: Matrix<f64>,
    threshold: f64,
    times: usize,
    limit: usize,
) -> Result<Vec<f64>, String> {
    let x_k_ = vec![vec![v_guess.to_vec().clone(); v_guess.to_vec().len()]; v_guess.to_vec().len()];
    let x_k = x_k_.concat();
    let jacobian_applicated = jacobian.applicate(&x_k);
    let f_x_k = vec_f.applicate(&vec![v_guess.to_vec().clone(); v_guess.to_vec().len()]);
    let v_next = Matrix::solve_eqn(&jacobian_applicated, &f_x_k);
    // TODO: check value's value.
    if limit == times + 1 {
        return Err(format!(
            "solution doesn't converge: last value is {:?}.",
            v_next.to_vec()
        ));
    }
    let dx: f64 = v_next.norm2();
    println!("dx : {}", dx);
    if dx <= threshold {
        Ok((&v_guess - &v_next).to_vec())
    } else {
        println!("{}, {:?}", times, (&v_guess - &v_next).to_vec());
        jacobian_newton_method(
            vec_f,
            jacobian,
            &v_guess - &v_next,
            threshold,
            times + 1,
            limit,
        )
    }
}

fn newton_method(
    f: Rc<dyn Fn(f64) -> f64>,
    guess: f64,
    threshold: f64,
    times: usize,
    limit: usize,
    expected_value: f64,
    mut data: Vec<(f64, f64)>,
) -> Result<(f64, Vec<(f64, f64)>), String> {
    let next = f(guess);
    if next == f64::NEG_INFINITY || next == f64::INFINITY || next.is_nan() {
        return Err(format!("x^(k+1) is not a number: last value is {}.", guess));
    }
    if limit == times + 1 {
        return Err(format!(
            "solution doesn't converge: last value is {}.",
            next
        ));
    }
    data.push((times as f64, (next - expected_value).abs()));
    if (next - guess).abs() <= threshold {
        Ok((next, data))
    } else {
        newton_method(f, next, threshold, times + 1, limit, expected_value, data)
    }
}

trait CheckConvergence {
    fn is_convergence(&self) -> bool;
}

impl CheckConvergence for Rc<dyn Fn(f64) -> f64> {
    fn is_convergence(&self) -> bool {
        true // TODO: implement using differntail
    }
}
// }

#[cfg(test)]
mod tests_newton_raphson_method {
    use crate::newton_raphson_method::newton_method;
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
        assert_eq!(
            newton_raphson_method(f, -1., 1.414213566237).unwrap().0,
            -1.4142135623730951
        );
    }

    #[test]
    fn test_newton_raphson_method_newton_method_neg_inf() {
        let f: Rc<dyn Fn(f64) -> f64> = Rc::new(|x: f64| -> f64 { x });
        assert_eq!(
            newton_method(
                f,
                f64::NEG_INFINITY,
                0.1e-10,
                1,
                10000,
                1.41,
                vec![(0f64, 0f64)]
            ),
            Err("x^(k+1) is not a number: last value is -inf.".to_string())
        );
    }

    #[test]
    fn test_newton_raphson_method_newton_method_inf() {
        let f: Rc<dyn Fn(f64) -> f64> = Rc::new(|x: f64| -> f64 { x });
        assert_eq!(
            newton_method(
                f,
                f64::INFINITY,
                0.1e-10,
                1,
                10000,
                1.41,
                vec![(0f64, 0f64)]
            ),
            Err("x^(k+1) is not a number: last value is inf.".to_string())
        );
    }

    #[test]
    fn test_newton_raphson_method_newton_method_nan() {
        let f: Rc<dyn Fn(f64) -> f64> = Rc::new(|x: f64| -> f64 { x });
        assert_eq!(
            newton_method(f, f64::NAN, 0.1e-10, 1, 10000, 1.41, vec![(0f64, 0f64)]),
            Err("x^(k+1) is not a number: last value is NaN.".to_string())
        );
    }

    #[test]
    fn test_newton_raohson_jacobi_newton_raphson() {
        let f1: Rc<dyn Fn(Vec<f64>) -> f64> =
            Rc::new(|x1: Vec<f64>| -> f64 { x1[0] * x1[0] + x1[1] * x1[1] - 2.0 });
        let f2: Rc<dyn Fn(Vec<f64>) -> f64> =
            Rc::new(|x2: Vec<f64>| -> f64 { x2[0] - x2[1] * x2[1] });
        let mut vec_f: Vec<Rc<dyn Fn(Vec<f64>) -> f64>> = Vec::new();
        vec_f.push(f1.clone());
        vec_f.push(f2.clone());
        assert_eq!(
            jacobian_newton_raphson_method(vec_f, vec![2.0f64.sqrt(); 2]).unwrap(),
            vec![1.0f64, 1.0f64]
        );
        let f3: Rc<dyn Fn(Vec<f64>) -> f64> =
            Rc::new(|x1: Vec<f64>| -> f64 { x1[0] * x1[0] + x1[1] - 5.0 });
        let f4: Rc<dyn Fn(Vec<f64>) -> f64> =
            Rc::new(|x2: Vec<f64>| -> f64 { x2[0] - x2[1] * x2[1] - 1.0 });

        let mut vec_f: Vec<Rc<dyn Fn(Vec<f64>) -> f64>> = Vec::new();
        vec_f.push(f3.clone());
        vec_f.push(f4.clone());

        assert_eq!(
            jacobian_newton_raphson_method(vec_f, vec![2.0f64.sqrt(); 2]).unwrap(),
            vec![2.0f64, 1.0f64]
        );
    }

    #[test]
    #[should_panic(
        expected = "`jacobian_newton_raphson_method` needs vec_f and vec_init the same size."
    )]
    fn test_newton_raohson_jacobi_newton_raphson_panic() {
        let f1: Rc<dyn Fn(Vec<f64>) -> f64> =
            Rc::new(|x1: Vec<f64>| -> f64 { x1[0] * x1[0] + x1[1] * x1[1] - 2.0 });
        let f2: Rc<dyn Fn(Vec<f64>) -> f64> =
            Rc::new(|x2: Vec<f64>| -> f64 { x2[0] - x2[1] * x2[1] });
        let mut vec_f: Vec<Rc<dyn Fn(Vec<f64>) -> f64>> = Vec::new();
        vec_f.push(f1.clone());
        vec_f.push(f2.clone());
        let _ = jacobian_newton_raphson_method(vec_f, vec![2.0f64.sqrt()]);
    }
}
