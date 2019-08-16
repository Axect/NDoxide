use std::ops::Add;
use ndarray::Array2;
use num_traits::{Num, Float};
use crate::structure::vector::Vector;
use crate::mathematics::linear_algebra::{LinearAlgebra, Norm, Perms};
use crate::mathematics::linear_algebra::Norm::*;

pub type Matrix<T> = Array2<T>;

impl<T: Num + Float + Clone> LinearAlgebra<T> for Matrix<T> {
    fn norm(&self, norm: Norm) -> T {
        match norm {
            Frobenius => {
                let mut s = T::zero();
                for x in self.genrows() {
                    s = s + x.mapv(|t| t.powi(2)).sum();
                }
                s.sqrt()
            }
            PQ(p, q) => {
                let mut s = T::zero();
                for y in self.gencolumns() {
                    s = s + y.mapv(|t| t.powi(p as i32)).sum().powf(T::from(q).unwrap() / T::from(p).unwrap());
                }
                s.powf(T::one() / T::from(q).unwrap())
            }
            One => {
                let mut m = T::min_value();
                for y in self.gencolumns() {
                    let s = y.sum();
                    if s > m {
                        m = s;
                    }
                }
                m
            }
            Infinity => {
                let mut m = T::min_value();
                for x in self.genrows() {
                    let s = x.sum();
                    if s > m {
                        m = s;
                    }
                }
                m
            }
        }
    }

    fn lu(&self) -> Option<(Perms, Perms, Self, Self)> {
        unimplemented!()
    }

    fn det(&self) -> T {
        unimplemented!()
    }

    fn inv(&self) -> Option<Self> {
        unimplemented!()
    }

    fn pseudo_inv(&self) -> Option<Self> {
        unimplemented!()
    }
}