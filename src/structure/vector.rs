use ndarray::{Array1, ArrayView1};

pub type Vector<T> = Array1<T>;
pub type VectorView<'a, T> = ArrayView1<'a, T>;
