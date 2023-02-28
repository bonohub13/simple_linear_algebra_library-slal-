#[derive(thiserror::Error, Debug, PartialEq)]
pub enum SlalError {
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
}

pub type SlalErr<V> = Result<V, SlalError>;
