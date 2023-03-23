macro_rules! impl_mul_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<$t> for super::Vertex<$t> {
            type Output = Self;

            fn mul(self, other: $t) -> super::Vertex<$t> {
                let rv_vec: Vec<$t> = (0..self.len())
                    .into_iter()
                    .map(|i| self[i] * other)
                    .collect();

                if self.is_transposed() {
                    Self::Output {
                        v: rv_vec,
                        vertical: true,
                    }
                } else {
                    Self::Output {
                        v: rv_vec,
                        vertical: false,
                    }
                }
            }
        }

        impl crate::linear::Dot<$t> for super::Vertex<$t> {
            type Output = super::Vertex<$t>;

            fn dot(&self, other: &$t) -> <Self as crate::linear::Dot<$t>>::Output {
                *self * *other
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
                use crate::matrix::Matrix;
                use std::iter::zip;

                if self.len() != other.len() {
                    panic!("Length of two vectors must match.");
                } else if self.is_transposed() == other.is_transposed() {
                    if self.is_transposed() {
                        panic!("Cannot multiply vectors that are both vertical.");
                    } else {
                        panic!("Cannot multiply vectors that are both horizontal.")
                    }
                }

                if self.is_transposed() {
                    let rv_vec: Vec<Vec<$t>> = (0..self.len())
                        .into_iter()
                        .map(|i| {
                            let rv_i = self[i] * other;

                            rv_i.to_vec()
                        })
                        .collect();
                    let rv: Vec<&[$t]> = rv_vec
                        .iter()
                        .map(|rv_i| rv_i.as_slice())
                        .collect();

                    match Matrix::<$t>::new(rv.as_slice()) {
                        Ok(matrix) => return matrix,
                        Err(e) => panic!("{}", e),
                    };
                } else {
                    let rv: $t = zip(self.v, other.v)
                        .map(|(v_i, w_i)| v_i * w_i)
                        .sum();

                    match Matrix::<$t>::new(&[&[rv]]) {
                        Ok(matrix) => return matrix,
                        Err(e) => panic!("{}", e),
                    };
                }
            }
        }

        impl crate::linear::Dot<super::Vertex<$t>> for super::Vertex<$t> {
            type Output = crate::error::SlalErr<$t, $t>;

            fn dot(&self, other: &Self) -> <Self as crate::linear::Dot<super::Vertex<$t>>>::Output {
                use crate::error::SlalError;
                use std::iter::zip;

                if self.len() != other.len() {
                    return Err(SlalError::UnmatchingVertexLength(
                        format!("{:?}", *self),
                        format!("{:?}", *other)
                    ));
                }
                if self.is_transposed() || !other.is_transposed() {
                    return Err(SlalError::VertexStateError(format!(
                        "{:?}", *self,
                    )));
                }

                let rv: $t = zip(self.v, other.v)
                    .map(|(v_i, w_i)| v_i * w_i)
                    .sum();

                Ok(rv)
            }
        }

        impl crate::linear::Cross<super::Vertex<$t>> for super::Vertex<$t> {
            type Output = crate::error::SlalErr<super::Vertex<$t>, $t>;

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

                if !self.is_transposed() || other.is_transposed() {
                    return Err(SlalError::VertexStateError(format!(
                        "{:?}", *self,
                    )));
                }

                let rv: Vec<$t> = (0..self_len)
                    .into_iter()
                    .map(|idx| {
                        self[(idx + 1) % self_len] * other[(idx + 2) % self_len]
                        - self[(idx + 2) % self_len] * other[(idx + 1) % self_len]
                    })
                    .collect();

                Ok(Vertex::new(rv.as_slice()))
            }
        }
    )*)
}

impl_mul_vertex! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
