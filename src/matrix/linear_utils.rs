macro_rules! impl_triangular_matrix {
    ($($t:ty)*) => ($(
        impl crate::linear::TriangularMatrix for super::Matrix<$t> {
            fn is_upper_triangular(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                let m_vec = self.to_vec();
                for j in 0..size.1 {
                    for i in (j+1)..size.0 {
                        if m_vec[j][i] != 0 as $t {
                            return false;
                        }
                    }
                }

                true
            }
            fn is_lower_triangular(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                let m_vec = self.to_vec();
                for i in 0..size.0 {
                    for j in (i+1)..size.1 {
                        if m_vec[j][i] != 0 as $t {
                            return false;
                        }
                    }
                }

                true
            }
        }
    )*)
}

impl_triangular_matrix! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_diagonal_matrix {
    ($($t:ty)*) => ($(
        impl crate::linear::DiagonalMatrix<$t> for super::Matrix<$t> {
            fn diagonal(diagonal: &[$t]) -> super::Matrix<$t> {
                let m_vec: Vec<Vec<$t>> = (0..diagonal.len())
                    .into_iter()
                    .map(|j| {
                        (0..diagonal.len())
                            .into_iter()
                            .map(|i| {
                                if i == j {
                                    diagonal[i]
                                } else {
                                    0 as $t
                                }
                            })
                            .collect::<Vec<$t>>()
                    })
                    .collect();

                super::Matrix {
                    m: m_vec,
                    size: [diagonal.len(), diagonal.len()],
                }
            }

            fn is_diagonal(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                let m_vec = self.to_vec();
                let zero = 0 as $t;
                for j in 0..size.1 {
                    for i in j..size.0 {
                        if i == j {
                            continue;
                        }

                        if m_vec[j][i] != zero || m_vec[i][j] != zero {
                            return false;
                        }
                    }
                }

                true
            }
        }
    )*)
}

impl_diagonal_matrix! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

// Add a method to compute triangular matrix of a matrix with size (2, 2) and above
// Necessary for computing determinant of matrix with size (4, 4) and above
macro_rules! impl_determinant {
    ($($t:ty)*) => ($(
        impl crate::linear::Determinant for super::Matrix<$t> {
            type Output = $t;

            fn det(&self) -> crate::error::SlalErr<Self::Output> {
                use crate::error::SlalError;
                use crate::linear::TriangularMatrix;

                let size = self.size();

                if size.0 != size.1 {
                    return Err(SlalError::NotSquareMatrix(
                        format!("{:?}", *self),
                        format!("{}", size.0),
                        format!("{}", size.1)
                    ))
                }

                let m_vec = self.to_vec();
                if self.is_upper_triangular() || self.is_lower_triangular() {
                    let mut rv: $t = 1 as $t;

                    (0..size.0).for_each(|idx| rv *= m_vec[idx][idx]);

                    return Ok(rv)
                }

                match size {
                    (0, 0) => Err(SlalError::EmptyMatrix(String::from(
                        "Cannot caluculate determinant for empty matrix"
                    ))),
                    (1, 1) => Ok(m_vec[0][0]),
                    (2, 2) => {
                        let rv = m_vec[0][0] * m_vec[1][1] - m_vec[1][0] * m_vec[0][1];

                        Ok(rv)
                    }
                    (3, 3) => {
                        let m_1 = m_vec[0][0] * (m_vec[1][1] * m_vec[2][2] - m_vec[2][1] * m_vec[1][2]);
                        let m_2 = m_vec[1][0] * (m_vec[0][1] * m_vec[2][2] - m_vec[2][1] * m_vec[0][2]);
                        let m_3 = m_vec[2][0] * (m_vec[0][1] * m_vec[1][2] - m_vec[1][1] * m_vec[0][2]);

                        Ok(m_1 - m_2 + m_3)
                    }
                    _ => {
                        /* TODO:
                            1. Add a method to compute triangular matrix
                            2. Compute the determinant based on the triangular matrix
                        */
                        todo!()
                    },
                }
            }
        }
    )*)
}

impl_determinant! { i8 i16 i32 i64 i128 isize f32 f64 }
