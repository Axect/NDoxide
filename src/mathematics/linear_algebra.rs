use crate::structure::vector::Vector;

pub trait LinearAlgebra<T>: Sized {
    fn norm(&self, norm: Norm) -> T;
    fn lu(&self) -> Option<(Perms, Perms, Self, Self)>;
    fn det(&self) -> T;
    fn inv(&self) -> Option<Self>;
    fn pseudo_inv(&self) -> Option<Self>;
}

#[derive(Debug, Copy, Clone)]
pub enum Norm {
    Frobenius,
    PQ(usize, usize),
    One,
    Infinity,
}

pub type Perms = Vector<(usize, usize)>;