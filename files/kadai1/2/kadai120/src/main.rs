#[derive(Clone, Debug, Default, PartialEq)]
pub struct Matrix<T> {
    n: usize, // line
    m: usize, // column
    array: Vec<T>,
}

impl<T> Matrix<T> 
    where T: Default + Clone
{
    pub fn new(n: usize, m: usize) -> Self {
        Matrix { n, m, array: vec![T::default(); n * m] }
    }
}

fn calculate_y(n: usize, A: &Vec<f64>) -> Vec<f64> {
    vec![0.0]
}

fn main() {
    println!(
        "{:?}",
        Matrix {
            n: 1,
            m: 1,
            array: vec![0.0]
        }
    );
}
