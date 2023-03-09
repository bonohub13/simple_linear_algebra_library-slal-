/**
    Calculation of dot product for vertices/matrices
*/
pub trait Dot<T>
where
    Self: Sized,
{
    type Output;

    fn dot(&self, other: &T) -> Self::Output;
}

/**
    Calculation of cross product for vertices/matrices
*/
pub trait Cross<T>
where
    Self: Sized,
{
    type Output;

    fn cross(&self, other: &T) -> Self::Output;
}

/**
    Calculation of magnitude for vertex
*/
pub trait Magnitude {
    type Output;

    fn magnitude(&self) -> Self::Output;
}

/**
    Checks if matrix is triangular matrix
*/
pub trait IsTriangularMatrix {
    fn is_upper_triangular(&self) -> bool;
    fn is_lower_triangular(&self) -> bool;
}

pub trait DiagonalMatrix<T> {
    /**
    Creates diagonal matrix from slice

    # Example
    ```
    use slal::matrix::Matrix;
    use slal::linear::DiagonalMatrix;

    let m = Matrix::diagonal(&[1, 2, 3, 4]); // Diagonal matrix of 4x4

    assert!(m == Matrix::new(&[&[1, 0, 0, 0], &[0, 2, 0, 0], &[0, 0, 3, 0], &[0, 0, 0, 4]]).unwrap());
    ```
     */
    fn diagonal(diagonal: &[T]) -> crate::matrix::Matrix<T>;

    /**
    Checks if matrix is diagonal

    # Example
    ```
    use slal::matrix::Matrix;
    use slal::linear::DiagonalMatrix;

    let m = Matrix::new(&[&[1, 0, 0, 0], &[0, -2, 0, 0], &[0, 0, 4, 0], &[0, 0, 0, -8]]).unwrap();

    assert!(m.is_diagonal());
    ```
     */
    fn is_diagonal(&self) -> bool;
}
