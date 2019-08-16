use crate::structure::vector::Vector;

pub trait MinimalMatrix: Sized {
    fn swap_row(&mut self, row1: usize, row2: usize);
    fn swap_col(&mut self, col1: usize, col2: usize);
    fn block(&self) -> (Self, Self, Self, Self);
    fn combine(m1: Self, m2: Self, m3: Self, m4: Self) -> Self;
}

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

pub type Perms = Vec<(usize, usize)>;