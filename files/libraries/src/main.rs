mod approximate;
mod bisection_method;
mod data;
mod draw;
mod matrix;
mod newton_raphson_method;

use gnuplot::*;
// use std::rc::Rc;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::{PointMarker, PointStyle, LineStyle, LineJoin};
// use plotlib::view::ContinuousView;
// use matrix::*;
use approximate::*;
use draw::*;
use std::f64::consts::PI;

fn main() {
    let mut true_x = vec![];
    let mut true_y = vec![];
    for i in 0..101 {
        true_x.push(-f64::sin(2.0 * PI * i as f64 / 100.0));
        true_y.push(-f64::cos(2.0 * PI * i as f64 / 100.0));
    }

    

    let mut errs = Vec::new();

    let mut now = 0.0;
    let tau = 2.0 * PI;
    for i in 3..19 {
        println!("p = {}", i);
        let h = tau * 2f64.powf(- i as f64);
        let max = euler2::<f64>(0.0, -1.0, h, 0.0, 0.0);
        // err.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        errs.push((i as f64, max));
    }
    let errs: Vec<(f64, f64)> = errs.iter().map(|e| {
        (e.0, f64::log2(e.1))
    }).collect();
    // let log = (data, err);
    // log.1.iter().fold((), |_, e| {
    //     println!("when {}, {}", e.0, e.1);
    // });

    // let log = euler::<f64>(0.0, -1.0, PI / 32.0, 0.0, (data, err));

    draw_graph(2.9, 18.1, 0.0, 1.6, "x", "log_2E_r", "blue", errs);
    // draw_graph(0.0, 1.7, 0.0, 1.6, "x", "log_2E_r", "blue", log.1.clone());
    // draw_graph(-4.0, 4.0, -4.0, 4.0, "x", "y", "blue", log.0.clone());
    // println!("{:?}", &log.1);
    // let mut fg = Figure::new();
    // {
    //     let axec = fg
    //         .axes2d()
    //         .set_x_axis(true, &[])
    //         .set_x_range(Fix(-2.2), Fix(2.2))
    //         .set_y_range(Fix(-2.2), Fix(2.2))
    //         .set_x_label("v_x", &[])
    //         .set_y_label("v_y", &[])
    //         .lines(
    //             true_x,
    //             true_y,
    //             &[Caption("Analytical solution"), Color("red")],
    //         );
    //     log.0.iter().fold((), |_, e| {
    //         axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //     });
    //     axec.points(
    //         &[300.0],
    //         &[300.0],
    //         &[Caption("Euler"), Color("blue"), PointSymbol('O')],
    //     );
    // }
    // let _ = fg.show();
}
