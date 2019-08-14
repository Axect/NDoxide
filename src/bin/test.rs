extern crate ndoxide;
extern crate ndarray;
use ndarray::prelude::*;
use ndoxide::prelude::*;

fn main() {
    let a: Matrix<f64> = zeros(10, 10);
    let b: Matrix<f64> = eye(5);
    println!("{}", a);
    println!("{}", b);
}