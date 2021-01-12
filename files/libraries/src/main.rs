mod bisection_method;
mod data;
mod matrix;
mod newton_raphson_method;

// use gnuplot::*;
// use std::rc::Rc;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::{PointMarker, PointStyle, LineStyle, LineJoin};
// use plotlib::view::ContinuousView;
use matrix::*;

// use crate::matrix::*;

fn main() {
    let matrixa = data::matrix21a_f64();
    let matrixb = data::matrix21b_f64();
    // println!("{}", matrixa);
    // println!("{}", matrixb);
    // let ab = Matrix::forward_erase(&matrixa, &matrixb);
    // println!("{}", ab.clone());
    // println!("{}", Matrix::backward_erase(ab));
    // println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));
    // println!("{}", Matrix::solve_eqn(&matrixa, &matrixb));
    // println!("{}", matrixa.lower_triangular_matrix());
    // println!("{}", matrixa.upper_triangular_matrix());
    // println!("{}", matrixa.diagonal_matrix());

    // let mut jacobi = Jacobi::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_jacobi = jacobi.solve(10e-10, 10_000);
    // // println!("times: {}", times_jacobi);
    // println!("{}", jacobi);
    // println!("{:?}", data_jacobi.pop().unwrap());
    //  println!("{:?}", data_jacobi);

    //  let mut fg = Figure::new();
    //  {
    //      let axec = fg
    //          .axes2d()
    //          .set_x_axis(true, &[])
    //          .set_x_range(Fix(0.0), Fix(4500.0))
    //          .set_y_range(Fix(1.0e-10), Fix(1.0))
    //          .set_y_log(Some(10.0))
    //          .set_x_label("times", &[])
    //          .set_y_label("residual\\_norm", &[]);
    //      // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //      // .lines(x, y, &[Caption("residual_norm"), Color("blue")]);

    //      data_jacobi.iter().fold((), |_, e| {
    //          axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //      })
    //  }
    //  let _ = fg.show();

    // let mut gauss_seidel =
    //     GaussSeidel::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_gauss_seidel = gauss_seidel.solve(10e-10, 10_000);
    // // println!("times: {}", times_gaussseidel);
    // println!("{}", gauss_seidel);
    // println!("{:?}", data_gauss_seidel.pop().unwrap());

    //  let mut fg = Figure::new();
    //  {
    //      let axec = fg
    //          .axes2d()
    //          .set_x_axis(true, &[])
    //          .set_x_range(Fix(0.0), Fix(2000.0))
    //          .set_y_range(Fix(1.0e-10), Fix(1.0))
    //          .set_y_log(Some(10.0))
    //          .set_x_label("times", &[])
    //          .set_y_label("residual\\_norm", &[]);
    //      // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //      // .lines(x, y, &[Caption("residual_norm"), Color("blue")]);

    //      data_gauss_seidel.iter().fold((), |_, e| {
    //          axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //      })
    //  }
    //  let _ = fg.show();

    // let mut sor = SOR::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.863);
    // let mut data_sor = sor.solve(10e-10, 10_000);
    // // println!("times: {}", times_sor);
    // println!("{}", sor);
    // println!("{:?}", data_sor.pop().unwrap());

    // println!(
    //     "Jacobi:       {}",
    //     (&jacobi.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f64>()
    // );
    // println!(
    //     "Gauss-Seidek: {}",
    //     (&gauss_seidel.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f64>()
    // );
    // println!(
    //     "SOR:          {}",
    //     (&sor.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f64>()
    // );

    // println!("");

    //     let mut fg = Figure::new();

    //     let x_jacobi: Vec<f64> = data_jacobi.iter().map(|e| e.0).collect();
    //     let y_jacobi: Vec<f64> = data_jacobi.iter().map(|e| e.1).collect();
    //     let x_gause_seidel: Vec<f64> = data_gauss_seidel.iter().map(|e| e.0).collect();
    //     let y_gause_seidel: Vec<f64> = data_gauss_seidel.iter().map(|e| e.1).collect();
    //     let x_sor: Vec<f64> = data_sor.iter().map(|e| e.0).collect();
    //     let y_sor: Vec<f64> = data_sor.iter().map(|e| e.1).collect();
    //     {
    //         let _axec = fg
    //             .axes2d()
    //             .set_x_axis(true, &[])
    //             .set_x_range(Fix(0.0), Fix(4500.0))
    //             .set_y_range(Fix(10e-10), Fix(1.0))
    //             .set_y_log(Some(10.0))
    //             .set_x_label("times", &[])
    //             .set_y_label("residual\\_norm", &[])
    //         // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //         .lines(x_jacobi, y_jacobi, &[Caption("Jacobi"), Color("green")])
    //         .lines(x_gause_seidel, y_gause_seidel, &[Caption("GaussSeidel"), Color("red")])
    //         .lines(x_sor, y_sor, &[Caption("SOR"), Color("blue")]);

    //       //   data_sor.iter().fold((), |_, e| {
    //       //       axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //       //   });
    //       //   data_gauss_seidel.iter().fold((), |_, e| {
    //       //    axec.points(&[e.0], &[e.1], &[Color("red"), PointSymbol('O')]);
    //       //   });
    //       //   data_jacobi.iter().fold((), |_, e| {
    //       //    axec.points(&[e.0], &[e.1], &[Color("green"), PointSymbol('O')]);
    // //   });
    //     }
    //     let _ = fg.show();

    // println!("ZZZZZZZZZZZZ f32 ZZZZZZZZZZZZZzz");

    // let matrixa = data::matrix21a_f32();
    // let matrixb = data::matrix21b_f32();
    // println!("{}", matrixa);
    // println!("{}", matrixb);
    // let ab = Matrix::forward_erase(&matrixa, &matrixb);
    // println!("{}", ab.clone());
    // println!("{}", Matrix::backward_erase(ab));
    // println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));
    // println!("{}", Matrix::solve_eqn(&matrixa, &matrixb));
    // println!("{}", matrixa.lower_triangular_matrix());
    // println!("{}", matrixa.upper_triangular_matrix());
    // println!("{}", matrixa.diagonal_matrix());

    // println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));

    // println!("{}", Matrix::solve_eqn(&matrixa, &matrixb));

    // let mut jacobi = Jacobi::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_jacobi = jacobi.solve(10e-7, 10_000);
    // // println!("times: {}", times_jacobi);
    // println!("{}", jacobi);
    // println!("{:?}", data_jacobi.pop().unwrap());

    // let mut gauss_seidel =
    //     GaussSeidel::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_gaussseidel = gauss_seidel.solve(10e-7, 10_000);
    // // println!("times: {}", times_gaussseidel);
    // println!("{}", gauss_seidel);
    // println!("{:?}", data_gaussseidel.pop().unwrap());

    // println!("KOKO?");
    // let mut sor = SOR::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.84);
    // let mut data_sor = sor.solve(10e-7, 10_000);
    // // println!("times: {}", times_sor);
    // println!("{}", sor);
    // println!("{:?}", data_sor.pop().unwrap());

    // println!(
    //     "Jacobi:       {}",
    //     (&jacobi.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f32>()
    // );
    // println!(
    //     "Gauss-Seidek: {}",
    //     (&gauss_seidel.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f32>()
    // );
    // println!(
    //     "SOR:          {}",
    //     (&sor.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f32>()
    // );
    //     let mut fg = Figure::new();

    //     let x_jacobi: Vec<f32> = data_jacobi.iter().map(|e| e.0).collect();
    //     let y_jacobi: Vec<f32> = data_jacobi.iter().map(|e| e.1).collect();
    //     let x_gause_seidel: Vec<f32> = data_gaussseidel.iter().map(|e| e.0).collect();
    //     let y_gause_seidel: Vec<f32> = data_gaussseidel.iter().map(|e| e.1).collect();
    //     let x_sor: Vec<f32> = data_sor.iter().map(|e| e.0).collect();
    //     let y_sor: Vec<f32> = data_sor.iter().map(|e| e.1).collect();
    //     {
    //         let _axec = fg
    //             .axes2d()
    //             .set_x_axis(true, &[])
    //             .set_x_range(Fix(0.0), Fix(2200.0))
    //             .set_y_range(Fix(10e-10), Fix(1.0))
    //             .set_y_log(Some(10.0))
    //             .set_x_label("times", &[])
    //             .set_y_label("residual\\_norm", &[])
    //         // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //         .lines(x_jacobi, y_jacobi, &[Caption("Jacobi"), Color("green")])
    //         .lines(x_gause_seidel, y_gause_seidel, &[Caption("GaussSeidel"), Color("red")])
    //         .lines(x_sor, y_sor, &[Caption("SOR"), Color("blue")]);

    //       //   data_sor.iter().fold((), |_, e| {
    //       //       axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //       //   });
    //       //   data_gauss_seidel.iter().fold((), |_, e| {
    //       //    axec.points(&[e.0], &[e.1], &[Color("red"), PointSymbol('O')]);
    //       //   });
    //       //   data_jacobi.iter().fold((), |_, e| {
    //       //    axec.points(&[e.0], &[e.1], &[Color("green"), PointSymbol('O')]);
    // //   });
    //     }
    //     let _ = fg.show();

    //  let mut data = Vec::new();
    // for i in 1..200 {
    //    let mut sor = SOR::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), i as f32 / 100.0);
    //    let mut data_sor = sor.solve(10e-7, 10_000);
    //    let times = data_sor.pop().unwrap().0;
    //    data.push((i as f64 / 100.0,  times));
    //    println!("ω: {}, times: {}", i as f64 / 100.0, times);
    //    //
    // }

    //  let mut fg = Figure::new();
    //  {
    //      let axec = fg
    //          .axes2d()
    //          .set_x_axis(true, &[])
    //          .set_x_range(Fix(0.0), Fix(2.0))
    //          .set_y_range(Fix(0.0), Fix(10000.0))
    //          // .set_y_log(Some(10.0))
    //          .set_x_label("relaxation\\_parameter", &[])
    //          .set_y_label("times", &[]);
    //      // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //      // .lines(x, y, &[Caption("residual_norm"), Color("blue")]);

    //      data.iter().fold((), |_, e| {
    //          axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //      })
    //  }
    //  let _ = fg.show();

    //  let matrixa = data::matrix23a_f64();
    //  let matrixb = data::matrix23b_f64();
    //  println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));

    // let matrixa = data::matrix21a_f64();
    // let matrixb = data::matrix21b_f64();

    // let mut data = Vec::new();
    for i in 1..2000 {
       let mut sor = SOR::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), i as f64 / 1000.0);
       let omega = i as f64 / 1000.0;
       let times = sor.solve(10e-10, 10_000);
       data.push((omega, times));
       println!("ω: {}, times: {}", omega, times);
    }
    let mut fg = Figure::new();
    {
        let axec = fg
            .axes2d()
            .set_x_axis(true, &[])
            .set_x_range(Fix(0.0), Fix(2.0))
            .set_y_range(Fix(0.0), Fix(10000.0))
            // .set_y_log(Some(10.0))
            .set_x_label("ω", &[])
            .set_y_label("times", &[]);
            // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
            // .lines(x, y, &[Caption("newton"), Color("blue")]);

        data.iter().fold((), |_, e| {
            axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
        })
    }
    let _ = fg.show();

    // let matrixa = data::matrix23a_f64();
    // let matrixb = data::matrix23b_f64();
    // println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));
    // println!("{}", Matrix::solve_eqn(&matrixa, &matrixb));

    // let mut jacobi = Jacobi::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_jacobi = jacobi.solve(10e-10, 10_000);
    // println!("{}", jacobi);
    // println!("{:?}", data_jacobi.pop().unwrap());

    // let mut gauss_seidel =
    //     GaussSeidel::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_gauss_seidel = gauss_seidel.solve(10e-10, 10_000);
    // println!("{}", gauss_seidel);
    // println!("{:?}", data_gauss_seidel.pop().unwrap());

    // let mut sor = SOR::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.8);
    // let mut data_sor = sor.solve(10e-10, 10_000);
    // println!("{}", sor);
    // println!("{:?}", data_sor.pop().unwrap());

    //  let mut fg = Figure::new();

    //  let x_jacobi: Vec<f64> = data_jacobi.iter().map(|e| e.0).collect();
    //  let y_jacobi: Vec<f64> = data_jacobi.iter().map(|e| e.1).collect();
    //  let x_gause_seidel: Vec<f64> = data_gauss_seidel.iter().map(|e| e.0).collect();
    //  let y_gause_seidel: Vec<f64> = data_gauss_seidel.iter().map(|e| e.1).collect();
    //  let x_sor: Vec<f64> = data_sor.iter().map(|e| e.0).collect();
    //  let y_sor: Vec<f64> = data_sor.iter().map(|e| e.1).collect();
    //  {
    //      let _axec = fg
    //          .axes2d()
    //          .set_x_axis(true, &[])
    //          .set_x_range(Fix(0.0), Fix(10000.0))
    //          .set_y_range(Fix(20e-6), Fix(4000000.0))
    //          .set_y_log(Some(10.0))
    //          .set_x_label("times", &[])
    //          .set_y_label("residual\\_norm", &[])
    //          .lines(x_jacobi, y_jacobi, &[Caption("Jacobi"), Color("green")])
    //          .lines(
    //              x_gause_seidel,
    //              y_gause_seidel,
    //              &[Caption("GaussSeidel"), Color("red")],
    //          )
    //          .lines(x_sor, y_sor, &[Caption("SOR"), Color("blue")]);
    //  }
    //  let _ = fg.show();

    // println!(
    //     "{}",
    //     (&(&matrixa.diagonal_matrix_inverse()
    //         * &(&matrixa.lower_triangular_matrix() + &matrixa.upper_triangular_matrix()))
    //         * (-1.0))
    //         .power_method(10000)
    //         .pop()
    //         .unwrap()
    //         .1
    // );

    let matrixa = data::matrix21a_f64();
    let matrixb = data::matrix21b_f64();
    let mut cg = CG::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    let mut data_cg = cg.solve(10e-10, 10_000);
    println!("{}", cg);
    println!("{:?}", data_cg.pop().unwrap());
    // println!("{}", matrixa.to_transpose());
    // let p0 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         1.1976790319381074,
    //         0.6273556833961516,
    //         0.7414203531045427,
    //         0.34219400912517356,
    //         1.0265820273755206,
    //         0.7984526879587384,
    //         0.45625867883356475,
    //         1.42580837135489,
    //         1.0265820273755206,
    //     ],
    // );
    // let p1 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         1.1239633353811769,
    //         0.7333152276550705,
    //         0.6698437722024668,
    //         0.37598994163348876,
    //         1.1279698249004662,
    //         0.6669082635265703,
    //         0.6941213885987522,
    //         1.5106243308748601,
    //         1.012768948698846,
    //     ],
    // );
    // let p2 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         1.109299282154137,
    //         0.7758355078662453,
    //         0.6546165280016074,
    //         0.35210031799115715,
    //         1.1578492770423459,
    //         0.6759281675067013,
    //         0.694852741205554,
    //         1.4695685915700991,
    //         1.0476937869624943,
    //     ],
    // );
    // let p3 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         1.1043911573254652,
    //         0.7793635555241604,
    //         0.6621923868098872,
    //         0.351521151632305,
    //         1.2018946538098345,
    //         0.6830606584369792,
    //         0.6915049325783603,
    //         1.4460050456230744,
    //         1.0320500615690578,
    //     ],
    // );
    // let p4 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         1.0990605362514667,
    //         0.8174095864190228,
    //         0.6424052657640149,
    //         0.3746339811530472,
    //         1.2330747110926898,
    //         0.7081545539919184,
    //         0.6690217834006685,
    //         1.437490540944326,
    //         0.9939318443210646,
    //     ],
    // );
    // let p5 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         1.0997071267786025,
    //         0.8199530643614047,
    //         0.6444800027709047,
    //         0.3739457415819264,
    //         1.2314083622096672,
    //         0.7113260382311992,
    //         0.6718484918750219,
    //         1.4356318677864865,
    //         0.9909052534842518,
    //     ],
    // );
    // let p6 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         1.1023061235273899,
    //         0.8208131418981773,
    //         0.6459348020928561,
    //         0.38659228471586093,
    //         1.2263341759912314,
    //         0.723236418901717,
    //         0.6840974639617289,
    //         1.4235064665940833,
    //         0.9893506707939286,
    //     ],
    // );
    // let p7 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         1.1086726197534653,
    //         0.8378618500417478,
    //         0.6716432961391207,
    //         0.4387478729269256,
    //         1.2066109216161214,
    //         0.7406657119597907,
    //         0.7123109984802011,
    //         1.3815510439987146,
    //         0.9876354900608108,
    //     ],
    // );
    // let p8 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         0.9999999999999895,
    //         0.9999999999999929,
    //         0.9999999999999889,
    //         0.9999999999999944,
    //         1.000000000000011,
    //         0.9999999999999745,
    //         1.0000000000000175,
    //         1.000000000000013,
    //         0.9999999999999826,
    //     ],
    // );
    // let p9 = Matrix::append(
    //     9,
    //     1,
    //     vec![
    //         0.9999999999999895,
    //         0.9999999999999929,
    //         0.9999999999999889,
    //         0.9999999999999944,
    //         1.000000000000011,
    //         0.9999999999999745,
    //         1.0000000000000175,
    //         1.000000000000013,
    //         0.9999999999999826,
    //     ],
    // );
    // println!("{}", (&p0.to_transpose() * &(&matrixa * &p1)).to_value());
    // println!("{}", (&p1.to_transpose() * &(&matrixa * &p2)).to_value());
    // println!("{}", (&p2.to_transpose() * &(&matrixa * &p3)).to_value());
    // println!("{}", (&p3.to_transpose() * &(&matrixa * &p4)).to_value());
    // println!("{}", (&p4.to_transpose() * &(&matrixa * &p5)).to_value());
    // println!("{}", (&p5.to_transpose() * &(&matrixa * &p6)).to_value());
    // println!("{}", (&p6.to_transpose() * &(&matrixa * &p7)).to_value());
    // println!("{}", (&p6.to_transpose() * &(&matrixa * &p8)).to_value());
    // println!("{}", (&p6.to_transpose() * &(&matrixa * &p9)).to_value());
    let matrixa = data::matrix21a_f32();
    let matrixb = data::matrix21b_f32();
    let mut cg = CG::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    let mut data_cg = cg.solve(10e-6, 10_000);
    println!("{}", cg);
    println!("{:?}", data_cg.pop().unwrap());

    let matrixa = data::matrix23a_f64();
    let matrixb = data::matrix23b_f64();
    let mut cg = CG::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    let mut data_cg = cg.solve(10e-10, 10_000);
    println!("{}", cg);
    println!("{:?}", data_cg.pop().unwrap());
    println!("{}", &matrixa);

    let matrix21a = data::matrix21a_f64();
    let matrix23a = data::matrix23a_f64();

    let mut qr21 = QR::new(matrix21a.clone());
    let mut qr23 = QR::new(matrix23a.clone());

    let mut data_21 = qr21.solve(10e-10, 10_000);
    let mut data_23 = qr23.solve(10e-10, 10_000);

    println!("{}", data_21.pop().unwrap().0);
    println!("{}", qr21);
    println!("{}", data_23.pop().unwrap().0);
    println!("{}", qr23);

    // let mut fg = Figure::new();

    // let x_21: Vec<f64> = data_21.iter().map(|e| e.0).collect();
    // let y_21: Vec<f64> = data_21.iter().map(|e| e.1).collect();
    // let x_23: Vec<f64> = data_23.iter().map(|e| e.0).collect();
    // let y_23: Vec<f64> = data_23.iter().map(|e| e.1).collect();
    // {
    //     let _axec = fg
    //         .axes2d()
    //         .set_x_axis(true, &[])
    //         .set_x_range(Fix(0.0), Fix(130.0))
    //         .set_y_range(Fix(10e-10), Fix(10.0))
    //         .set_y_log(Some(10.0))
    //         .set_x_label("times", &[])
    //         .set_y_label("norm", &[])
    //         .lines(x_21, y_21, &[Caption("課題2.1"), Color("green")])
    //         .lines(x_23, y_23, &[Caption("課題2.3"), Color("red")]);
    // }
    // let _ = fg.show();

    // println!("{:?}", matrixa.to_vec_line(3));
    // println!("{}", matrixa.to_matrix_line(3));
    // println!("{:?}", matrixa.to_vec_culumn(3));
    // println!("{}", matrixa.to_matrix_culumn(3));
    // println!("{}", matrixa.gram_schmidt());
    // println!("{}", matrixa.qr_decompose().0);
    // println!("{}", matrixa.qr_decompose().1);
    // println!("{}", &matrixa.qr_decompose().0 * &matrixa.qr_decompose().1);
    // println!("{}", &matrixa.qr_decompose().1 * &matrixa.qr_decompose().0);
    // println!("{}", &matrixa.gram_schmidt().to_matrix_culumn(3).to_transpose() * &matrixa.gram_schmidt().to_matrix_culumn(5));
    // println!("{}", matrixa.gram_schmidt().to_matrix_culumn(0));
    // println!("{}", &matrixa.gram_schmidt().to_matrix_culumn(0).to_transpose() * &matrixa.gram_schmidt().to_matrix_culumn(0));
    // println!("{}", &matrixa.gram_schmidt().to_matrix_culumn(3).to_transpose() * &matrixa.gram_schmidt().to_matrix_culumn(3));
    // println!("{}", &matrixa.gram_schmidt().to_matrix_culumn(0).to_transpose() * &matrixa.gram_schmidt().to_matrix_culumn(3));
}
