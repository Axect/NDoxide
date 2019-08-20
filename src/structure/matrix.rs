use ndarray::{Array2, ArrayView2, arr2, FixedInitializer};
use num_traits::{Num, Float};
use crate::structure::vector::{Vector, MissingVector};
use crate::mathematics::linear_algebra::{LinearAlgebra, Norm, Perms, MissingMatrix};
use crate::mathematics::linear_algebra::Norm::*;
use crate::util::matlab_fn::zeros;

pub type Matrix<T> = Array2<T>;
pub type MatrixView<'a, T> = ArrayView2<'a, T>;

impl<T: Num + Copy + Clone> MissingMatrix<T> for Matrix<T> {
    fn swap_row(&mut self, row1: usize, row2: usize) {
        let temp1: Vector<T> = self.row(row1).to_owned();
        let temp2: Vector<T> = self.row(row2).to_owned();
        self.row_mut(row1).zip_mut_with(&temp2, |x, y| *x = *y);
        self.row_mut(row2).zip_mut_with(&temp1, |x, y| *x = *y);
    }

    fn swap_col(&mut self, col1: usize, col2: usize) {
        let temp1: Vector<T> = self.column(col1).to_owned();
        let temp2: Vector<T> = self.column(col2).to_owned();
        self.column_mut(col1).zip_mut_with(&temp2, |x, y| *x = *y);
        self.column_mut(col2).zip_mut_with(&temp1, |x, y| *x = *y);
    }

    fn map_row<F>(&self, f: F) -> Self where
        F: Fn(Vector<T>) -> Vector<T> {
        let mut result: Matrix<T> = zeros(self.rows(), self.cols());
        let mut i = 0usize;
        for v in self.genrows() {
            result.row_mut(i).assign(&v);
            i += 1;
        }
        result
    }

    fn map_col<F>(&self, f: F) -> Self where
        F: Fn(Vector<T>) -> Vector<T> {
        let mut result: Matrix<T> = zeros(self.rows(), self.cols());
        let mut i = 0usize;
        for v in self.gencolumns() {
            result.column_mut(i).assign(&v);
            i += 1;
        }
        result
    }

    fn map_row_mut<F>(&mut self, f: F) where
        F: Fn(Vector<T>) -> Vector<T> {
        for v in self.genrows_mut() {
            v = f(v);
        }
    }

    fn map_col_mut<F>(&mut self, f: F) where
        F: Fn(Vector<T>) -> Vector<T> {
        for v in self.gencolumns_mut() {
            v = f(v);
        }
    }

    fn block(&self) -> (Self, Self, Self, Self) {
        let r = self.rows();
        let c = self.cols();
        let l_r = r / 2;
        let l_c = c / 2;
        let r_l = r - l_r;
        let c_l = c - l_c;

        let mut m1: Matrix<T> = zeros(l_r, l_c);
        let mut m2: Matrix<T> = zeros(l_r, c_l);
        let mut m3: Matrix<T> = zeros(r_l, l_c);
        let mut m4: Matrix<T> = zeros(r_l, c_l);

        for idx_row in 0..r {
            for idx_col in 0..c {
                match (idx_row, idx_col) {
                    (i, j) if (i < l_r) && (j < l_c) => {
                        m1[[i, j]] = self[[i, j]];
                    }
                    (i, j) if (i < l_r) && (j >= l_c) => {
                        m2[[i, j - l_c]] = self[[i, j]];
                    }
                    (i, j) if (i >= l_r) && (j < l_c) => {
                        m3[[i - l_r, j]] = self[[i, j]];
                    }
                    (i, j) if (i >= l_r) && (j >= l_c) => {
                        m4[[i - l_r, j - l_c]] = self[[i, j]];
                    }
                    _ => (),
                }
            }
        }
        (m1, m2, m3, m4)
    }

