use num_traits::{Float, FromPrimitive};

/// 0 ~ t ~ T
/// N = 100
pub fn freed_end_pulus<F>(t: F, T: F, mut data: Vec<(F, Vec<(F, F)>)>) -> Vec<(F, Vec<(F, F)>)>
where
    F: Float + FromPrimitive,
{
    let n = F::from_f64(100.0).unwrap();
    let dx = F::from_f64(1.0).unwrap() / n;
    let dt = F::from_f64(0.001).unwrap();
    let c = F::from_f64(5.0).unwrap();
    let ts = F::from_f64(0.1).unwrap();
    let td = F::from_f64(0.1).unwrap();
    let a = F::from_f64(0.01).unwrap();
    let one = F::from_f64(1.0).unwrap();
    let two = F::from_f64(2.0).unwrap();
    let mu = c * c * dt * dt / dx / dx;

    let past2: Vec<F> = data[data.len() - 2].1.clone().iter().map(|e| e.1).collect();
    let past1: Vec<F> = data[data.len() - 1].1.clone().iter().map(|e| e.1).collect();

    let mut data_now = Vec::new();

    // x0
    let x0 = (((t - ts) / a).tanh() + one) / two - (((t - ts - td) / a).tanh() + one) / two;
    data_now.push((F::from_f64(0.0).unwrap(), x0));

    // xi (0 < i < 99)
    for i in 1..99 {
        let u = -past2[i] + mu * past1[i - 1] + two * (one - mu) * past1[i] + mu * past1[i + 1];
        data_now.push((dx * F::from_usize(i).unwrap(), u))
    }

    // x_{n-1}
    let xn_1 = -past2[100 - 1] + mu * past1[100 - 2] + (two - mu) * past1[100 - 1];
    data_now.push((dx * F::from_f64(99.0).unwrap(), xn_1));

    let now = t + dt;

    data.push((now, data_now));

    if t < T {
        freed_end_pulus(now, T, data)
    } else {
        data
    }
}

/// 0 ~ t ~ T
/// N = 100
pub fn freed_end_pulus2<F>(t: F, T: F, mut data: Vec<(F, Vec<(F, F)>)>) -> Vec<(F, Vec<(F, F)>)>
where
    F: Float + FromPrimitive,
{
    let n = F::from_f64(100.0).unwrap();
    let dx = F::from_f64(1.0).unwrap() / n;
    let dt = F::from_f64(0.001).unwrap();
    let c = F::from_f64(5.0).unwrap();
    let ts = F::from_f64(0.1).unwrap();
    let td = F::from_f64(0.1).unwrap();
    let a = F::from_f64(0.01).unwrap();
    let one = F::from_f64(1.0).unwrap();
    let two = F::from_f64(2.0).unwrap();
    let mu = c * c * dt * dt / dx / dx;

    let past2: Vec<F> = data[data.len() - 2].1.clone().iter().map(|e| e.1).collect();
    let past1: Vec<F> = data[data.len() - 1].1.clone().iter().map(|e| e.1).collect();

    let mut data_now = Vec::new();

    // x0
    let x0 = if t < ts {
        F::from_f64(0.0).unwrap()
    } else if ts <= t && t <= ts + td {
        one
    } else {
        F::from_f64(0.0).unwrap()
    };
    data_now.push((F::from_f64(0.0).unwrap(), x0));

    // xi (0 < i < 99)
    for i in 1..99 {
        let u = -past2[i] + mu * past1[i - 1] + two * (one - mu) * past1[i] + mu * past1[i + 1];
        data_now.push((dx * F::from_usize(i).unwrap(), u))
    }

    // x_{n-1}
    let xn_1 = -past2[100 - 1] + mu * past1[100 - 2] + (two - mu) * past1[100 - 1];
    data_now.push((dx * F::from_f64(99.0).unwrap(), xn_1));

    let now = t + dt;

    data.push((now, data_now));

    if t < T {
        freed_end_pulus2(now, T, data)
    } else {
        data
    }
}

