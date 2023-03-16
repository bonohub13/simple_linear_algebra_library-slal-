use super::Matrix;
use crate::linear::{DiagonalMatrix, TriangularMatrix};

#[test]
fn is_upper_triangular() {
    let m =
        Matrix::<i8>::new(&[&[1, 0, 0, 0], &[2, 3, 0, 0], &[4, 5, 6, 0], &[7, 8, 9, 10]]).unwrap();

    assert!(m.is_upper_triangular());
}

#[test]
fn not_upper_triangular_size() {
    let m = Matrix::<u8>::new(&[&[1, 0, 0, 0], &[2, 3, 0, 0], &[4, 5, 6, 0]]).unwrap();

    assert!(!m.is_upper_triangular());
}

#[test]
fn not_upper_triangular() {
    let m =
        Matrix::<i16>::new(&[&[1, 0, 0, 1], &[2, 3, 2, 0], &[4, 5, 6, 0], &[7, 8, 9, 10]]).unwrap();

    assert!(!m.is_upper_triangular());
}

#[test]
fn is_lower_triangular() {
    let m =
        Matrix::<u16>::new(&[&[1, 2, 3, 4], &[0, 5, 6, 7], &[0, 0, 8, 9], &[0, 0, 0, 10]]).unwrap();

    assert!(m.is_lower_triangular());
}

#[test]
fn not_lower_triangular_size() {
    let m = Matrix::<u16>::new(&[&[1, 2, 3, 4], &[0, 5, 6, 7], &[0, 0, 8, 9]]).unwrap();

    assert!(!m.is_lower_triangular());
}

#[test]
fn not_lower_triangular() {
    let m =
        Matrix::<u16>::new(&[&[1, 2, 3, 4], &[0, 5, 6, 7], &[0, 1, 8, 9], &[0, 2, 3, 10]]).unwrap();

    assert!(!m.is_lower_triangular());
}

#[test]
fn diagonal_matrix() {
    let m = Matrix::diagonal(&[-2, 4, -8, 16]);

    assert!(
        m == Matrix::new(&[
            &[-2, 0, 0, 0],
            &[0, 4, 0, 0],
            &[0, 0, -8, 0],
            &[0, 0, 0, 16]
        ])
        .unwrap()
    );
}

#[test]
fn is_diagonal() {
    let m = Matrix::<u32>::new(&[&[1, 0], &[0, 2]]).unwrap();

    assert!(m.is_diagonal());
}

#[test]
fn not_diagonal() {
    let m = Matrix::<i64>::new(&[&[0x1fffffff, 0x000000], &[0x2fffffff, 0x3fffffff]]).unwrap();

    assert!(!m.is_diagonal());
}
