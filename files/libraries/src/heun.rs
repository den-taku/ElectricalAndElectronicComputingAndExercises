use num_traits::{Float, FromPrimitive};
use std::f64::consts::PI;

pub fn heun<F>(
    v_x: F,
    v_y: F,
    h: F,
    t: F,
    mut log: (Vec<(F, F)>, Vec<(F, F)>),
) -> (Vec<(F, F)>, Vec<(F, F)>)
where
    F: Float + FromPrimitive,
{
    let v_x_n_1 = v_x + h * (F::from_f64(2.0).unwrap() + h) * v_y / F::from_f64(2.0).unwrap();
    let v_y_n_1 = v_y - h * (F::from_f64(2.0).unwrap() + h) * v_x / F::from_f64(2.0).unwrap();
    log.0.push((v_x_n_1, v_y_n_1));

    let now = (t + h).to_f64().unwrap();

    let err_norm = ((v_x_n_1 + F::from_f64(-f64::sin(now)).unwrap())
        * (v_x_n_1 + F::from_f64(-f64::sin(now)).unwrap())
        + (v_y_n_1 + F::from_f64(-f64::cos(now)).unwrap())
            * (v_y_n_1 + F::from_f64(-f64::cos(now)).unwrap()))
    .sqrt();
    log.1.push((t, err_norm));

    let now = F::from_f64(now).unwrap();

    if now <= F::from_f64(5.0).unwrap() * F::from_f64(PI).unwrap() {
        heun(v_x_n_1, v_y_n_1, h, now, log)
    } else {
        log
    }
}

pub fn heun2<F>(v_x: F, v_y: F, h: F, t: F, max: F) -> F
where
    F: Float + FromPrimitive,
{
    let v_x_n_1 = v_x + h * (F::from_f64(2.0).unwrap() + h) * v_y / F::from_f64(2.0).unwrap();
    let v_y_n_1 = v_y - h * (F::from_f64(2.0).unwrap() + h) * v_x / F::from_f64(2.0).unwrap();

    let now = (t + h).to_f64().unwrap();

    let err_norm = ((v_x_n_1 + F::from_f64(-f64::sin(now)).unwrap())
        * (v_x_n_1 + F::from_f64(-f64::sin(now)).unwrap())
        + (v_y_n_1 + F::from_f64(-f64::cos(now)).unwrap())
            * (v_y_n_1 + F::from_f64(-f64::cos(now)).unwrap()))
    .sqrt();

    let now = F::from_f64(now).unwrap();

    let max = if max >= err_norm { max } else { err_norm };

    // while 0 <= t <= 2π
    if now <= F::from_f64(2.0).unwrap() * F::from_f64(PI).unwrap() {
        heun2(v_x_n_1, v_y_n_1, h, now, max)
    } else {
        max
    }
}

fn fun32<F: Float + FromPrimitive>(x: F, y: F, gamma: F) -> (F, F) {
    (
        (F::from_f64(3.0).unwrap() - gamma * x - F::from_f64(9.0).unwrap() * y) * x,
        (-F::from_f64(2.0).unwrap() + F::from_f64(2.0).unwrap() * x) * y,
    )
}

/// dx/dt = (3 - γ * x - 9 * y) * x
/// dy/dt = (-2 + 2 * x) * y
pub fn heun32<F>(
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
    let (k1x, k1y) = match fun32(x, y, gamma) {
        (a, b) => (a * h, b * h),
    };
    let (k2x, k2y) = match fun32(x + k1x, y + k1y, gamma) {
        (a, b) => (a * h, b * h),
    };

    let x_n_1 = x + (k1x + k2x) / F::from_f64(2.0).unwrap();
    let y_n_1 = y + (k1y + k2y) / F::from_f64(2.0).unwrap();

    let now = t + h;

    log.0.push((now, x_n_1));
    log.1.push((now, y_n_1));

    // while 0 <= t <= 20
    if now <= F::from_f64(20.0).unwrap() {
        heun32(x_n_1, y_n_1, h, now, gamma, log)
    } else {
        log
    }
}
