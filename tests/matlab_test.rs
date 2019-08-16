extern crate ndoxide;
use ndoxide::prelude::*;

#[test]
fn linspace_test() {
    let v: Vector<f64> = linspace(0f64, 10f64, 11);
    let w: Vector<f64> = Vector::from_vec(vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.,]);
    assert_eq!(v, w);
}

#[test]
fn logspace_test() {
    let v: Vector<f64> = logspace(0f64, 5f64, 6);
    let w: Vector<f64> = Vector::from_vec(vec![1., 10., 100., 1000., 10000., 100000.,]);
    assert_eq!(v, w);
}