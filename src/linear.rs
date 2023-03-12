/**
    Calculation of dot product for vertices/matrices
*/
pub trait Dot<T>
where
    Self: Sized,
{
    type Output;

    /**
    Calculates dot product of vector/matrix

    # Example
    ```
    use slal::linear::Dot;
    use slal::vertex::Vertex;

    let v = Vertex::new(&[1, 2, 3]);
    let mut w = Vertex::new(&[1, 4, 9]);

    w.t();

    assert!(v.dot(&w) == Ok(36));
    ```
     */
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

    /**
    Computes magnitude of vectors;

    # Example
    ```
    use slal::linear::Magnitude;
    use slal::vertex::Vertex;

    let v = Vertex::new(&[1, 2, 3]);

    assert!(v.magnitude() == (14.0 as f64).sqrt());
    ```
    */
    fn magnitude(&self) -> Self::Output;
}

/**
    Checks if matrix is triangle matrix
*/
pub trait TriangleMatrix {
    type Output;

    /**
    Checks if matrix is lower triangle matrix

    # Example
    ```
    use slal::linear::TriangleMatrix;
    use slal::matrix::Matrix;

    let m = Matrix::new(&[&[1, 2, 3], &[0, 4, 5], &[0, 0, 6]]).unwrap();

    assert!(m.is_lower_triangle())
    ```
     */
    fn is_lower_triangle(&self) -> bool;

    /**
    Checks if matrix is lower triangle matrix

    # Example
    ```
    use slal::linear::TriangleMatrix;
    use slal::matrix::Matrix;

    let m = Matrix::new(&[&[1, 0, 0], &[2, 3, 0], &[4, 5, 6]]).unwrap();

    assert!(m.is_lower_triangle())
    ```
     */
    fn is_upper_triangle(&self) -> bool;
    fn lower_triangle(&self) -> Self::Output;
    fn upper_triangle(&self) -> Self::Output;
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

pub trait Determinant {
    type Output;

    /**
    Calculates the determinant of a matrix with size (n, n)

    # Example
    ```
    use slal::linear::Determinant;
    use slal::matrix::Matrix;

    let m = Matrix::new(&[&[1, 2], &[3, 4]]).unwrap();

    assert!(m.det() == Ok(-2));
    ```
     */
    fn det(&self) -> crate::error::SlalErr<Self::Output>;
}
