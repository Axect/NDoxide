extern crate ndoxide;
extern crate ndarray;
use ndarray::prelude::*;
use ndoxide::prelude::*;

fn main() {
    let a: Matrix<f64> = zeros(5000, 5000);
    println!("{}", a.norm(PQ(1,2)));
}