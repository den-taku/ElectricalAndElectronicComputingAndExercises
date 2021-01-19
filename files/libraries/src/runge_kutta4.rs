use num_traits::{Float, FromPrimitive};
use std::f64::consts::PI;

fn fun31<F: Float + FromPrimitive>(x: F, y: F) -> (F, F) {
    (y, -x)
}

pub fn runge4<F>(
    v_x: F,
    v_y: F,
    h: F,
    t: F,
    mut log: (Vec<(F, F)>, Vec<(F, F)>),
) -> (Vec<(F, F)>, Vec<(F, F)>)
where
    F: Float + FromPrimitive,
{
    let half = F::from_f64(0.5).unwrap();

    let (k1x, k1y) = match fun31(v_x, v_y) {
        (a, b) => (a * h, b * h),
    };
    let (k2x, k2y) = match fun31(v_x + k1x * half, v_y + k1y * half) {
        (a, b) => (a * h, b * h),
    };
    let (k3x, k3y) = match fun31(v_x + k2x * half, v_y + k2y * half) {
        (a, b) => (a * h, b * h),
    };
    let (k4x, k4y) = match fun31(v_x + k3x, v_y + k3y) {
        (a, b) => (a * h, b * h),
    };

    let v_x_n_1 = v_x
        + (k1x + F::from_f64(2.0).unwrap() * k2x + F::from_f64(2.0).unwrap() * k3x + k4x)
            / F::from_f64(6.0).unwrap();
    let v_y_n_1 = v_y
        + (k1y + F::from_f64(2.0).unwrap() * k2y + F::from_f64(2.0).unwrap() * k3y + k4y)
            / F::from_f64(6.0).unwrap();

    log.0.push((v_x_n_1, v_y_n_1));

    let now = (t + h).to_f64().unwrap();

    let err_norm = ((v_x_n_1 - F::from_f64(-f64::sin(now)).unwrap())
        * (v_x_n_1 - F::from_f64(-f64::sin(now)).unwrap())
        + (v_y_n_1 - F::from_f64(-f64::cos(now)).unwrap())
            * (v_y_n_1 - F::from_f64(-f64::cos(now)).unwrap()))
    .sqrt();
    log.1.push((t, err_norm));

    let now = F::from_f64(now).unwrap();

    if now <= F::from_f64(5.0).unwrap() * F::from_f64(PI).unwrap() {
        runge4(v_x_n_1, v_y_n_1, h, now, log)
    } else {
        log
    }
}

pub fn runge42<F>(v_x: F, v_y: F, h: F, t: F, max: F) -> F
where
    F: Float + FromPrimitive,
{
    let half = F::from_f64(0.5).unwrap();

    let (k1x, k1y) = match fun31(v_x, v_y) {
        (a, b) => (a * h, b * h),
    };
    let (k2x, k2y) = match fun31(v_x + k1x * half, v_y + k1y * half) {
        (a, b) => (a * h, b * h),
    };
    let (k3x, k3y) = match fun31(v_x + k2x * half, v_y + k2y * half) {
        (a, b) => (a * h, b * h),
    };
    let (k4x, k4y) = match fun31(v_x + k3x, v_y + k3y) {
        (a, b) => (a * h, b * h),
    };

    let v_x_n_1 = v_x
        + (k1x + F::from_f64(2.0).unwrap() * k2x + F::from_f64(2.0).unwrap() * k3x + k4x)
            / F::from_f64(6.0).unwrap();
    let v_y_n_1 = v_y
        + (k1y + F::from_f64(2.0).unwrap() * k2y + F::from_f64(2.0).unwrap() * k3y + k4y)
            / F::from_f64(6.0).unwrap();

    let now = (t + h).to_f64().unwrap();

    let err_norm = ((v_x_n_1 - F::from_f64(-f64::sin(now)).unwrap())
        * (v_x_n_1 - F::from_f64(-f64::sin(now)).unwrap())
        + (v_y_n_1 - F::from_f64(-f64::cos(now)).unwrap())
            * (v_y_n_1 - F::from_f64(-f64::cos(now)).unwrap()))
    .sqrt();

    let now = F::from_f64(now).unwrap();

    let max = if max >= err_norm { max } else { err_norm };

    // while 0 <= t <= 2π
    if now <= F::from_f64(2.0).unwrap() * F::from_f64(PI).unwrap() {
        runge42(v_x_n_1, v_y_n_1, h, now, max)
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
pub fn runge432<F>(
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
    let half = F::from_f64(0.5).unwrap();

    let (k1x, k1y) = match fun32(x, y, gamma) {
        (a, b) => (a * h, b * h),
    };
    let (k2x, k2y) = match fun32(x + k1x * half, y + k1y * half, gamma) {
        (a, b) => (a * h, b * h),
    };
    let (k3x, k3y) = match fun32(x + k2x * half, y + k2y * half, gamma) {
        (a, b) => (a * h, b * h),
    };
    let (k4x, k4y) = match fun32(x + k3x, y + k3y, gamma) {
        (a, b) => (a * h, b * h),
    };

    let x_n_1 = x
        + (k1x + F::from_f64(2.0).unwrap() * k2x + F::from_f64(2.0).unwrap() * k3x + k4x)
            / F::from_f64(6.0).unwrap();
    let y_n_1 = y
        + (k1y + F::from_f64(2.0).unwrap() * k2y + F::from_f64(2.0).unwrap() * k3y + k4y)
            / F::from_f64(6.0).unwrap();

    let now = t + h;

    log.0.push((now, x_n_1));
    log.1.push((now, y_n_1));

    // while 0 <= t <= 20
    if now <= F::from_f64(20.0).unwrap() {
        runge432(x_n_1, y_n_1, h, now, gamma, log)
    } else {
        log
    }
}
