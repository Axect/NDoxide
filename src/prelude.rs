pub use crate::num_traits::Num;

pub use crate::structure::matrix::{
    Matrix,
    MatrixView,
    combine,
    mat,
};
pub use crate::structure::vector::{
    Vector,
    VectorView,
    MissingVector,
};
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
pub use crate::statistics::rand::{
    rand_num,
    ziggurat,
    marsaglia_polar
};

pub use crate::util::matlab_fn::{
    zeros,
    zero_vec,
    eye,
    linspace,
    logspace,
    rand,
};