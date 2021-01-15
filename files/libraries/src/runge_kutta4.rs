use num_traits::{Float, FromPrimitive};
use std::f64::consts::PI;

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
    let hs = h
        + h * h / F::from_f64(2.0).unwrap()
        + h * h * h / F::from_f64(6.0).unwrap()
        + h * h * h * h / F::from_f64(24.0).unwrap();
    let v_x_n_1 = v_x + hs * v_y;
    let v_y_n_1 = v_y - hs * v_x;
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
        runge4(v_x_n_1, v_y_n_1, h, now, log)
    } else {
        log
    }
}

pub fn runge42<F>(v_x: F, v_y: F, h: F, t: F, max: F) -> F
where
    F: Float + FromPrimitive,
{
    let hs = h
        + h * h / F::from_f64(2.0).unwrap()
        + h * h * h / F::from_f64(6.0).unwrap()
        + h * h * h * h / F::from_f64(24.0).unwrap();
    let v_x_n_1 = v_x + hs * v_y;
    let v_y_n_1 = v_y - hs * v_x;

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
        runge42(v_x_n_1, v_y_n_1, h, now, max)
    } else {
        max
    }
}
