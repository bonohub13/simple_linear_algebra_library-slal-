use super::Matrix;
use crate::linear::Dot;

#[test]
fn mul_scala() {
    let m = Matrix::<i8>::new(&[&[1, 2, 3], &[-4, -5, -6]]).unwrap();

    assert_eq!(
        -3 * m,
        Matrix::new(&[&[-3, -6, -9], &[12, 15, 18]]).unwrap()
    );
}

#[test]
fn mul_with_scala() {
    let m = Matrix::<u8>::new(&[&[2, 3], &[5, 7]]).unwrap();

    assert_eq!(m * 4, Matrix::new(&[&[8, 12], &[20, 28]]).unwrap());
}

#[test]
fn dot_scala() {
    let m = Matrix::<i16>::new(&[&[1, 1, 2, 3, 5, 8], &[13, 21, 34, 55, 89, 144]]).unwrap();

    assert_eq!(
        m.dot(&2),
        Matrix::new(&[&[2, 2, 4, 6, 10, 16], &[26, 42, 68, 110, 178, 288]]).unwrap()
    );
}

#[test]
fn dot_with_scala() {
    let m = Matrix::<u16>::new(&[&[1, 1, 2, 3, 5, 8], &[13, 21, 34, 55, 89, 144]]).unwrap();

    assert_eq!(
        2.dot(&m),
        Matrix::new(&[&[2, 2, 4, 6, 10, 16], &[26, 42, 68, 110, 178, 288]]).unwrap()
    );
}
