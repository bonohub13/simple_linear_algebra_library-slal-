#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    m: Vec<Vec<T>>,
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
    pub fn new(matrix: &[&[T]]) -> crate::error::SlalErr<Self> {
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
        let mut m: Vec<Vec<T>> = vec![vec![]; matrix.len()];
        let mut previous_horizontal_length: usize = 0;

        for (idx, v) in matrix.into_iter().enumerate() {
            if idx == 0 {
                previous_horizontal_length = v.len();
            } else if previous_horizontal_length != v.len() {
                return Err(SlalError::MatrixInitializationError(String::from(
                    Self::DIFFERENT_HORIZONTAL_VECTOR_IN_MATRIX,
                )));
            }
            m[idx] = v.to_vec();
        }

        Ok(Self {
            size: [previous_horizontal_length, m.len()],
            m,
        })
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
        self.size == [0, 0]
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
                m: vec![vec![1, 2], vec![3, 4], vec![5, 6]],
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
    fn not_empty() {
        let m = Matrix::new(&[&[1, 0], &[2, 3]]).unwrap();

        assert!(!m.is_empty());
    }

    #[test]
    fn size() {
        let m = Matrix::new(&[&[1, 2], &[3, 4], &[5, 6]]).unwrap();

        assert_eq!(m.size(), (2, 3));
    }
}
