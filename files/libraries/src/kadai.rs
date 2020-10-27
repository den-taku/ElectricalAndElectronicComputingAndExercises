use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use crate::matrix::*;

use std::rc::Rc;

fn _kadai121(times: usize) {
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

fn plot() {
    let s1 =
        Plot::new(kadai123(0.1, 1000)).point_style(PointStyle::new().marker(PointMarker::Circle));
    let s2 =
        Plot::new(kadai123(1.0, 1000)).point_style(PointStyle::new().marker(PointMarker::Square));
    let s3 =
        Plot::new(kadai123(3.8, 1000)).point_style(PointStyle::new().marker(PointMarker::Cross));

    let s4 =
        Plot::new(kadai123(800.0, 1000)).point_style(PointStyle::new().marker(PointMarker::Cross));

    let f =
        Rc::new(|x: f64| x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.);

    let s5 = {
        let mut data =
            bisection_method::bisection_method(-2f64..0f64, 1e-4, f.clone(), 1.414213566237).1;
        data = data.iter().map(|e| (e.0, e.1.log10())).collect();
        Plot::new(data).point_style(PointStyle::new().marker(PointMarker::Circle))
    };
    // let s6 = Plot::new(bisection_method::bisection_method(-2f64..0f64, 1e-4, f.clone(), 1.414213566237))
    // .point_style(PointStyle::new().marker(PointMarker::Circle));

    let v1 = ContinuousView::new()
        .add(s1)
        .x_range(0., 1000.)
        .y_range(3.5, 4.)
        .x_label("times")
        .y_label("value");

    let v2 = ContinuousView::new()
        .add(s2)
        .x_range(0., 1000.)
        .y_range(3.5, 4.)
        .x_label("times")
        .y_label("value");

    let v3 = ContinuousView::new()
        .add(s3)
        .x_range(0., 1000.)
        .y_range(3.5, 4.)
        .x_label("times")
        .y_label("value");

    let v4 = ContinuousView::new()
        .add(s4)
        .x_range(0., 1000.)
        .y_range(3.5, 4.)
        .x_label("times")
        .y_label("value");

    let v5 = ContinuousView::new()
        .add(s5)
        .x_range(0., 20.)
        .y_range(-12., 10.)
        .x_label("times")
        .y_label("error size");

    println!(
        "{}",
        Page::single(&v1).dimensions(80, 10).to_text().unwrap()
    );
    println!(
        "{}",
        Page::single(&v2).dimensions(80, 10).to_text().unwrap()
    );
    println!(
        "{}",
        Page::single(&v3).dimensions(80, 10).to_text().unwrap()
    );
    println!(
        "{}",
        Page::single(&v4).dimensions(80, 10).to_text().unwrap()
    );

    println!(
        "{}",
        Page::single(&v5).dimensions(80, 10).to_text().unwrap()
    );
}

fn nokorikasu() {
    kadai121(10);
    kadai121(1000);

    kadai123(0.1, 10);
    kadai123(1.0, 10);
    kadai123(3.8, 10);

    let data = vec![
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];

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

    let lu = a.lu_decompose();
    println!("{}", lu.0);
    println!("{}", lu.1);
    println!("{}", &lu.0 * &lu.1);
    println!("{}", &lu.1 * &lu.0);
    println!("{:?}", lu);
    kadai123(1000);

    let n = Matrix::append(1, 2, vec![3.0, 4.0]);
    let n2: f32 = n.norm2();
    println!("test: {}", n2);
    let data1 = vec![
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];

    // We create our scatter plot from the data
    let s1: Plot = Plot::new(data1).point_style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#DD3355"),
    ); // and a custom colour

    // We can plot multiple data sets in the same view
    let data2 = vec![(-1.4, 2.5), (7.2, -0.3)];
    let s2: Plot = Plot::new(data2).point_style(
        PointStyle::new() // uses the default marker
            .colour("#35C788"),
    ); // and a different colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("scatter.svg").unwrap();

    let f: Rc<dyn Fn(f64) -> f64> = Rc::new(|x: f64| -> f64 {
        x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.
    });
    let _ = newton_raphson_method::newton_raphson_method(f.clone(), 0.6);
    let _ = bisection_method::bisection_method(0f64..1.2f64, 1e-10, f.clone());
    println!(
        "{}",
        if let Ok(v) = newton_raphson_method::newton_raphson_method(f.clone(), -1.) {
            v
        } else {
            0.0
        }
    );
    // This call induce stack overflow
    println!(
        "{}",
        if let Ok(v) =
            newton_raphson_method::newton_raphson_method::newton_raphson_method(f.clone(), 0.6)
        {
            v
        } else {
            0.0
        }
    );
    for i in (9972)..(10016) {
        println!("{}, {},", (i as f64) / 10000., f((i as f64) / 10000.));
    }
    if let Err(s) = newton_raphson_method::newton_raphson_method(f.clone(), 0.6) {
        println!("{}", s);
    };
    let a = Matrix::append(2, 2, vec![2.0, 3.0, 4.0, 5.0]);
    let b = Matrix::append(2, 1, vec![7.0, 13.0]);
    println!("{}", a);
    println!("{}", b);
    let c = Matrix::solve_eqn(&a, &b);
    println!("{}", c);

    let f1: Rc<dyn Fn(Vec<f64>) -> f64> =
        Rc::new(|x1: Vec<f64>| -> f64 { x1[0] * x1[0] + x1[1] - 5.0 });
    let f2: Rc<dyn Fn(Vec<f64>) -> f64> =
        Rc::new(|x2: Vec<f64>| -> f64 { x2[0] - x2[1] * x2[1] - 1.0 });

    let mut vec_f: Vec<Rc<dyn Fn(Vec<f64>) -> f64>> = Vec::new();
    vec_f.push(f1.clone());
    vec_f.push(f2.clone());

    println!(
        "{:?}",
        newton_raphson_method::jacobian_newton_raphson_method(vec_f, vec![2.0f64.sqrt(); 2])
            .unwrap()
    );
    //
}

