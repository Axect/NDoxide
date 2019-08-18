extern crate ndoxide;
extern crate ndarray;
use ndarray::prelude::*;
use ndoxide::prelude::*;

fn main() {
    let mut a: Matrix<f64> = arr2(&[[1f64,2f64],[3f64,4f64]]);
    let (p, q, l, u) = a.lu().unwrap();
    println!("{:?}", p);
    println!("{:?}", q);
    println!("{}", l);
    println!("{}", u);
}