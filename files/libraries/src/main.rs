mod bisection_method;
mod data;
mod draw;
mod euler;
mod heun;
mod matrix;
mod newton_raphson_method;
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
use runge_kutta4::*;
use std::f64::consts::PI;

fn main() {
    let x_0 = 2.0;
    let y_0 = 1.0;
    let gamma = 0.0;
    let h = 0.1;
    let t_0 = 0.0;

    let mut log = (Vec::new(), Vec::new());
    log.0.push((t_0, x_0));
    log.1.push((t_0, y_0));

    // let log = euler32(x_0, y_0, h, t_0, gamma, log);
    // let log = heun32(x_0, y_0, h, t_0, gamma, log);
    let log = runge432(x_0, y_0, h, t_0, gamma, log);

    let x_max = {
        let mut copy = log.0.clone();
        copy.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        println!("x_max: {:?}", &copy[copy.len() - 1]);
        copy.pop().unwrap().1
    };

    let y_max = {
        let mut copy = log.1.clone();
        copy.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        println!("y_max: {:?}", &copy[copy.len() - 1]);
        copy.pop().unwrap().1
    };

    let mut data = Vec::new();

    for i in 0..log.1.len() {
        data.push((log.0[i].1.clone(), log.1[i].1.clone()));
    }

    // draw_graph(0.0, 20.0, 0.0, max + 1.0, "time", "")
    let mut fg = Figure::new();
    {
        let axec = fg
            .axes2d()
            .set_x_axis(true, &[])
            .set_x_range(Fix(0.0), Fix(x_max + 0.2))
            .set_y_range(Fix(0.0), Fix(y_max + 0.2))
            // .set_y_range(Fix(0.00001), Fix(10e10))
            // .set_y_log(Some(10.0))
            .set_x_label("time", &[])
            .set_y_label("number", &[]);
        data.iter().fold((), |_, e| {
            axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
        });
        axec.points(
            &[300.0],
            &[300.0],
            &[
                Caption("Numerical solution"),
                Color("blue"),
                PointSymbol('O'),
            ],
        );
        axec.points(
            &[2.0 / 2.0],
            &[(3.0 * 2.0 - 2.0 * gamma) / 9.0 / 2.0],
            &[
                Caption("Equilibrium solution"),
                Color("red"),
                PointSymbol('x'),
            ],
        );
    }
    let _ = fg.show();
}
