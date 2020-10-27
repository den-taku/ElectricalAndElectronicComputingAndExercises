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

    let data = newton_raphson_method::jacobian_newton_raphson_method(
        vec_f,
        vec![2.0f64.sqrt(); 2],
        vec![0f64, 0f64],
    )
    .unwrap();

    let mut fg = Figure::new();
    {
        let axes = fg.axes2d();
        // x軸を表示
        axes.set_x_axis(true, &[]);
        // 表示範囲の指定
        axes.set_x_range(Fix(0.9), Fix(1.2));
        axes.set_y_range(Fix(0.9), Fix(1.2));

        data.iter().fold((), |_, e| {
            axes.points(
                &[e.1[0].clone()],
                &[e.1[1].clone()],
                &[Color("blue"), PointSymbol('O')],
            );
        });
    }
    let _ = fg.show();
}
