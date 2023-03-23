macro_rules! impl_dot_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Matrix<$t>> for $t {
            type Output = super::Matrix<$t>;

            fn mul(self, other: Self::Output) -> Self::Output {
                let rv: Vec<$t> = (0..(other.size[0] * other.size[1]))
                    .into_iter()
                    .map(|ij| other.m[ij] * self)
                    .collect();

                Self::Output {
                    m: rv,
                    size: other.size,
                }
            }
        }

        impl crate::linear::Dot<super::Matrix<$t>> for $t {
            type Output = super::Matrix<$t>;

            fn dot(&self, other: &Self::Output) -> Self::Output {
                *self * *other
            }
        }
    )*)
}

impl_dot_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_dot_with_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<$t> for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn mul(self, other: $t) -> Self::Output {
                other * self
            }
        }

        impl crate::linear::Dot<$t> for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn dot(&self, other: &$t) -> Self::Output {
                *other * *self
            }
        }
    )*)
}

impl_dot_with_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_dot_vertex {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Matrix<$t>> for crate::vertex::Vertex<$t> {
            type Output = crate::vertex::Vertex<$t>;

            fn mul(self, other: super::Matrix<$t>) -> Self::Output {
                let m_size = other.size();
                if self.is_transposed() {
                    panic!("Cannot multiply transposed vector with Matrix.");
                } else if self.len() != m_size.1 {
                    panic!("Length of vector and height of matrix must match.");
                }

                let rv_vec: Vec<$t> = (0..m_size.0)
                    .into_iter()
                    .map(|idx| {
                        (0..m_size.1)
                            .into_iter()
                            .map(|inner_idx| {
                                self[inner_idx] * other[inner_idx][idx]
                            })
                            .sum()
                    })
                    .collect();

                crate::vertex::Vertex::<$t>::new(rv_vec.as_slice())
            }
        }

        impl crate::linear::Dot<super::Matrix<$t>> for crate::vertex::Vertex<$t> {
            type Output = crate::error::SlalErr<Self, $t>;

            fn dot(&self, other: &super::Matrix<$t>) -> Self::Output {
                use crate::error::SlalError;
                use crate::vertex::Vertex;

                let m_size = other.size();
                if self.is_transposed() {
                    return Err(SlalError::VertexStateError(format!(
                        "Vertex must not be transposed when computing product of vertex {:?} and matrix {:?}",
                        *self,
                        *other,
                    )))
                } else if self.len() != m_size.1 {
                    return Err(SlalError::VertexLengthAndMatrixHeightNotMatch(
                        format!("{:?}", *self),
                        format!("{:?}", *other),
                        String::from("while computing product of vertex and matrix"),
                    ));
                }

                let rv_vec: Vec<$t> = (0..m_size.0)
                    .into_iter()
                    .map(|idx| {
                        (0..m_size.1)
                            .into_iter()
                            .map(|inner_idx| {
                                self[inner_idx] * other[inner_idx][idx]
                            })
                            .sum()
                    })
                    .collect();

                Ok(Vertex::<$t>::new(rv_vec.as_slice()))
            }
        }
    )*)
}

impl_dot_vertex! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_dot_with_vertex {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<crate::vertex::Vertex<$t>> for super::Matrix<$t> {
            type Output = crate::vertex::Vertex<$t>;

            fn mul(self, other: Self::Output) -> Self::Output {
                use crate::vertex::Vertex;

                let m_size = self.size();
                if !other.is_transposed() {
                    panic!("Vertex not transposed while multiplication of matrix and vertex");
                } else if m_size.0 != other.len() {
                    panic!("Vertex length does not match the width of matrix while in multiplication of matrix and vertex");
                }

                let rv_vec: Vec<$t> = (0..m_size.1)
                    .into_iter()
                    .map(|j| {
                        self[j]
                            .iter()
                            .enumerate()
                            .map(|(idx, m_ij)| *m_ij * other[idx])
                            .sum()
                    })
                    .collect();
                let mut rv = Vertex::<$t>::new(rv_vec.as_slice());

                rv.t();

                rv
            }
        }

        impl crate::linear::Dot<crate::vertex::Vertex<$t>> for super::Matrix<$t> {
            type Output = crate::error::SlalErr<crate::vertex::Vertex<$t>, $t>;

            fn dot(&self, other: &crate::vertex::Vertex<$t>) -> Self::Output {
                use crate::vertex::Vertex;
                use crate::error::SlalError;

                let m_size = self.size();
                if !other.is_transposed() {
                    return Err(SlalError::VertexStateError(format!(
                        "Vertex must be transposed when computing product of matrix {:?} and vertex {:?}",
                        *self,
                        *other,
                    )));
                } else if m_size.0 != other.len() {
                    return Err(SlalError::VertexLengthAndMatrixWidthNotMatch(
                        format!("{:?}", *other),
                        format!("{:?}", *self),
                        String::from("while computing product of matrix and vertex"),
                    ));
                }

                let rv_vec: Vec<$t> = (0..m_size.1)
                    .into_iter()
                    .map(|j| {
                        self[j]
                            .iter()
                            .enumerate()
                            .map(|(idx, m_ij)| *m_ij * other[idx])
                            .sum()
                    })
                    .collect();
                let mut rv = Vertex::<$t>::new(rv_vec.as_slice());

                rv.t();

                Ok(rv)
            }
        }
    )*)
}

impl_dot_with_vertex! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_dot_matrix {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Matrix<$t>> for super::Matrix<$t> {
            type Output = Self;

            fn mul(self, other: Self::Output) -> Self::Output {
                let m_size = self.size();
                let n_size = other.size();
                if m_size.0 != n_size.1 {
                    panic!(
                        "Width of matrix {:?} and height of matrix {:?} must match while computing product of two matrices",
                        self,
                        other
                    );
                }

                let mut rv: Vec<$t> = Vec::with_capacity(n_size.0 * m_size.1);
                (0..m_size.1).for_each(|j| (0..n_size.0).for_each(|i| {
                    rv.push((0..m_size.0)
                        .into_iter()
                        .map(|idx| self[j][idx] * other[idx][i])
                        .sum()
                    );
                }));

                Self::Output {
                    m: rv,
                    size: [n_size.0, m_size.1]
                }
            }
        }

        impl crate::linear::Dot<super::Matrix<$t>> for super::Matrix<$t> {
            type Output = crate::error::SlalErr<super::Matrix<$t>, $t>;

            fn dot(&self, other: &Self) -> Self::Output {
                use crate::error::SlalError;

                let self_size = self.size();
                let other_size = other.size();
                if self_size.0 != other_size.1 {
                    return Err(SlalError::UnmatchingMatrixSize(
                            format!("{:?}", *self),
                            format!("{:?}", *other),
                    ))
                }

                let mut rv: Vec<$t> = Vec::with_capacity(self_size.1 * other_size.0);
                (0..self_size.1).for_each(|j| (0..other_size.0).for_each(|i| {
                    rv.push(
                        (0..self_size.0)
                        .into_iter()
                        .map(|idx| self[j][idx] * other[idx][i])
                        .sum()
                    );
                }));

                Ok(Self {
                    m: rv,
                    size: [other_size.0, self_size.1],
                })
            }
        }
    )*)
}

impl_dot_matrix! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
