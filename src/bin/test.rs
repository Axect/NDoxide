extern crate ndoxide;
extern crate ndarray;
use ndarray::prelude::*;
use ndoxide::prelude::*;

fn main() {
    let mut a: Matrix<usize> = zeros(5, 5);
    a[(0,1)] = 1;
    println!("{}", a);
    a.swap_row(0, 1);
    println!("{}", a);
    a.swap_col(0, 1);
    println!("{}", a);
}