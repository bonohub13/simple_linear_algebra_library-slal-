fn upper_triangular(m: &super::Matrix<f64>) -> crate::error::SlalErr<super::Matrix<f64>, f64> {
    use crate::error::SlalError;

    // Ignore anything below this value and treat it as 0.0
    const DELTA: f64 = 1e-10;

    // Assumes all matrices are square matrix
    let size = m.size();

    // Using Doolittle's method to compute upper triangular matrix
    let mut l: Vec<f64> = Vec::with_capacity(size.0 * size.1);
    let mut u: Vec<f64> = Vec::with_capacity(size.0 * size.1);

    for j in 0..size.1 {
        for i in 0..size.0 {
            if i < j {
                // lower triangular matrix
                u.push(0.);
                l.push(if i == 0 {
                    m[j][0] / m[0][0]
                } else {
                    (m[j][i]
                        - (0..(i - 1))
                            .into_iter()
                            .map(|i_| l[j * size.1 + i_] * u[i_ * size.1 + i])
                            .sum::<f64>())
                        / u[(j * size.1 - 1) + (i - 1)]
                });
                // Treat values smaller or equal to DELTA as 0.0
                if l[j * size.1 + i].abs() <= DELTA {
                    l[j * size.1 + i] = 0.;
                }
            } else if i == j {
                // for diagonal lines
                l.push(1.);
                u.push(if j == 0 {
                    m[0][i]
                } else {
                    m[j][i]
                        - (0..(i - 1))
                            .into_iter()
                            .map(|i_| l[j * size.1 + i_] * u[i_ * size.1 + i])
                            .sum::<f64>()
                });
                // If any value of diagonal line in upper triangular matrix is
                // 0 (below or including DELTA), computation of triangular matrix
                // is impossible
                if u[j * size.1 + i].abs() <= DELTA {
                    println!("{:?}", u);

                    return Err(SlalError::TriangularMatrixNotExist(m.clone()));
                }
            } else if i > j {
                // upper triangular matrix
                l.push(0.);
                u.push(if j == 0 {
                    m[0][i]
                } else {
                    m[j][i]
                        - (0..(i - 1))
                            .into_iter()
                            .map(|i_| l[j * size.1 + i_] * u[i_ * size.1 + j])
                            .sum::<f64>()
                });
                // Treat values smaller or equal to DELTA as 0.0
                if u[j * size.1 + i].abs() <= DELTA {
                    u[j * size.1 + i] = 0.;
                }
            }
        }
    }

    Ok(super::Matrix::<f64> {
        m: u,
        size: [size.0, size.1],
    })
}

macro_rules! impl_triangular_matrix {
    ($($t:ty)*) => ($(
        impl crate::linear::TriangularMatrix for super::Matrix<$t> {
            type Output = crate::error::SlalErr<super::Matrix<f64>, f64>;

            fn is_lower_triangular(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                for j in 0..size.1 {
                    for i in (j+1)..size.0 {
                        if self[j][i] != 0 as $t {
                            return false;
                        }
                    }
                }

                true
            }

            fn is_upper_triangular(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                for j in 0..size.1 {
                    for i in 0..j {
                        if self[j][i] != 0 as $t {
                            return false;
                        }
                    }
                }

                true
            }

            fn lower_triangular(&self) -> Self::Output {
                let mut l = match self.upper_triangular() {
                    Ok(l_matrix) => l_matrix,
                    Err(err) => return Err(err),
                };

                l.t();

                Ok(l)
            }

            fn upper_triangular(&self) -> Self::Output {
                use crate::error::SlalError;

                let size = self.size();

                if size.0 != size.1 {
                    return Err(SlalError::NotSquareMatrix(
                            format!("{:?}", *self),
                            format!("{}", size.0),
                            format!("{}", size.1),
                    ));
                }

                let m: super::Matrix<f64> = super::Matrix::from(self.clone());

                upper_triangular(&m)
            }
        }
    )*)
}

impl_triangular_matrix! { i8 u8 i16 u16 i32 u32 f32 f64 }