fn plot_bis_new() {
    let f =
        Rc::new(|x: f64| x.powf(5.) - 3. * x.powf(4.) + x.powf(3.) + 5. * x.powf(2.) - 6. * x + 2.);

    let data_bisection =
        bisection_method::bisection_method(-2f64..0f64, 1e-4, f.clone(), -1.414213566237).1;
    let data_newton = newton_raphson_method::newton_raphson_method(f.clone(), -1.0, -1.414213566237)
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

fn newton() {
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
    let data1 = kadai123(0.1, 1000);
    let data2 = kadai123(1.0, 1000);
    let data3 = kadai123(3.0, 1000);
    let data4 = kadai123(4.0, 1000);
    let data5 = kadai123(300.0, 1000);

    let x1: Vec<f64> = data1.iter().map(|e| e.0).collect();
    let y1: Vec<f64> = data1.iter().map(|e| e.1).collect();

    let x2: Vec<f64> = data2.iter().map(|e| e.0).collect();
    let y2: Vec<f64> = data2.iter().map(|e| e.1).collect();

    let x3: Vec<f64> = data3.iter().map(|e| e.0).collect();
    let y3: Vec<f64> = data3.iter().map(|e| e.1).collect();

    let x4: Vec<f64> = data4.iter().map(|e| e.0).collect();
    let y4: Vec<f64> = data4.iter().map(|e| e.1).collect();

    let x5: Vec<f64> = data5.iter().map(|e| e.0).collect();
    let y5: Vec<f64> = data5.iter().map(|e| e.1).collect();

    let mut fg = Figure::new();
    {
        let _axec = fg
            .axes2d()
            .set_x_axis(true, &[])
            .set_x_range(Fix(0.0), Fix(1000.0))
            .set_y_range(Fix(3.5), Fix(4.0))
            .set_x_label("times", &[])
            .set_y_label("error", &[])
            // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
            .lines(
                x1,
                y1,
                &[Caption("init: 0.1"), Color("blue")],
            )
            .lines(
                x2,
                y2,
                &[Caption("init: 1.0"), Color("red")],
            )
            .lines(
                x3,
                y3,
                &[Caption("init: 3.0"), Color("yellow")],
            )
            .lines(
                x4,
                y4,
                &[Caption("init: 4.0"), Color("black")],
            )
            .lines(
                x5,
                y5,
                &[Caption("init: 300.0"), Color("purple")]
            );


    }
    let _ = fg.show();
}

fn last() {
    let f1: Rc<dyn Fn(Vec<f64>) -> f64> =
    Rc::new(|x1: Vec<f64>| -> f64 { x1[0] * x1[0] + x1[1] - 5.0 });
let f2: Rc<dyn Fn(Vec<f64>) -> f64> =
    Rc::new(|x2: Vec<f64>| -> f64 { x2[0] - x2[1] * x2[1] - 1.0 });
let mut vec_f: Vec<Rc<dyn Fn(Vec<f64>) -> f64>> = Vec::new();
vec_f.push(f1.clone());
vec_f.push(f2.clone());

let _data = newton_raphson_method::jacobian_newton_raphson_method(
    vec_f.clone(),
    vec![2.0f64.sqrt(); 2],
    vec![0f64, 0f64],
)
.unwrap();

let data2 = newton_raphson_method::jacobian_newton_raphson_method(
    vec_f,
    vec![2f64.sqrt(); 2],
    vec![0f64, 0f64],
)
.unwrap();

// let x: Vec<f64> = data2.iter().map(|e| e.0).collect();
// let y: Vec<f64> = data2
//     .iter()
//     .map(|e| (e.1[0].powf(2.0) + e.1[1].powf(2.0)).sqrt())
//     .collect();

// println!("{:?}", x);
// println!("{:?}", y);

let mut fg = Figure::new();
{
    let axec = fg
        .axes2d()
        .set_x_axis(true, &[])
        .set_x_range(Fix(1.95), Fix(2.1))
        .set_y_range(Fix(0.95), Fix(1.1))
        // .set_y_log(Some(10.0))
        .set_x_label("x1", &[])
        .set_y_label("x2", &[]);
        // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
        // .lines(x, y, &[Caption("newton"), Color("blue")]);

    data2.iter().fold((), |_, e| {
        axec.points(&[e.1[0]], &[e.1[1]], &[Color("blue"), PointSymbol('O')]);
    })
}
let _ = fg.show(); 
}