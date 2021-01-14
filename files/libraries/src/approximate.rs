use num_traits::Float;
use num_traits::FromPrimitive;
use std::f64::consts::PI;

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

    let err_norm = ((v_x_n_1 - v_x) * (v_x_n_1 - v_x) + (v_y_n_1 - v_y) + (v_y_n_1 - v_y)).sqrt();
    log.1.push((t, err_norm));

    let now = t + h;

    // while 0 <= t <= 5π
    if now <= F::from_f64(5.0).unwrap() * F::from_f64(PI).unwrap() {
        euler(v_x_n_1, v_y_n_1, h, now, log)
    } else {
        log
    }
}
