use num_traits::{Num, Float};
use ndarray::Array1;
use crate::structure::vector::Vector;

pub fn arange<T: Num + Float + Clone>(start: T, end: T, step: T) -> Vector<T> {
    Array1::range(start, end, step)
}



