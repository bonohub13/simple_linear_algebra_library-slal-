macro_rules! impl_add {
    ($($t:ty)*) => ($(
        impl std::ops::Add for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn add(self, other: super::Matrix<$t>) -> super::Matrix<$t> {
                use super::Matrix;
                use std::iter::zip;

                if self.size() == other.size() {
                    let rv: Vec<Vec<$t>> = zip(self.to_vec(), other.to_vec())
                        .map(|(v, w)| {
                            let rv_i: Vec<$t> = zip(v.as_slice(), w.as_slice())
                                .map(|(v_i, w_i)| *v_i + *w_i)
                                .collect();

                            rv_i
                        })
                        .collect();
                    let rv_slices: Vec<&[$t]> = rv
                        .iter()
                        .map(|rv_i| rv_i.as_slice())
                        .collect();

                    Matrix::<$t>::new(rv_slices.as_slice()).unwrap()
                } else {
                    panic!("Failed to add two matrices with differing size");
                }
            }
        }
    )*)
}

impl_add! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_sub {
    ($($t:ty)*) => ($(
        impl std::ops::Sub for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn sub(self, other: super::Matrix<$t>) -> super::Matrix<$t> {
                use super::Matrix;
                use std::iter::zip;

                if self.size() == other.size() {
                    let rv: Vec<Vec<$t>> = zip(self.to_vec(), other.to_vec())
                        .map(|(v, w)| {
                            let rv_i: Vec<$t> = zip(v.as_slice(), w.as_slice())
                                .map(|(v_i, w_i)| *v_i - *w_i)
                                .collect();

                            rv_i
                        })
                        .collect();
                    let rv_slices: Vec<&[$t]> = rv
                        .iter()
                        .map(|rv_i| rv_i.as_slice())
                        .collect();

                    Matrix::<$t>::new(rv_slices.as_slice()).unwrap()
                } else {
                    panic!("Failed to substract two matrices with differing size");
                }
            }
        }
    )*)
}

impl_sub! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_mul_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<$t> for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn mul(self, other: $t) -> super::Matrix<$t> {
                use super::Matrix;

                let rv: Vec<Vec<$t>> = self
                    .to_vec()
                    .iter()
                    .map(|v| {
                        let rv_i: Vec<$t> = v
                            .iter()
                            .map(|v_i| *v_i * other)
                            .collect();

                        rv_i
                    })
                    .collect();
                let rv_slices: Vec<&[$t]> = rv
                    .iter()
                    .map(|rv_i| rv_i.as_slice())
                    .collect();

                Matrix::<$t>::new(rv_slices.as_slice()).unwrap()
            }
        }
    )*)
}

impl_mul_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_mul_with_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Matrix<$t>> for $t {
            type Output = super::Matrix<$t>;

            fn mul(self, other: super::Matrix<$t>) -> super::Matrix<$t> {
                use super::Matrix;

                let rv: Vec<Vec<$t>> = other
                    .to_vec()
                    .iter()
                    .map(|v| {
                        let rv_i: Vec<$t> = v
                            .iter()
                            .map(|v_i| *v_i * self)
                            .collect();

                        rv_i
                    })
                    .collect();
                let rv_slices: Vec<&[$t]> = rv
                    .iter()
                    .map(|rv_i| rv_i.as_slice())
                    .collect();

                Matrix::<$t>::new(rv_slices.as_slice()).unwrap()
            }
        }
    )*)
}

impl_mul_with_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_mul_vector {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<crate::vertex::Vertex<$t>> for super::Matrix<$t> {
            type Output = crate::vertex::Vertex<$t>;

            fn mul(self, other: crate::vertex::Vertex<$t>) -> crate::vertex::Vertex<$t> {
                use crate::vertex::Vertex;
                use std::iter::zip;

                if other.is_transposed() {
                    if self.size().0 == other.len() {
                        let v = other.to_vec();
                        let prod: Vec<$t> = self
                            .to_vec()
                            .iter()
                            .map(|m_i| {
                                let prod_i: $t = zip(m_i.as_slice(), v.as_slice())
                                    .map(|(m_ij, v_i)| *m_ij * v_i)
                                    .sum();

                                prod_i
                            })
                            .collect();
                        let mut rv = Vertex::<$t>::new(prod.as_slice());

                        rv.t();

                        rv
                    } else {
                        panic!(
                            "Cannot multiply matrix and vector when width of matrix and length of transposed vector does not match"
                            );
                    }
                } else {
                    panic!(
                        "Cannot multiply matrix with vector when vector is not transposed"
                        );
                }
            }
        }
    )*)
}

impl_mul_vector! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_mul_with_vector {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Matrix<$t>> for crate::vertex::Vertex<$t> {
            type Output = crate::vertex::Vertex<$t>;

            fn mul(self, other: super::Matrix<$t>) -> crate::vertex::Vertex<$t> {
                use crate::vertex::Vertex;

                if !self.is_transposed() {
                    let size = other.size();
                    if size.1 == self.len() {
                        let m = other.to_vec();
                        let v = self.to_vec();
                        let prod: Vec<$t> = (0..size.0)
                            .into_iter()
                            .map(|idx| {
                                let prod_i: Vec<$t> = (0..size.1)
                                    .into_iter()
                                    .map(|inner_idx| {
                                        m[inner_idx][idx] * v[inner_idx]
                                    })
                                    .collect();

                                prod_i.iter().sum()
                            })
                            .collect();

                        Vertex::<$t>::new(prod.as_slice())
                    } else {
                        panic!(
                            "Cannot multiply vector with matrix when height of matrix and length of vector does not match"
                            );
                    }
                } else {
                    panic!(
                        "Cannot multiply vector with matrix when vector is transposed"
                        );
                }
            }
        }
    )*)
}

impl_mul_with_vector! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