macro_rules! impl_diagonal_matrix {
    ($($t:ty)*) => ($(
        impl crate::linear::DiagonalMatrix<$t> for super::Matrix<$t> {
            fn diagonal(diagonal: &[$t]) -> super::Matrix<$t> {
                let mut m: Vec<$t> = Vec::with_capacity(diagonal.len().pow(2));
                (0..diagonal.len()).for_each(|j| {
                    (0..diagonal.len()).for_each(|i| {
                        if i == j {
                            m.push(diagonal[i]);
                        } else {
                            m.push(0 as $t);
                        }
                    })
                });

                super::Matrix {
                    m: m,
                    size: [diagonal.len(), diagonal.len()],
                }
            }

            fn is_diagonal(&self) -> bool {
                let size = self.size();

                if size.0 != size.1 {
                    return false;
                }

                let zero = 0 as $t;
                for j in 0..size.1 {
                    for i in j..size.0 {
                        if i == j {
                            continue;
                        }

                        if self[j][i] != zero || self[i][j] != zero {
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
        impl crate::linear::Determinant<$t> for super::Matrix<$t> {
            fn det(&self) -> crate::error::SlalErr<f64, $t> {
                use crate::error::SlalError;
                use crate::linear::TriangularMatrix;

                let size = self.size();

                if self.is_empty() {
                    return Err(SlalError::EmptyMatrix(format!("{:?}", self.clone())));
                }

                if size.0 != size.1 {
                    return Err(SlalError::NotSquareMatrix(
                        format!("{:?}", *self),
                        format!("{}", size.0),
                        format!("{}", size.1)
                    ))
                }

                if self.is_upper_triangular() || self.is_lower_triangular() {
                    let mut rv: $t = 1 as $t;

                    (0..size.0).for_each(|idx| rv *= self[idx][idx]);

                    return Ok(rv as f64)
                }

                match size {
                    (0, 0) => Err(SlalError::EmptyMatrix(String::from(
                        "Cannot caluculate determinant for empty matrix"
                    ))),
                    (1, 1) => Ok(self[0][0] as f64),
                    (2, 2) => {
                        let rv = (self[0][0] * self[1][1]) as f64 - (self[1][0] * self[0][1]) as f64;

                        Ok(rv)
                    }
                    (3, 3) => {
                        let m_1 = self[0][0] as f64 * ((self[1][1] * self[2][2]) as f64 - (self[2][1] * self[1][2]) as f64);
                        let m_2 = self[1][0] as f64 * ((self[0][1] * self[2][2]) as f64 - (self[2][1] * self[0][2]) as f64);
                        let m_3 = self[2][0] as f64 * ((self[0][1] * self[1][2]) as f64 - (self[1][1] * self[0][2]) as f64);

                        Ok(m_1 - m_2 + m_3)
                    }
                    _ => {
                        let u = match self.upper_triangular() {
                            Ok(m) => m,
                            Err(_) => return Err(SlalError::TriangularMatrixNotExist(
                                self.clone()
                            )),
                        };
                        let mut det = 1.;
                        (0..size.1)
                            .for_each(|ij| {
                                det *= u[ij][ij];
                            });

                        Ok(det)
                    },
                }
            }
        }
    )*)
}

impl_determinant! { i8 u8 i16 u16 i32 u32 f32 f64 }

fn minor<T: Copy>(mtx: &super::Matrix<T>, row: usize, column: usize) -> super::Matrix<T> {
    use super::Matrix;

    // Square matrix check has been done
    let mut minor: Vec<T> = Vec::with_capacity(mtx.size[0] * mtx.size[1]);
    (0..mtx.size[1]).for_each(|j| {
        (0..mtx.size[0]).for_each(|i| {
            if j == row || i == column {
                return;
            }

            minor.push(mtx.m[j * mtx.size[1] + i]);
        })
    });

    Matrix::<T> {
        m: minor.clone(),
        size: [mtx.size[0] - 1, mtx.size[1] - 1],
    }
}

macro_rules! impl_cofactor {
    ($($t:ty)*) => ($(
        impl crate::linear::Cofactor<$t> for super::Matrix<$t> {
            type Output = super::Matrix<f64>;

            fn cofactor(&self) -> crate::error::SlalErr<Self::Output, $t> {
                use crate::error::SlalError;
                use crate::linear::Determinant;

                if self.size[0] != self.size[1] {
                    return Err(SlalError::NotSquareMatrix(
                        format!("{:?}", self.clone()),
                        format!("{}", self.size[0]),
                        format!("{}", self.size[1]),
                    ));
                }

                let mut m: Vec<f64> = Vec::with_capacity(self.size[0] * self.size[1]);
                let gain: f64 = -1.;
                for j in 0..self.size[1] {
                    for i in 0..self.size[0] {
                        let det = match minor(self, j, i).det() {
                            Ok(det) => det,
                            Err(err) => return Err(err),
                        };

                        m.push(gain.powi((2 - ((i + j) % 2)) as i32) * det);
                    }
                }

                Ok(Self::Output { m, size: self.size })
            }
        }
    )*)
}

impl_cofactor! { i8 u8 i16 u16 i32 u32 f32 f64 }
