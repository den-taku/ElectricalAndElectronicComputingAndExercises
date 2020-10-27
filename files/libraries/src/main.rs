mod bisection_method;
mod matrix;
mod newton_raphson_method;

use gnuplot::*;
use std::rc::Rc;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::{PointMarker, PointStyle, LineStyle, LineJoin};
// use plotlib::view::ContinuousView;

// use crate::matrix::*;

fn main() {
    let f1: Rc<dyn Fn(Vec<f64>) -> f64> =
        Rc::new(|x1: Vec<f64>| -> f64 { x1[0] * x1[0] + x1[1] * x1[1] - 2.0 });
    let f2: Rc<dyn Fn(Vec<f64>) -> f64> = Rc::new(|x2: Vec<f64>| -> f64 { x2[0] - x2[1] * x2[1] });
    let mut vec_f: Vec<Rc<dyn Fn(Vec<f64>) -> f64>> = Vec::new();
    vec_f.push(f1.clone());
    vec_f.push(f2.clone());

    let _data = newton_raphson_method::jacobian_newton_raphson_method(
        vec_f.clone(),
        vec![2.0f64.sqrt(); 2],
        vec![0f64, 0f64],
    )
    .unwrap();

    let data2 = newton_raphson_method::jacobian_newton_raphson_method(
        vec_f, 
        vec![2f64.sqrt(); 2],
        vec![1f64;2]
    ).unwrap();


    let x: Vec<f64> = data2.iter().map(|e| e.0).collect();
    let y: Vec<f64> = data2.iter().map(|e| (e.1[0].powf(2.0) + e.1[1].powf(2.0)).sqrt()).collect();

    println!("{:?}", x);
    println!("{:?}", y);

    let mut fg = Figure::new();
    {
        let _axec = fg
        .axes2d()
        .set_x_axis(true, &[])
        .set_x_range(Fix(0.0), Fix(5.0))
        .set_y_range(Fix(1.0e-12), Fix(2.0))
        .set_y_log(Some(10.0))
        .set_x_label("times", &[])
        .set_y_label("error", &[])
        // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
        .lines(
            x,
            y,
            &[Caption("bisection_method"), Color("blue")],
        );
    }
    let _ = fg.show();
}
