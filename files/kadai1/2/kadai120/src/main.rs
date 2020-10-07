use num_traits::Zero;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<T> {
    n: usize,      // line           [* * * * *]
    m: usize,      // column         [* * * * *] -> n = 3, m = 5
    array: Vec<T>, //                [* * * * *]
}

impl<T> Matrix<T>
where
    T: Zero + Clone,
{
    pub fn new(n: usize, m: usize) -> Self {
        Matrix {
            n,
            m,
            array: vec![T::zero(); n * m],
        }
    }

    pub fn append_line(vec: Vec<Vec<T>>) -> Self {
        let n = vec.len();
        let m = vec[0].len();
        if vec.iter().all(|e| e.len() == m) {
            panic!("`append_line` needs appropriaty sized Vec<Vec<>>");
        }
        Matrix {
            n,
            m,
            array: vec.concat()
        }
    }
}

fn _calculate_y(_n: usize, _a: &Vec<f64>) -> Vec<f64> {
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
    println!("{:?}", Matrix::<f32>::new(3, 4));
}
