macro_rules! impl_triangular_matrix {
    ($($t:ty)*) => ($(
        impl crate::linear::IsTriangularMatrix for super::Matrix<$t> {
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
