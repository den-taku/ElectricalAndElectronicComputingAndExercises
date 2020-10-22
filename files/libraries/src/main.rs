mod bisection_method;
mod matrix;
mod newton_raphson_method;

// use std::rc::Rc;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::{PointMarker, PointStyle};
// use plotlib::view::ContinuousView;
use matrix::Matrix;

fn kadai121(times: usize) {
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

    let f = |x: Matrix<f32>, i: usize| -> Matrix<f32> {
        let y = &a * &x;
        let y_norm = y.norm2();
        println!("M: {}, y_norm: {}", i, y_norm);
        &y / y_norm
    };

    let mut x = Matrix::new(10, 1);
    x += 1.0;

    for i in 0..times {
        x = f(x, i);
    }
}

fn kadai123(times: usize) {
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

    let f = |x: Matrix<f32>, i: usize| -> Matrix<f32> {
        let y = &a * &x;
        let y_norm = y.norm2();
        println!("M: {}, y_norm: {}", i, y_norm);
        &y / y_norm
    };

    let mut x = Matrix::new(10, 1);
    x += 3.8;

    for i in 0..times {
        x = f(x, i);
    }
}

fn main() {

    kadai121(10);
    kadai121(1000);

    kadai123(10);
    // kadai123(1000);
    
    // let n = Matrix::append(1, 2, vec![3.0, 4.0]);
    // let n2: f32 = n.norm2();
    // println!("test: {}", n2);
    // let data1 = vec![
    //     (-3.0, 2.3),
    //     (-1.6, 5.3),
    //     (0.3, 0.7),
    //     (4.3, -1.4),
    //     (6.4, 4.3),
    //     (8.5, 3.7),
    // ];

    // // We create our scatter plot from the data
    // let s1: Plot = Plot::new(data1).point_style(
    //     PointStyle::new()
    //         .marker(PointMarker::Square) // setting the marker to be a square
    //         .colour("#DD3355"),
    // ); // and a custom colour

    // // We can plot multiple data sets in the same view
    // let data2 = vec![(-1.4, 2.5), (7.2, -0.3)];
    // let s2: Plot = Plot::new(data2).point_style(
    //     PointStyle::new() // uses the default marker
    //         .colour("#35C788"),
    // ); // and a different colour

    // // The 'view' describes what set of data is drawn
    // let v = ContinuousView::new()
    //     .add(s1)
    //     .add(s2)
    //     .x_range(-5., 10.)
    //     .y_range(-2., 6.)
    //     .x_label("Some varying variable")
    //     .y_label("The response of something");

    // // A page with a single view is then saved to an SVG file
    // Page::single(&v).save("scatter.svg").unwrap();

    // let f: Rc<dyn Fn(f64) -> f64> = Rc::new(|x: f64| -> f64 {
    //     x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.
    // });
    // let _ = newton_raphson_method::newton_raphson_method(f.clone(), 0.6);
    // let _ = bisection_method::bisection_method(0f64..1.2f64, 1e-10, f.clone());
    // println!(
    //     "{}",
    //     if let Ok(v) = newton_raphson_method::newton_raphson_method(f.clone(), -1.) {
    //         v
    //     } else {
    //         0.0
    //     }
    // );
    // This call induce stack overflow
    // println!(
    //     "{}",
    //     if let Ok(v) =
    //         newton_raphson_method::newton_raphson_method::newton_raphson_method(f.clone(), 0.6)
    //     {
    //         v
    //     } else {
    //         0.0
    //     }
    // );
    // for i in (9972)..(10016) {
    //     println!("{}, {},", (i as f64) / 10000., f((i as f64) / 10000.));
    // }
    // if let Err(s) = newton_raphson_method::newton_raphson_method(f.clone(), 0.6) {
    //     println!("{}", s);
    // };
}
