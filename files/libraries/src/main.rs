mod bisection_method;
mod data;
mod draw;
mod euler;
mod heun;
mod matrix;
mod newton_raphson_method;

use gnuplot::*;
// use std::rc::Rc;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::{PointMarker, PointStyle, LineStyle, LineJoin};
// use plotlib::view::ContinuousView;
// use matrix::*;
use draw::*;
use euler::*;
use std::f64::consts::PI;

fn main() {
    let mut true_x = vec![];
    let mut true_y = vec![];
    for i in 0..101 {
        true_x.push(-f64::sin(2.0 * PI * i as f64 / 100.0));
        true_y.push(-f64::cos(2.0 * PI * i as f64 / 100.0));
    }

    let data: Vec<(f64, f64)> = Vec::new();
    let err: Vec<(f64, f64)> = Vec::new();
    let log = euler::<f64>(0.0, -1.0, PI / 32.0, 0.0, (data, err));
    // draw_graph(-4.0, 4.0, -4.0, 4.0, "x", "y", "blue", log.0.clone());
    // draw_graph(0.0, 5.0 * PI, 0.0, 4.0, "x", "y", "blue", log.1.clone());
    // println!("{:?}", &log.1);
    let mut fg = Figure::new();
    {
        let axec = fg
            .axes2d()
            .set_x_axis(true, &[])
            .set_x_range(Fix(-2.2), Fix(2.2))
            .set_y_range(Fix(-2.2), Fix(2.2))
            .set_x_label("v_x", &[])
            .set_y_label("v_y", &[])
            .lines(
                true_x,
                true_y,
                &[Caption("Analytical solution"), Color("red")],
            );
        log.0.iter().fold((), |_, e| {
            axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
        });
        axec.points(
            &[300.0],
            &[300.0],
            &[Caption("Euler"), Color("blue"), PointSymbol('O')],
        );
    }
    let _ = fg.show();
}
