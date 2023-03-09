use super::Vertex;
use crate::linear::{Cross, Dot};
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
    let v1: Vertex<i32> = Vertex::new(&[1, 2, 3]);
    let v2: Vertex<i32> = Vertex::new(&[4, 5, 6]);

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

    let _ = v1 * v2;
}

#[test]
fn dot_scala() {
    let v = Vertex::<u64>::new(&[1, 2, 3]);
    let scala: u64 = 3;

    assert!(scala.dot(&v) == Vertex::new(&[3, 6, 9]))
}

#[test]
fn dot_with_scala() {
    let v = Vertex::<i128>::new(&[1, 2, 3]);
    let scala: i128 = -4;

    assert!(v.dot(&scala) == Vertex::new(&[-4, -8, -12]))
}

#[test]
fn dot_vertex() {
    let v = Vertex::<u128>::new(&[1, 2, 3]);
    let w = Vertex::<u128>::new(&[1, 4, 9]);

    assert!(v.dot(&w) == Ok(1 + 8 + 27))
}

#[test]
fn dot_vertex_invalid() {
    let v = Vertex::<isize>::new(&[1, 2, 3]);
    let w = Vertex::<isize>::new(&[1, 4, 27, 64]);

    assert!(match v.dot(&w) {
        Ok(_) => false,
        Err(_) => true,
    })
}

#[test]
fn cross() {
    let v = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);
    let w = Vertex::<f32>::new(&[0.1, 0.8, 2.7]);

    assert!(
        v.cross(&w)
            == Ok(Vertex::new(&[
                2.0 * 2.7 - 3.0 * 0.8,
                3.0 * 0.1 - 1.0 * 2.7,
                1.0 * 0.8 - 2.0 * 0.1
            ]))
    )
}

#[test]
fn cross_invalid() {
    let v = Vertex::<f64>::new(&[0.0, 1.0, 2.0, 3.0]);
    let w = Vertex::<f64>::new(&[0.1, 0.8, 2.7]);

    assert!(match v.cross(&w) {
        Ok(_) => false,
        Err(_) => true,
    })
}
