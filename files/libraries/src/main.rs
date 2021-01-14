mod approximate;
mod bisection_method;
mod data;
mod draw;
mod matrix;
mod newton_raphson_method;

// use gnuplot::*;
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
    let data: Vec<(f64, f64)> = Vec::new();
    let err: Vec<(f64, f64)> = Vec::new();
    let log = euler::<f64>(0.0, -1.0, PI / 32.0, 0.0, (data, err));
    println!("{:?}", &log.0);
    println!("");
    println!("{:?}", &log.1);
    draw_graph(-4.0, 4.0, -4.0, 4.0, "x", "y", "blue", log.0.clone());
}
