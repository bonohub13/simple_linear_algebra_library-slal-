macro_rules! impl_mul_scala {
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

impl_mul_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_mul_with_scala {
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

impl_mul_with_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_mul_vertex {
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
            type Output = crate::error::SlalErr<$t>;

            fn dot(&self, other: &Self) -> <Self as crate::linear::Dot<super::Vertex<$t>>>::Output {
                use crate::error::SlalError;
                use std::iter::zip;

                if self.len() != other.len() {
                    return Err(SlalError::UnmatchingVertexLength(
                        format!("{:?}", *self),
                        format!("{:?}", *other)
                    ));
                }

                let rv: $t = zip(self.to_vec(), other.to_vec())
                    .map(|(v_i, w_i)| v_i * w_i)
                    .sum();

                Ok(rv)
            }
        }

        impl crate::linear::Cross<super::Vertex<$t>> for super::Vertex<$t> {
            type Output = crate::error::SlalErr<super::Vertex<$t>>;

            fn cross(&self, other: &Self) -> Self::Output {
                use super::Vertex;
                use crate::error::SlalError;

                let self_len = self.len();

                if self_len != other.len() {
                    return Err(SlalError::UnmatchingVertexLength(
                        format!("{:?}", *self),
                        format!("{:?}", *other)
                    ))
                }

                let self_vec = self.to_vec();
                let other_vec = other.to_vec();
                let rv: Vec<$t> = (0..self_len)
                    .into_iter()
                    .map(|idx| {
                        self_vec[(idx + 1) % self_len] * other_vec[(idx + 2) % self_len]
                        - self_vec[(idx + 2) % self_len] * other_vec[(idx + 1) % self_len]
                    })
                    .collect();

                Ok(Vertex::new(rv.as_slice()))
            }
        }
    )*)
}

impl_mul_vertex! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_magnitude_vertex {
    ($($t:ty)*) => ($(
        impl crate::linear::Magnitude for super::Vertex<$t> {
            type Output = f64;

            fn magnitude(&self) -> Self::Output {
                self.to_vec()
                    .iter()
                    .map(|v_i| (*v_i * *v_i) as f64)
                    .sum::<f64>()
                    .sqrt()
            }
        }
    )*)
}

impl_magnitude_vertex! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
