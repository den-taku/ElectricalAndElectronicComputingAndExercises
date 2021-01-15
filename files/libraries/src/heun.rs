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
