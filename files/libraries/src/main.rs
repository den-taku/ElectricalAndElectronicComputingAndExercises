mod bisection_method;
mod matrix;
mod newton_raphson_method;

use gnuplot::*;
// use std::rc::Rc;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::{PointMarker, PointStyle, LineStyle, LineJoin};
// use plotlib::view::ContinuousView;

use crate::matrix::*;

fn kadai123(init: f32, times: usize) -> Vec<(f64, f64)> {
    let a = Matrix::append_line(vec![
        vec![2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![-1.0, 2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0, -1.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0, -1.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0, -1.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0],
    ]);

    let mut data = Vec::new();

    let mut f = |x: Matrix<f32>, i: usize| -> Matrix<f32> {
        let y = &a * &x;
        let y_norm = y.norm2();
        println!("M: {}, y_norm: {}", i, y_norm);
        data.push((i as f64, y_norm as f64));
        &y / y_norm
    };

    let mut x = Matrix::new(10, 1);
    x += init;

    for i in 0..times {
        x = f(x, i);
    }

    data
}

fn main() {
    let data = kadai123(1.0, 30);

    let mut fg = Figure::new();
    // axes2d()はmutable borrowを作るので後でshow()するには別スコープを作る必要がある
    {
        let axes = fg.axes2d();
        // x軸を表示
        axes.set_x_axis(true, &[]);
        // 表示範囲の指定
        axes.set_x_range(Fix(0.0), Fix(30.0));
        axes.set_y_range(Fix(3.5), Fix(3.7));

        data.iter().fold((), |_, e| {
            axes.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
        });
    }
    let _ = fg.show();
    // let s1 =
    //     Plot::new(kadai123(0.1, 1000)).point_style(PointStyle::new().marker(PointMarker::Circle));
    // let s2 =
    //     Plot::new(kadai123(1.0, 1000)).point_style(PointStyle::new().marker(PointMarker::Square));
    // let s3 =
    //     Plot::new(kadai123(3.8, 1000)).point_style(PointStyle::new().marker(PointMarker::Cross));

    // let s4 =
    //     Plot::new(kadai123(800.0, 1000)).point_style(PointStyle::new().marker(PointMarker::Cross));

    // let v1 = ContinuousView::new()
    //     .add(s1)
    //     .x_range(0., 1000.)
    //     .y_range(3.5, 4.)
    //     .x_label("times")
    //     .y_label("value");

    // let v2 = ContinuousView::new()
    //     .add(s2)
    //     .x_range(0., 1000.)
    //     .y_range(3.5, 4.)
    //     .x_label("times")
    //     .y_label("value");

    // let v3 = ContinuousView::new()
    //     .add(s3)
    //     .x_range(0., 1000.)
    //     .y_range(3.5, 4.)
    //     .x_label("times")
    //     .y_label("value");

    // let v4 = ContinuousView::new()
    //     .add(s4)
    //     .x_range(0., 1000.)
    //     .y_range(3.5, 4.)
    //     .x_label("times")
    //     .y_label("value");

    // println!(
    //     "{}",
    //     Page::single(&v1).dimensions(80, 10).to_text().unwrap()
    // );
    // println!(
    //     "{}",
    //     Page::single(&v2).dimensions(80, 10).to_text().unwrap()
    // );
    // println!(
    //     "{}",
    //     Page::single(&v3).dimensions(80, 10).to_text().unwrap()
    // );
    // println!(
    //     "{}",
    //     Page::single(&v4).dimensions(80, 10).to_text().unwrap()
    // );

    // let s0 =
    //     Plot::new(kadai123(1.0, 30)).point_style(PointStyle::new().marker(PointMarker::Circle));

    // let v0 = ContinuousView::new()
    //     .add(s0)
    //     .x_range(0., 30.)
    //     .y_range(3.5, 4.)
    //     .x_label("times")
    //     .y_label("value");

    // println!(
    //     "{}",
    //     Page::single(&v0).dimensions(80, 10).to_text().unwrap()
    // );

    // let l0 = Plot::new(kadai123(1.0, 30))
    //     .line_style(
    //         LineStyle::new()
    //             .colour("blue")
    //             .linejoin(LineJoin::Round)
    //     ).point_style(PointStyle::new());

    //     let v0 = ContinuousView::new().add(l0);

    //     Page::single(&v0)
    //         .save("kadai121.svg")
    //         .expect("saving svg");
}
