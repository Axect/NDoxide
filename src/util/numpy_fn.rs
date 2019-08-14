use num_traits::Num;
use ndarray::Array1;
use crate::structure::vector::Vector;

pub fn arange<T: Num + Clone>(start: T, end: T, step: T) -> Vector<T> {
    Array1::range(start, end, step)
}



