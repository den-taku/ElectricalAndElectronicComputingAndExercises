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

    // let log = fixed_end_pulus::<f64>(0.0, 5.0, data.clone());
    // let log = freed_end_pulus::<f64>(0.0, 5.0, data.clone());
    // let log = fixed_end_pulus2::<f64>(0.0, 5.0, data.clone());
    let log = freed_end_pulus2::<f64>(0.0, 5.0, data.clone());

    // for i in 0..log.len() {
    //     println!("{} writing...", i);
    //     write_to_png(
    //         &format!("png/png4/free2_{:04}.png", i),
    //         0.0,
    //         1.0,
    //         -2.0,
    //         2.0,
    //         "x(t)",
    //         "u(t)",
    //         "blue",
    //         log[i].1.clone(),
    //     );
    // }

    draw_graph(0.0, 1.0, -2.0, 2.0, "x", "u", "blue", log[400].1.clone());

    // let u = log.iter().map(|e| {
    //     e.1.iter().map(|v| {
    //         v.1
    //     }).collect::<Vec<f64>>()
    // }).collect::<Vec<Vec<f64>>>();
    // let u = u.concat();

    // let x = {
    //     let mut v = vec![];
    //     for i in 0..5000 {
    //         for j in 0..100 {
    //             v.push(j as f64 * 1.0 / 100.0);
    //         }
    //     }
    //     v
    // };

    // let y = {
    //     let mut v = vec![];
    //     for i in 0..5000 {
    //         for j in 0..100 {
    //             v.push(i as f64 * 0.001)
    //         }
    //     }
    //     v
    // };

    // let mut fg = Figure::new();
    // {
    //     fg.axes3d()
    //         .points(x, y, u, &[PointSymbol('o'), Color("blue"), PointSize(2.0)])
    //         .set_title("Fix End Pulus", &[]);
    // }
    // fg.show();
}
