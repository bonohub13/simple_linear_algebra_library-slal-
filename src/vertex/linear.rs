macro_rules! impl_dot_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<$t> for super::Vertex<$t> {
            type Output = Self;

            fn mul(self, other: $t) -> super::Vertex<$t> {
                use super::Vertex;

                let rv_vec: Vec<$t> = self
                    .to_vec()
                    .iter()
                    .map(|v_i| *v_i * other)
                    .collect();

                let rv = if self.is_transposed() {
                    let mut rv = Vertex::<$t>::new(rv_vec.as_slice());

                    rv.t();

                    rv
                } else {
                    Vertex::<$t>::new(rv_vec.as_slice())
                };

                rv
            }
        }

        impl crate::linear::Dot<$t> for super::Vertex<$t> {
            type Output = super::Vertex<$t>;

            fn dot(&self, other: &$t) -> <Self as crate::linear::Dot<$t>>::Output {
                use super::Vertex;

                let rv_vec: Vec<$t> = self
                    .to_vec()
                    .iter()
                    .map(|v_i| *v_i * other)
                    .collect();

                if self.is_transposed() {
                    let mut rv = Vertex::<$t>::new(rv_vec.as_slice());

                    rv.t();

                    rv
                } else {
                    Vertex::<$t>::new(rv_vec.as_slice())
                }
            }
        }
    )*)
}

impl_dot_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_dot_with_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Vertex<$t>> for $t {
            type Output = super::Vertex<$t>;

            fn mul(self, other: super::Vertex<$t>) -> super::Vertex<$t> {
                other * self
            }
        }

        impl crate::linear::Dot<super::Vertex<$t>> for $t {
            type Output = super::Vertex<$t>;

            fn dot(&self, other: &super::Vertex<$t>) -> <Self as crate::linear::Dot<super::Vertex<$t>>>::Output {
                other.dot(self)
            }
        }
    )*)
}

impl_dot_with_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_dot_vertex {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Vertex<$t>> for super::Vertex<$t> {
            type Output = crate::matrix::Matrix<$t>;

            fn mul(self, other: super::Vertex<$t>) -> Self::Output {
                use crate::error::SlalError;
                use crate::matrix::Matrix;
                use std::iter::zip;

                if self.len() != other.len() {
                    panic!(
                        "{}",
                        SlalError::UnmatchingVertexLength(
                            format!("{:?}", self),
                            format!("{:?}", other)
                        )
                    );
                } else if self.is_transposed() == other.is_transposed() {
                    panic!("{}", SlalError::BothVerticesTransposed(
                            format!("{:?}", self),
                            format!("{:?}", other),
                            String::from("while calculating product")
                    ));
                }

                if self.is_transposed() && !other.is_transposed() {
                    let rv_vec: Vec<Vec<$t>> = self
                        .to_vec()
                        .iter()
                        .map(|v_i| {
                            let rv_i = *v_i * other.clone();

                            rv_i.to_vec()
                        })
                        .collect();
                    let rv: Vec<&[$t]> = rv_vec
                        .iter()
                        .map(|rv_i| rv_i.as_slice())
                        .collect();

                    Matrix::<$t>::new(rv.as_slice()).unwrap()
                } else {
                    let rv: $t = zip(self.to_vec(), other.to_vec())
                        .map(|(v_i, w_i)| v_i * w_i)
                        .sum();

                    Matrix::<$t>::new(&[&[rv]]).unwrap()
                }
            }
        }

        impl crate::linear::Dot<super::Vertex<$t>> for super::Vertex<$t> {
            type Output = crate::error::SlalErr<crate::matrix::Matrix<$t>>;

            fn dot(&self, other: &Self) -> <Self as crate::linear::Dot<super::Vertex<$t>>>::Output {
                use crate::error::SlalError;
                use crate::matrix::Matrix;
                use std::iter::zip;

                if self.len() != other.len() {
                    return Err(SlalError::UnmatchingVertexLength(
                        format!("{:?}", self),
                        format!("{:?}", other)
                    ));
                } else if self.is_transposed() == other.is_transposed() {
                    return Err(SlalError::BothVerticesTransposed(
                        format!("{:?}", self),
                        format!("{:?}", other),
                        String::from("while calculating product")
                    ))
                }

                if self.is_transposed() && !other.is_transposed() {
                    let rv_vec: Vec<Vec<$t>> = self
                        .to_vec()
                        .iter()
                        .map(|v_i| {
                            let rv_i = *v_i * other.clone();

                            rv_i.to_vec()
                        })
                        .collect();
                    let rv: Vec<&[$t]> = rv_vec
                        .iter()
                        .map(|rv_i| rv_i.as_slice())
                        .collect();

                    Matrix::<$t>::new(rv.as_slice())
                } else {
                    let rv: $t = zip(self.to_vec(), other.to_vec())
                        .map(|(v_i, w_i)| v_i * w_i)
                        .sum();

                    Matrix::<$t>::new(&[&[rv]])
                }
            }
        }
    )*)
}

impl_dot_vertex! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
