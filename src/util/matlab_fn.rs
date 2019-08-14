use ndarray::{Array1, Array2};
use num_traits::Num;
use crate::structure::matrix::Matrix;
use crate::structure::vector::Vector;

pub fn zeros<T: Num + Clone>(r: usize, c: usize) -> Matrix<T> {
    Array2::zeros((r, c))
}

pub fn eye<T: Num + Clone>(n: usize) -> Matrix<T> {
    Array2::eye(n)
}

pub fn linspace<T: Num + Clone>(start: T, end: T, num: usize) -> Vector<T> {
    Array1::linspace(start, end, num)
}

pub fn logspace<T: Num + Clone>(start: T, end: T, num: usize) -> Vector<T> {
    unimplemented!()
}