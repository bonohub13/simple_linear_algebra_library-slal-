mod convert;
mod linear_arithmatic;
mod linear_utils;
mod math;
mod utils;

#[cfg(test)]
mod linear_arithmatic_test;
#[cfg(test)]
mod linear_utils_test;
#[cfg(test)]
mod math_test;
#[cfg(test)]
mod utils_test;

pub use crate::linear::{Cross, Determinant, DiagonalMatrix, Dot, Magnitude, TriangularMatrix};
pub use linear_arithmatic::*;
pub use linear_utils::*;
pub use math::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    m: Vec<T>,
    size: [usize; 2],
}

impl<T> Matrix<T>
where
    T: Copy + std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div,
{
    const EMPTY_MATRIX_ON_INITIALIZATION: &'static str = "Cannot create matrix of size (0, 0)";
    const EMPTY_HORIZONTAL_VECTOR_ON_INITIALIZATION: &'static str =
        "Cannot create matrix of size (0, y)";
    const DIFFERENT_HORIZONTAL_VECTOR_IN_MATRIX: &'static str =
        "Cannot create matrix with different horizontal size";
    const DIFFERENT_HORIZONTAL_VECTOR_IN_MATRIX_ON_UPDATE: &'static str =
        "Cannot update matrix with different horizontal size";

    /**
    Creates new matrix from slice with size (x, y)

    # Example
    ```
    use slal::matrix::Matrix;

    // Matrix<f32> with size (2, 3)
    //  | 1.0 1.1 |
    //  | 2.0 2.1 |
    //  | 3.0 3.1 |
    let m = Matrix::<f32>::new(&[&[1.0, 1.1], &[2.0, 2.1], &[3.0, 3.1]]).unwrap();
    ```
     */
    pub fn new(matrix: &[&[T]]) -> crate::error::SlalErr<Self, T> {
        use crate::error::SlalError;

        match matrix.len() {
            0 => {
                return Err(SlalError::MatrixInitializationError(String::from(
                    Self::EMPTY_MATRIX_ON_INITIALIZATION,
                )));
            }
            _ => {
                if matrix[0].len() == 0 {
                    return Err(SlalError::MatrixInitializationError(String::from(
                        Self::EMPTY_HORIZONTAL_VECTOR_ON_INITIALIZATION,
                    )));
                }
            }
        }
        let size = (matrix[0].len(), matrix.len());
        let mut m: Vec<T> = Vec::with_capacity(size.0 * size.1);
        let mut previous_horizontal_length: usize = size.0;

        for (idx, v) in matrix.into_iter().enumerate() {
            if idx > 0 {
                if previous_horizontal_length != v.len() {
                    return Err(SlalError::MatrixInitializationError(String::from(
                        Self::DIFFERENT_HORIZONTAL_VECTOR_IN_MATRIX,
                    )));
                }

                previous_horizontal_length = v.len();
            }

            v.into_iter().for_each(|v_i| m.push(*v_i));
        }

        Ok(Self {
            size: [size.0, size.1],
            m,
        })
    }

    /**
    Creates new transposed matrix from slice with size (x, y)

    # Example
    ```
    use slal::matrix::Matrix;

    // Matrix<f32> with size (3, 2)
    //  | 1.0 2.0 3.0 |
    //  | 1.1 2.1 3.1 |
    let m = Matrix::<f32>::new_transposed(&[&[1.0, 1.1], &[2.0, 2.1], &[3.0, 3.1]]).unwrap();
    ```
     */
    pub fn new_transposed(matrix: &[&[T]]) -> crate::error::SlalErr<Self, T> {
        let mut m = Self::new(matrix)?;

        m.t();

        Ok(m)
    }

    /**
    Creates new empty matrix with size (0, 0)

    # Example
    ```
    use slal::matrix::Matrix;

    // Matrix<f32> with size (0, 0)
    //  | |
    let m = Matrix::<f32>::empty();
    ```
     */
    pub fn empty() -> Self {
        Self {
            m: vec![],
            size: [0, 0],
        }
    }

    /**
    Checks if matrix is empty

    # Example
    ```
    use slal::matrix::Matrix;

    let m = Matrix::<f32>::new(&[&[0.1, 1.1], &[2.1, 3.2]]).unwrap();

    if !m.is_empty() {
        println!("{:?} is not empty.", m)
    }
    ```
     */
    pub fn is_empty(&self) -> bool {
        self.size == [0, 0] && self.m.is_empty()
    }

    /**
    Checks if matrix is a square matrix

    # Example
    ```
    use slal::matrix::Matrix;

    let m = Matrix::new(&[&[1, 2], &[0, 3]]).unwrap();

    if m.is_square_matrix() {
        println!("{:?} is a square matrix!", m);
    }
    ```
     */
    pub fn is_square_matrix(&self) -> bool {
        self.size[0] == self.size[1] && !self.is_empty()
    }

    /**
    Resets matrix with new matrix.

    # Examples
    ```
    use slal::matrix::Matrix;

    // Matrix<i32>
    let mut m = Matrix::new(&[&[1, 2, 3], &[2, 3, 4], &[3, 4, 5]]).unwrap();
    // Still Matrix<i32>
    m.set_matrix(&[&[0, 2, 4], &[3, 9, 27], &[25, 125, 625]]).unwrap();
    ```
     */
    pub fn set_matrix(&mut self, matrix: &[&[T]]) -> crate::error::SlalErr<(), T> {
        use crate::error::SlalError;

        let size = [matrix[0].len(), matrix.len()];
        let mut m: Vec<T> = Vec::with_capacity(size[0] * size[1]);
        let mut past_size: usize = size[0];
        for (matrix_idx, v) in matrix.into_iter().enumerate() {
            if matrix_idx > 0 {
                if past_size != v.len() {
                    return Err(SlalError::MatrixUpdateError(
                        Self::DIFFERENT_HORIZONTAL_VECTOR_IN_MATRIX_ON_UPDATE.to_string(),
                    ));
                }

                past_size = v.len();
            }

            v.iter().for_each(|v_i| m.push(*v_i));
        }

        self.m = m;
        self.size = size;

        Ok(())
    }

    /**
    Returns the size of matrix

    # Example
    ```
    use slal::matrix::Matrix;

    let m = Matrix::new(&[&[1.0, 2.0]]).unwrap();
    let (m_hor, m_vert) = m.size();     // returns (2, 1)
    ```
     */
    pub fn size(&self) -> (usize, usize) {
        (self.size[0], self.size[1])
    }

    /**
    Transposes matrix (x, y) into (y, x)

    # Example
    ```
    use slal::matrix::Matrix;

    // Matrix of
    //  | 1 2 3 |
    //  | 2 3 4 |
    let mut m = Matrix::new(&[&[1, 2, 3], &[2, 3, 4]]).unwrap();
    // This transposes the matrix m into
    //  | 1 2 |
    //  | 2 3 |
    //  | 3 4 |
    m.t();
    ```
     */
    pub fn t(&mut self) {
        let mut m: Vec<T> = Vec::with_capacity(self.size[0] * self.size[1]);
        (0..self.size[0]).for_each(|i| {
            (0..self.size[1]).for_each(|j| {
                m.push(self.m[j * self.size[0] + i]);
            })
        });

        self.m = m;
        self.size = [self.size[1], self.size[0]];
    }

    /**
    Returns a square vector (`Vec<Vec<T>>`) of matrix.

    # Example
    ```
    use slal::matrix::Matrix;

    // Matrix<i32>
    let m = Matrix::new(&[&[1, 2], &[2, 3]]).unwrap();
    // m_vec -> Vec<Vec<i32>>
    let m_vec = m.to_vec();
    ```
     */
    pub fn to_vec(&self) -> Vec<Vec<T>> {
        (0..self.size[1])
            .into_iter()
            .map(|j| {
                (0..self.size[0])
                    .into_iter()
                    .map(|i| self.m[j * self.size[1] + i])
                    .collect()
            })
            .collect()
    }

    /**
    Returns a string (&str) of type name

    # Example
    ```
    use slal::matrix::Matrix;

    let m = Matrix::<f32>::new(&[&[1.0, 2.0, 3.0], &[0.1, 0.2, 0.3]]).unwrap();
    // Prints "type name of m: slal::matrix::Matrix<f32>"
    //  Maybe useful for debugging calculating product of matrices with different types.
    println!("type name of m: {}", m.type_name());
    ```
     */
    pub fn type_name(&self) -> &str {
        use std::any::type_name;

        type_name::<Matrix<T>>()
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;
    use crate::error::SlalError;

    #[test]
    fn new() {
        let m = Matrix::new(&[&[1, 2], &[3, 4], &[5, 6]]).unwrap();

        assert_eq!(
            m,
            Matrix {
                m: vec![1, 2, 3, 4, 5, 6],
                size: [2, 3],
            }
        );
    }

    #[test]
    fn new_fail_empty() {
        let m = Matrix::<i32>::new(&[]);

        assert_eq!(
            m,
            Err(SlalError::MatrixInitializationError(
                Matrix::<i8>::EMPTY_MATRIX_ON_INITIALIZATION.to_owned(),
            ))
        );
    }

    #[test]
    fn new_fail_empty_horizontal() {
        let m = Matrix::<i32>::new(&[&[]]);

        assert_eq!(
            m,
            Err(SlalError::MatrixInitializationError(
                Matrix::<i8>::EMPTY_HORIZONTAL_VECTOR_ON_INITIALIZATION.to_owned()
            ))
        );
    }

    #[test]
    fn new_fail_different_horizontal_size() {
        let m = Matrix::new(&[&[1, 2], &[3]]);

        assert_eq!(
            m,
            Err(SlalError::MatrixInitializationError(
                Matrix::<i8>::DIFFERENT_HORIZONTAL_VECTOR_IN_MATRIX.to_owned()
            ))
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            Matrix::<u32>::empty(),
            Matrix::<u32> {
                m: vec![],
                size: [0, 0]
            }
        );
    }

    #[test]
    fn is_empty() {
        assert!(Matrix::<i8>::empty().is_empty());
    }

    #[test]
    fn is_square_matrix() {
        assert!(Matrix::<u8>::new(&[&[1, 2], &[0, 3]])
            .unwrap()
            .is_square_matrix())
    }

    #[test]
    fn not_empty() {
        let m = Matrix::new(&[&[1, 0], &[2, 3]]).unwrap();

        assert!(!m.is_empty());
    }

    #[test]
    fn set_matrix_valid() {
        let mut m = Matrix::new(&[&[1, 2, 3]]).unwrap();

        assert_eq!(m.set_matrix(&[&[1, 2, 3], &[2, 3, 4], &[3, 4, 5]]), Ok(()));
    }

    #[test]
    fn set_matrix() {
        let mut m = Matrix::new(&[&[1, 2]]).unwrap();
        m.set_matrix(&[&[2, 3], &[3, 4]]).unwrap();

        assert_eq!(
            m,
            Matrix::<i32> {
                m: vec![2, 3, 3, 4],
                size: [2, 2]
            }
        );
    }

    #[test]
    fn set_matrix_invalid() {
        let mut m = Matrix::new(&[&[1, 2, 3]]).unwrap();

        assert_eq!(
            m.set_matrix(&[&[1], &[21, 22]]),
            Err(SlalError::MatrixUpdateError(
                Matrix::<i32>::DIFFERENT_HORIZONTAL_VECTOR_IN_MATRIX_ON_UPDATE.to_string()
            ))
        );
    }

    #[test]
    fn size() {
        let m = Matrix::new(&[&[1, 2], &[3, 4], &[5, 6]]).unwrap();

        assert_eq!(m.size(), (2, 3));
    }

    #[test]
    fn transpose() {
        let mut m = Matrix::new(&[&[1, 2, 3], &[1, 4, 9]]).unwrap();
        m.t();

        assert_eq!(
            m,
            Matrix::<i32> {
                m: vec![1, 1, 2, 4, 3, 9],
                size: [2, 3],
            }
        );
    }

    #[test]
    fn to_vec() {
        let m = Matrix::new(&[&[1, 2], &[3, 4]]).unwrap();

        assert_eq!(m.to_vec(), vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn type_name() {
        let m = Matrix::<usize>::new(&[&[1]]).unwrap();

        assert_eq!(m.type_name(), "slal::matrix::Matrix<usize>");
    }

    #[test]
    fn new_transposed() -> crate::error::SlalErr<(), f32> {
        let m_t = Matrix::<f32>::new_transposed(&[&[1., 2., 3.], &[0.1, 0.2, 0.3]])?;

        Ok(assert!(
            m_t == Matrix::<f32> {
                m: vec![1., 0.1, 2., 0.2, 3., 0.3],
                size: [2, 3],
            }
        ))
    }
}
