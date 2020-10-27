mod bisection_method;
mod matrix;
mod newton_raphson_method;

use gnuplot::*;
use std::rc::Rc;

fn main() {
    

    let f =
        Rc::new(|x: f64| x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.);

    let f_ = newton_raphson_method::differential_f(f.clone());

    let mut t = vec![];
    for i in 0..100i32 {
        t.push(i as f64 * 0.1 - 5.0);
    }
    let mut fg = Figure::new();
    // axes2d()はmutable borrowを作るので後でshow()するには別スコープを作る必要がある
    {
        let axes = fg.axes2d();
        // x軸を表示
        axes.set_x_axis(true, &[]);
        // 表示範囲の指定
        axes.set_x_range(Fix(-5.0), Fix(0.0));
        axes.set_y_range(Fix(-500.0), Fix(20.0));
        // fのプロット
        axes.lines(t.iter(), t.iter().map(|&x| f(x)), &[Color("red")]);

        // ニュートン法
        let mut x = -1.0; // 初期値
        while f(x).abs() > 1e-10 { // f(x)が十分小さくなるまで続ける
            // 関数値のプロット
            axes.points(&[x], &[f(x)], &[Color("blue"), PointSymbol('O')]); 
            // 関数値に縦線を引く
            axes.lines(&[x, x], &[-500.0, 20.0], &[Color("blue")]);
            // 接線のプロット
            axes.lines(t.iter(), t.iter().map(|&p| f_(x)*(p-x)+f(x)), &[Color("black")]);
            // 値の更新
            x = x - f(x)/f_(x);
        }
        println!("solution: {}", x);
    }
    let _ = fg.show();
}
