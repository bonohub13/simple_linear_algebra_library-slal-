use super::Vertex;
use crate::matrix::Matrix;

#[test]
fn mul_scala() {
    let v = Vertex::<i8>::new(&[1, 2, 3]);

    assert_eq!(-3 * v, Vertex::<i8>::new(&[-3, -6, -9]));
}

#[test]
fn mul_with_scala() {
    let v = Vertex::<u8>::new(&[1, 2, 3]);

    assert_eq!(v * 3, Vertex::new(&[3, 6, 9]));
}

#[test]
fn mul() {
    let v1 = Vertex::<i16>::new(&[1, 2, 3]);
    let mut v2 = Vertex::<i16>::new(&[4, 5, 6]);

    v2.t();

    assert_eq!(v1 * v2, Matrix::new(&[&[32]]).unwrap());
}

#[test]
fn mul_trasposed() {
    let v1 = Vertex::<u16>::new(&[1, 2, 3]);
    let mut v2 = Vertex::<u16>::new(&[4, 5, 6]);

    v2.t();

    assert_eq!(
        v2 * v1,
        Matrix::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
    );
}

#[test]
#[should_panic]
fn mul_none_transposed() {
    let v1 = Vertex::new(&[1, 2, 3]);
    let v2 = Vertex::new(&[4, 5, 6]);

    let _ = v1 * v2;
}

#[test]
#[should_panic]
fn mul_both_transposed() {
    let mut v1 = Vertex::<u32>::new(&[1, 2, 3]);
    let mut v2 = Vertex::<u32>::new(&[4, 5, 6]);

    v1.t();
    v2.t();

    let _ = v1 * v2;
}

#[test]
#[should_panic]
fn mul_non_matching_size() {
    let mut v1 = Vertex::<i64>::new(&[1, 2, 3]);
    let v2 = Vertex::<i64>::new(&[4, 5, 6, 7]);

    v1.t();

    let _ = v2 * v1;
}
