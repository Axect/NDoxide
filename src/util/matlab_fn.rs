use ndarray::{Array1, Array2};
use num_traits::{Num, Float};
use rand::distributions::uniform::SampleUniform;
use rand::prelude::*;
use crate::structure::matrix::Matrix;
use crate::structure::vector::Vector;

pub fn zero_vec<T: Num + Clone>(l: usize) -> Vector<T> {
    Array1::zeros(l)
}

pub fn zeros<T: Num + Clone>(r: usize, c: usize) -> Matrix<T> {
    Array2::zeros((r, c))
}

pub fn eye<T: Num + Clone>(n: usize) -> Matrix<T> {
    Array2::eye(n)
}

pub fn linspace<T: Num + Float + Clone>(start: T, end: T, num: usize) -> Vector<T> {
    Array1::linspace(start, end, num)
}

pub fn logspace<T: Num + Float + Clone>(start: T, end: T, num: usize) -> Vector<T> {
    let exp: Array1<T> = Array1::linspace(start, end, num);
    exp.view().map(|t: &T| (T::from(10f64).unwrap()).powf(*t))
}

pub fn rand<T: Num + SampleUniform + Clone>(r: usize, c: usize) -> Matrix<T> {
    let mut m: Matrix<T> = zeros(r, c);
    let mut rng = thread_rng();
    m.mapv_inplace(|_| rng.gen_range(T::zero(), T::one()));
    m
}