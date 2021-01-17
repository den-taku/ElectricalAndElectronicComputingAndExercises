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
use std::f64::consts::PI;

fn main() {
    let init = vec![0f64; 100];
    let data = vec![(-0.001, init.clone()), (0.0, init)];

    let log = fixed_end_pulus(0.0, 5.0, data.clone());
}
