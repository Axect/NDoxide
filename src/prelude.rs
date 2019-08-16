pub use crate::num_traits::Num;

pub use crate::structure::matrix::{
    Matrix,
    combine,
};
pub use crate::structure::vector::Vector;
pub use crate::mathematics::linear_algebra::{
    Perms,
    LinearAlgebra,
    MinimalMatrix,
    Norm,
    Norm::{
        Frobenius,
        PQ,
        One,
        Infinity,
    }
};

pub use crate::util::matlab_fn::{
    zeros,
    eye,
    linspace,
    logspace,
};