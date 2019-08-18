use ndarray::{Array1, ArrayView1};
use num_traits::Num;

pub type Vector<T> = Array1<T>;
pub type VectorView<'a, T> = ArrayView1<'a, T>;

pub trait MissingVector {
    type Scalar;

    fn mul(&self) -> Self::Scalar;
}

impl<T: Num + Clone> MissingVector for Vector<T> {
    type Scalar = T;

    fn mul(&self) -> Self::Scalar {
        self.fold(T::one(), |x, y| x * y.clone())
    }
}

impl<'a, T: Num + Clone> MissingVector for VectorView<'a, T> {
    type Scalar = T;

    fn mul(&self) -> Self::Scalar {
        self.fold(T::one(), |x, y| x * y.clone())
    }
}