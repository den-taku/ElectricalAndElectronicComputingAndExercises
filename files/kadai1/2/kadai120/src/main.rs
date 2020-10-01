#[derive(Clone, Debug, Default, PartialEq)]
pub struct Matrix<T> 
{
    n: i64,
    m: i64,
    array: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    //
}

fn calculate_y(n: usize, A: &Vec<f64>) -> Vec<f64> {
    vec![0.0]
}

fn main() {
    println!(
        "{:?}",
        Matrix {
            array: vec![vec![0.0]],
            n: 1,
            m: 1
        }
    );
}
