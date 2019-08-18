extern crate ndoxide;
extern crate ndarray;
use ndoxide::prelude::*;
use ndarray::s;
fn main() {
    // Make random matrix
    let mut s: Matrix<f64> = rand(5, 5);
    for i in 0 .. 5 {
        s[(i,i)] = s[(i,i)] * 10f64;
        for j in 0 .. i {
            s[(i,j)] = s[(j,i)];
        }
    }

    // println!("{}", s);

    let mut ds = gram_schmidt(&s);

    // println!("{}", ds);

    // matmul(&matmul(&ds.t(),&s), &ds).print();

    for _i in 0 .. 10 {
        s = (ds.t().dot(&s)).dot(&ds);
        ds = gram_schmidt(&s);
    }

    println!("{}", s);
    
    // let mut s_raw = s.to_vec();
    // let mut ds_raw = gram_schmidt_raw(&s_raw);

    // for _n in 0 .. 100 {
    //     let ds = py_matrix(ds_raw.clone());
    //     let s = py_matrix(s_raw.clone());
    //     s_raw = (ds.t() * s * ds).to_vec();
    //     ds_raw = gram_schmidt_raw(&s_raw);
    // }
}

fn proj(u: &VectorView<f64>, v: &VectorView<f64>) -> Vector<f64> {
    let uv = u.dot(v);
    let uu = u.dot(u);
    u.mapv(|x| x * uv / uu)
}

fn gram_schmidt(vs: &Matrix<f64>) -> Matrix<f64> {
    let mut result: Matrix<f64> =  zeros(vs.rows(), vs.cols());
    result.row_mut(0).assign(&vs.row(0));

    for k in 1 .. vs.cols() {
        let vk = vs.column(k);
        let mut puv: Vector<f64> = zero_vec(vk.len());
        for j in 0 .. k {
            puv += &proj(&result.row(j), &vk);
        }
        result.row_mut(k).assign(&(&vk - &puv));
    }

    for i in 0 .. result.rows() {
        let rr = result.row(i);
        let norm = rr.dot(&rr);
        result.slice_mut(s![i, ..]).mapv_inplace(|x| x / norm);
    }
    result
}