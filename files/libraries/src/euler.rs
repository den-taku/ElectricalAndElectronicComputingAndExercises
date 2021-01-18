use num_traits::FromPrimitive;
use num_traits::{Float, Zero};
use std::f64::consts::PI;

pub fn least_squares_method<F>(data: Vec<(F, F)>) -> (F, F)
where
    F: Float + Zero + FromPrimitive,
{
    let sigma_one = F::from_usize(data.len()).unwrap();
    let mut sigma_x = F::zero();
    let mut sigma_y = F::zero();
    let mut sigma_xy = F::zero();
    let mut sigma_xx = F::zero();

    for e in data {
        sigma_x = sigma_x + e.0;
        sigma_y = sigma_y + e.1;
        sigma_xy = sigma_xy + e.0 * e.1;
        sigma_xx = sigma_xx + e.0 * e.0;
    }

    (
        (sigma_x * sigma_y - sigma_one * sigma_xy) / (sigma_x * sigma_x - sigma_one * sigma_xx),
        (sigma_x * sigma_xy - sigma_y * sigma_xx) / (sigma_x * sigma_x - sigma_one * sigma_xx),
    )
}

/// v_x_n+1 = v_x + h * v_y
/// v_y_n+1 = v_y - h * v_x
/// v_z_n+1 = 0
pub fn euler<F>(
    v_x: F,
    v_y: F,
    h: F,
    t: F,
    mut log: (Vec<(F, F)>, Vec<(F, F)>), // (data, norm)
) -> (Vec<(F, F)>, Vec<(F, F)>)
where
    F: Float + FromPrimitive,
{
    let v_x_n_1 = v_x + h * v_y;
    let v_y_n_1 = v_y - h * v_x;
    log.0.push((v_x_n_1, v_y_n_1));

    let now = (t + h).to_f64().unwrap();

    let err_norm = ((v_x_n_1 - F::from_f64(-f64::sin(now)).unwrap())
        * (v_x_n_1 - F::from_f64(-f64::sin(now)).unwrap())
        + (v_y_n_1 - F::from_f64(-f64::cos(now)).unwrap())
            * (v_y_n_1 - F::from_f64(-f64::cos(now)).unwrap()))
    .sqrt();
    log.1.push((t, err_norm));

    let now = F::from_f64(now).unwrap();

    // while 0 <= t <= 5π
    if now <= F::from_f64(5.0).unwrap() * F::from_f64(PI).unwrap() {
        euler(v_x_n_1, v_y_n_1, h, now, log)
    } else {
        log
    }
}

pub fn euler2<F>(v_x: F, v_y: F, h: F, t: F, max: F) -> F
where
    F: Float + FromPrimitive,
{
    let v_x_n_1 = v_x + h * v_y;
    let v_y_n_1 = v_y - h * v_x;

    let now = (t + h).to_f64().unwrap();

    let err_norm = ((v_x_n_1 - F::from_f64(-f64::sin(now)).unwrap())
        * (v_x_n_1 - F::from_f64(-f64::sin(now)).unwrap())
        + (v_y_n_1 - F::from_f64(-f64::cos(now)).unwrap())
            * (v_y_n_1 - F::from_f64(-f64::cos(now)).unwrap()))
    .sqrt();
    // log.1.push((t, err_norm));

    let now = F::from_f64(now).unwrap();

    let max = if max >= err_norm { max } else { err_norm };
    // println!("max: {}", max.to_f64().unwrap());

    // while 0 <= t <= 2π
    if now <= F::from_f64(2.0).unwrap() * F::from_f64(PI).unwrap() {
        euler2(v_x_n_1, v_y_n_1, h, now, max)
    } else {
        max
    }
}

/// dx/dt = (3 - γ * x - 9 * y) * x
/// dy/dt = (-2 + 2 * x) * y
pub fn euler32<F>(
    x: F,
    y: F,
    h: F,
    t: F,
    gamma: F,
    mut log: (Vec<(F, F)>, Vec<(F, F)>), // (data, norm)
) -> (Vec<(F, F)>, Vec<(F, F)>)
where
    F: Float + FromPrimitive,
{
    let x_n_1 = x + h * (F::from_f64(3.0).unwrap() - gamma * x - F::from_f64(9.0).unwrap() * y) * x;
    let y_n_1 = y + h * (-F::from_f64(2.0).unwrap() + F::from_f64(2.0).unwrap() * x) * y;

    let now = t + h;

    log.0.push((now, x_n_1));
    log.1.push((now, y_n_1));

    // while 0 <= t <= 20
    if now <= F::from_f64(20.0).unwrap() {
        euler32(x_n_1, y_n_1, h, now, gamma, log)
    } else {
        log
    }
}

// v_x_n+1 = v_x + h * v_y
// v_y_n+1 = v_y - h * v_x
// v_z_n+1 = 0
// pub fn euler2_dash<F>(
//     h: F,
//     now: F,
//     log: (&mut Vec<(F, F)>, &mut Vec<(F, F)>), // (data, norm)
// ) where
//     F: Float + FromPrimitive,
// {
//     let (v_x, v_y) = log.0.pop().unwrap();
//     log.0.push((v_x, v_y));
//     let v_x_n_1 = v_x + h * v_y;
//     let v_y_n_1 = v_y - h * v_x;
//     log.0.push((v_x_n_1, v_y_n_1));

//     let now = now.to_f64().unwrap();

//     let err_norm = ((v_x_n_1 + F::from_f64(-f64::sin(now)).unwrap())
//         * (v_x_n_1 + F::from_f64(-f64::sin(now)).unwrap())
//         + (v_y_n_1 + F::from_f64(-f64::cos(now)).unwrap())
//             * (v_y_n_1 + F::from_f64(-f64::cos(now)).unwrap()))
//     .sqrt();
//     let log_err_norm = F::log2(err_norm);
//     // println!("       {}", log_err_norm.to_f64().unwrap());
//     log.1.push((F::from_f64(now).unwrap(), log_err_norm));
// }
