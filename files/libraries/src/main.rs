mod bisection_method;
mod data;
mod draw;
mod euler;
mod heun;
mod matrix;
mod newton_raphson_method;
mod pulus;
mod runge_kutta4;

use gnuplot::*;
// use std::rc::Rc;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::{PointMarker, PointStyle, LineStyle, LineJoin};
// use plotlib::view::ContinuousView;
// use matrix::*;
use draw::*;
use euler::*;
use heun::*;
use pulus::*;
use runge_kutta4::*;
// use std::f64::consts::PI;

fn main() {
    let mut init = Vec::new();
    for i in 0..100 {
        init.push((i as f64 * 0.01, 0f64));
    }
    let data = vec![(-0.001, init.clone()), (0.0, init)];

    let log = fixed_end_pulus(0.0, 5.0, data.clone());

    let u = log.iter().map(|e| {
        e.1.iter().map(|v| {
            v.1
        }).collect::<Vec<f64>>()
    }).collect::<Vec<Vec<f64>>>();
    let u = u.concat();

    draw_graph(0.0, 0.1, 0.0, 2.0, "x", "u", "blue", log[3000].1.clone());

    // let mut fg = Figure::new();
    // fg.axes3d()
    //     .surface()


}
