mod bisection_method;
mod matrix;
mod newton_raphson_method;

use gnuplot::*;
use std::rc::Rc;

fn main() {
    let f =
    Rc::new(|x: f64| x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.);

let data_bisection =
    bisection_method::bisection_method(0f64..1.2f64, 1e-4, f.clone(), 1.0).1;
let data_newton = newton_raphson_method::newton_raphson_method(f.clone(), 0.6, 1.0)
    .unwrap()
    .1;

let x_bisection: Vec<f64> = data_bisection.iter().map(|e| e.0).collect();
let y_bisection: Vec<f64> = data_bisection.iter().map(|e| e.1).collect();

let x_newton: Vec<f64> = data_newton.iter().map(|e| e.0).collect();
let y_newton: Vec<f64> = data_newton.iter().map(|e| e.1).collect();

let mut fg = Figure::new();
{
    let _axec = fg
        .axes2d()
        .set_x_axis(true, &[])
        .set_x_range(Fix(0.0), Fix(17.0))
        .set_y_range(Fix(0.1e-8), Fix(2.0))
        .set_y_log(Some(10.0))
        .set_x_label("times", &[])
        .set_y_label("error", &[])
        // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
        .lines(
            x_bisection,
            y_bisection,
            &[Caption("bisection_method"), Color("blue")],
        )
        .lines(
            x_newton,
            y_newton,
            &[Caption("newton_raphson_method"), Color("red")],
        );

    // data_bisection.iter().fold((), |_, e| {
    //     axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    // });

    // data_newton.iter().fold((), |_, e| {
    //     axec.points(&[e.0], &[e.1], &[Color("red"), PointSymbol('O')]);
    // });
}
let _ = fg.show(); 

    
}