    fn combine(m1: Self, m2: Self, m3: Self, m4: Self) -> Self {
        let l_r = m1.rows();
        let l_c = m1.cols();
        let c_l = m2.cols();
        let r_l = m3.rows();

        let r = l_r + r_l;
        let c = l_c + c_l;

        let mut m: Matrix<T> = zeros(r, c);

        for idx_row in 0..r {
            for idx_col in 0..c {
                match (idx_row, idx_col) {
                    (i, j) if (i < l_r) && (j < l_c) => {
                        m[[i, j]] = m1[[i, j]];
                    }
                    (i, j) if (i < l_r) && (j >= l_c) => {
                        m[[i, j]] = m2[[i, j - l_c]];
                    }
                    (i, j) if (i >= l_r) && (j < l_c) => {
                        m[[i, j]] = m3[[i - l_r, j]];
                    }
                    (i, j) if (i >= l_r) && (j >= l_c) => {
                        m[[i, j]] = m4[[i - l_r, j - l_c]];
                    }
                    _ => (),
                }
            }
        }
        m
    }
}

pub fn combine<T: Num + Copy + Clone>(m1: Matrix<T>, m2: Matrix<T>, m3: Matrix<T>, m4: Matrix<T>) -> Matrix<T> {
    MissingMatrix::combine(m1, m2, m3, m4)
}

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
        assert_eq!(self.cols(), self.rows());
        let n = self.rows();

        let mut l: Matrix<T> = zeros(n, n);
        let mut u: Matrix<T> = zeros(n, n);

        // ---------------------------------------
        // Pivoting - Complete
        // ---------------------------------------
        // Permutations
        let mut p: Perms = Vec::new();
        let mut q: Perms = Vec::new();

        let mut container: Matrix<T> = self.clone();

        for k in 0..(n - 1) {
            // Initialize maximum & Position
            let mut m = T::zero();
            let mut row_idx: usize = k;
            let mut col_idx: usize = k;
            // Find Maximum value & Position
            for i in k..n {
                for j in k..n {
                    let temp = container[(i, j)];
                    if temp.abs() > m.abs() {
                        m = temp;
                        row_idx = i;
                        col_idx = j;
                    }
                }
            }
            if k != row_idx {
                container.swap_row(k, row_idx); // Row perm
                p.push((k, row_idx));
            }
            if k != col_idx {
                container.swap_col(k, col_idx); // Col perm
                q.push((k, col_idx));
            }
        }

        // ---------------------------------------
        // Obtain L & U
        // ---------------------------------------
        let reference: MatrixView<T> = container.view();

        // Initialize U
        for i in 0..n {
            u[(0, i)] = reference[(0, i)];
        }

        // Initialize L
        for i in 0..n {
            l[(i, i)] = T::one();
        }

        for i in 0..n {
            for k in i..n {
                let mut s = T::zero();
                for j in 0..i {
                    s = s + l[(i, j)] * u[(j, k)];
                }
                u[(i, k)] = reference[(i, k)] - s;
                // Check non-zero diagonal
                if u[(i, i)] == T::zero() {
                    return None;
                }
            }

            for k in (i + 1)..n {
                let mut s = T::zero();
                for j in 0..i {
                    s = s + l[(k, j)] * u[(j, i)];
                }
                l[(k, i)] = (reference[(k, i)] - s) / u[(i, i)];
            }
        }
        Some((p, q, l, u))
    }

    fn det(&self) -> T {
        assert_eq!(self.rows(), self.cols());
        match self.lu() {
            None => T::zero(),
            Some(pqlu) => {
                let (p, q, _l, u) = pqlu;

                // sgn of perms
                let sgn_p = T::from(2.0 * (p.len() % 2) as f64 - 1.0).unwrap();
                let sgn_q = T::from(2.0 * (q.len() % 2) as f64 - 1.0).unwrap();

                u.diag().mul() * sgn_p * sgn_q
            }
        }
    }

    fn inv(&self) -> Option<Self> {
        unimplemented!()
    }

    fn pseudo_inv(&self) -> Option<Self> {
        unimplemented!()
    }
}

pub fn mat<T: Num + Clone, V: FixedInitializer<Elem=T>>(arr: &[V]) -> Matrix<T> 
where V: Clone
{
    arr2(arr)
}