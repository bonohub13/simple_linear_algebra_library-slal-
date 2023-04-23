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

    /**
    Calculates cross product of vector/matrix

    # Example
    ```
    use slal::linear::Cross;
    use slal::vertex::Vertex;

    let mut v = Vertex::new(&[1, 2, 3]);
    let w = Vertex::new(&[1, 4, 9]);

    v.t();

    assert!(v.cross(&w) == Ok(Vertex::<i32>::new(&[
        2 * 9 - 3 * 4,
        3 * 1 - 1 * 9,
        1 * 4 - 2 * 1,
    ])));
    ```
     */
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
    Checks if matrix is triangular matrix
*/
pub trait TriangularMatrix {
    type Output;

    /**
    Checks if matrix is lower triangular matrix

    # Example
    ```
    use slal::linear::TriangularMatrix;
    use slal::matrix::Matrix;

    let m = Matrix::new(&[&[1, 0, 0], &[2, 3, 0], &[4, 5, 6]]).unwrap();

    assert!(m.is_lower_triangular())
    ```
     */
    fn is_lower_triangular(&self) -> bool;

    /**
    Checks if matrix is lower triangular matrix

    # Example
    ```
    use slal::linear::TriangularMatrix;
    use slal::matrix::Matrix;

    let m = Matrix::new(&[&[1, 0, 0], &[2, 3, 0], &[4, 5, 6]]).unwrap();

    assert!(m.is_lower_triangular())
    ```
     */
    fn is_upper_triangular(&self) -> bool;

    /**
    Computes upper triangular matrix from square matrix

    # Example
    ```
    use slal::linear::TriangularMatrix;
    use slal::matrix::Matrix;

    let m = Matrix::<i32>::new(&[&[1, 0, 0], &[2, 3, 0], &[4, 5, 6]]).unwrap();
    let u: Matrix<f64> = m.upper_triangular().unwrap();

    assert!(u.is_upper_triangular())
    ```
     */
    fn upper_triangular(&self) -> Self::Output;

    /**
    Computes lower triangular matrix from matrix

    # Example
    ```
    use slal::linear::TriangularMatrix;
    use slal::matrix::Matrix;

    let m = Matrix::<u32>::new(&[&[1, 0, 0], &[2, 3, 0], &[4, 5, 6]]).unwrap();
    let l: Matrix<f64> = m.lower_triangular().unwrap();

    assert!(l.is_lower_triangular())
    ```
     */
    fn lower_triangular(&self) -> Self::Output;
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

pub trait Determinant<T>: TriangularMatrix {
    /**
    Calculates the determinant of a matrix with size (n, n)

    # Example
    ```
    use slal::linear::Determinant;
    use slal::matrix::Matrix;

    let m = Matrix::new(&[&[1, 2], &[3, 4]]).unwrap();

    assert!(m.det() == Ok(-2.0));
    ```
     */
    fn det(&self) -> crate::error::SlalErr<f64, T>;
}

pub trait Inverse<T>: Cofactor<T> {
    /**
    Calculates the inverse of a matrix
     */
    fn inverse(&self) -> crate::error::SlalErr<Self::Output, T>;
}

pub trait Cofactor<T> {
    type Output;

    /**
    Calculates the matrix of cofactors of a matrix
     */
    fn cofactor(&self) -> crate::error::SlalErr<Self::Output, T>;
}

pub trait Random {
    type Output;
    type Size;

    /**
    Outputs a vector/matrix with random values with specified size.
     */
    fn rand(size: Self::Size) -> Self::Output;
    /**
    Outputs a transposed vector/matrix with random values with specified size.
     */
    fn rand_transposed(size: Self::Size) -> Self::Output;
}

pub trait Normalize {
    type Output;

    /**
    Normalizes vector/matrix
     */
    fn norm(&self) -> Self::Output;
}

pub trait Eigen {
    type Output;

    /**
    Computes eigenvector of matrix
     */
    fn eigen(
        &self,
    ) -> crate::error::SlalErr<(crate::vertex::Vertex<Self::Output>, Self::Output), Self::Output>;
}
