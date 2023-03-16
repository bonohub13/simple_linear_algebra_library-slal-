#[derive(thiserror::Error, Debug, PartialEq)]
pub enum SlalError<T> {
    #[error("Failed to initialize slal::matrix::Matrix: {0}")]
    MatrixInitializationError(String),
    #[error("Failed to update slal::matrix::Matrix: {0}")]
    MatrixUpdateError(String),
    #[error("Length of two vectors {0} and {1} does not match")]
    UnmatchingVertexLength(String, String),
    #[error("Both vertices ({0}, {1}) are transposed {2}")]
    BothVerticesTransposed(String, String, String),
    #[error("Length of vertex {0} and width of matrix {1} does not match {2}")]
    VertexLengthAndMatrixWidthNotMatch(String, String, String),
    #[error("Length of vertex {0} and height of matrix {1} does not match {2}")]
    VertexLengthAndMatrixHeightNotMatch(String, String, String),
    #[error("{0}")]
    VertexStateError(String),
    #[error("Size of two matrices {0} and {1} does not match")]
    UnmatchingMatrixSize(String, String),
    #[error("Matrix {0} is not a square matrix ({1}, {2}).")]
    NotSquareMatrix(String, String, String),
    #[error("Empty Matrix: {0}")]
    EmptyMatrix(String),
    #[error("Cannot perform computation of triangular matrix for matrix {:?}.", 0)]
    TriangularMatrixNotExist(crate::matrix::Matrix<T>),
}

pub type SlalErr<V, T> = Result<V, SlalError<T>>;
