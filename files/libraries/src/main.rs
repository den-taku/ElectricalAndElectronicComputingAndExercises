mod bisection_method;
mod matrix;
mod newton_raphson_method;

use gnuplot::*;
use std::rc::Rc;

fn main() {
    let f =
        Rc::new(|x: f64| x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.);

    let data_bisection = bisection_method::bisection_method(-2f64..0f64, 1e-4, f.clone(), 1.414213566237).1;
    let data_newton = newton_raphson_method::newton_raphson_method(f.clone(), -1.0, 1.414213566237).unwrap().1;

    let mut fg = Figure::new();
    {
        let axec = fg
            .axes2d()
            .set_x_axis(true, &[])
            .set_x_range(Fix(0.0), Fix(17.0))
            .set_y_range(Fix(1.0), Fix(10.0))
            .set_y_log(Some(10.0));

        data_bisection.iter().fold((), |_, e| {
            axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
        });

        data_newton.iter().fold((), |_, e| {
            axec.points(&[e.0], &[e.1], &[Color("red"), PointSymbol('O')]);
        });
    }
    let _ = fg.show();
}