/// 0 ~ t ~ T
/// N = 100
pub fn fixed_end_pulus<F>(t: F, T: F, mut data: Vec<(F, Vec<(F, F)>)>) -> Vec<(F, Vec<(F, F)>)>
where
    F: Float + FromPrimitive,
{
    let n = F::from_f64(100.0).unwrap();
    let dx = F::from_f64(1.0).unwrap() / n;
    let dt = F::from_f64(0.001).unwrap();
    let c = F::from_f64(5.0).unwrap();
    let ts = F::from_f64(0.1).unwrap();
    let td = F::from_f64(0.1).unwrap();
    let a = F::from_f64(0.01).unwrap();
    let one = F::from_f64(1.0).unwrap();
    let two = F::from_f64(2.0).unwrap();
    let mu = c * c * dt * dt / dx / dx;

    let past2: Vec<F> = data[data.len() - 2].1.clone().iter().map(|e| e.1).collect();
    let past1: Vec<F> = data[data.len() - 1].1.clone().iter().map(|e| e.1).collect();

    let mut data_now = Vec::new();

    // x0
    let x0 = (((t - ts) / a).tanh() + one) / two - (((t - ts - td) / a).tanh() + one) / two;
    data_now.push((F::from_f64(0.0).unwrap(), x0));

    // xi (0 < i < 99)
    for i in 1..99 {
        let u = -past2[i] + mu * past1[i - 1] + two * (one - mu) * past1[i] + mu * past1[i + 1];
        data_now.push((dx * F::from_usize(i).unwrap(), u))
    }

    // x_{n-1}
    let xn_1 = -past2[100 - 1] + mu * past1[100 - 2] + two * (one - mu) * past1[100 - 1];
    data_now.push((dx * F::from_f64(99.0).unwrap(), xn_1));

    let now = t + dt;

    data.push((now, data_now));

    if t < T {
        fixed_end_pulus(now, T, data)
    } else {
        data
    }
}

/// 0 ~ t ~ T
/// N = 100
pub fn fixed_end_pulus2<F>(t: F, T: F, mut data: Vec<(F, Vec<(F, F)>)>) -> Vec<(F, Vec<(F, F)>)>
where
    F: Float + FromPrimitive,
{
    let n = F::from_f64(100.0).unwrap();
    let dx = F::from_f64(1.0).unwrap() / n;
    let dt = F::from_f64(0.001).unwrap();
    let c = F::from_f64(5.0).unwrap();
    let ts = F::from_f64(0.1).unwrap();
    let td = F::from_f64(0.1).unwrap();
    let a = F::from_f64(0.01).unwrap();
    let one = F::from_f64(1.0).unwrap();
    let two = F::from_f64(2.0).unwrap();
    let mu = c * c * dt * dt / dx / dx;

    let past2: Vec<F> = data[data.len() - 2].1.clone().iter().map(|e| e.1).collect();
    let past1: Vec<F> = data[data.len() - 1].1.clone().iter().map(|e| e.1).collect();

    let mut data_now = Vec::new();

    // x0
    let x0 = if t < ts {
        F::from_f64(0.0).unwrap()
    } else if ts <= t && t <= ts + td {
        one
    } else {
        F::from_f64(0.0).unwrap()
    };
    data_now.push((F::from_f64(0.0).unwrap(), x0));

    // xi (0 < i < 99)
    for i in 1..99 {
        let u = -past2[i] + mu * past1[i - 1] + two * (one - mu) * past1[i] + mu * past1[i + 1];
        data_now.push((dx * F::from_usize(i).unwrap(), u))
    }

    // x_{n-1}
    let xn_1 = -past2[100 - 1] + mu * past1[100 - 2] + two * (one - mu) * past1[100 - 1];
    data_now.push((dx * F::from_f64(99.0).unwrap(), xn_1));

    let now = t + dt;

    data.push((now, data_now));

    if t < T {
        fixed_end_pulus2(now, T, data)
    } else {
        data
    }
}
