use super::Matrix;
use crate::vertex::Vertex;

#[test]
fn add() {
    let m = Matrix::<u8>::new(&[&[1, 2, 3], &[11, 12, 13]]).unwrap();
    let n = Matrix::<u8>::new(&[&[21, 22, 23], &[31, 32, 33]]).unwrap();

    assert_eq!(m + n, Matrix::new(&[&[22, 24, 26], &[42, 44, 46]]).unwrap());
}

#[test]
#[should_panic]
fn add_invalid() {
    let m = Matrix::<u16>::new(&[&[1, 2, 3], &[11, 12, 13]]).unwrap();
    let n = Matrix::<u16>::new(&[&[21, 22, 23], &[31, 32, 33], &[41, 42, 43]]).unwrap();

    let _ = m + n;
}

#[test]
fn sub() {
    let m = Matrix::<i8>::new(&[&[1, 2, 3], &[11, 12, 13]]).unwrap();
    let n = Matrix::<i8>::new(&[&[21, 22, 23], &[31, 32, 33]]).unwrap();

    assert_eq!(
        m - n,
        Matrix::new(&[&[-20, -20, -20], &[-20, -20, -20]]).unwrap()
    );
}

#[test]
#[should_panic]
fn sub_invalid() {
    let m = Matrix::<i16>::new(&[&[1, 2, 3], &[11, 12, 13]]).unwrap();
    let n = Matrix::<i16>::new(&[&[21, 22, 23], &[31, 32, 33], &[41, 42, 43]]).unwrap();

    let _ = m - n;
}

#[test]
fn mul_scala() {
    let m = Matrix::<u32>::new(&[&[1, 2], &[4, 8]]).unwrap();

    assert_eq!(3 * m, Matrix::new(&[&[3, 6], &[12, 24]]).unwrap());
}

#[test]
fn mul_with_scala() {
    let m = Matrix::new(&[&[1, 2], &[4, 8]]).unwrap();

    assert_eq!(m * -3, Matrix::new(&[&[-3, -6], &[-12, -24]]).unwrap());
}

#[test]
fn mul_vector() {
    let m = Matrix::<i64>::new(&[&[16, 32, 64], &[3, 9, 27]]).unwrap();
    let mut v = Vertex::<i64>::new(&[25, 125, 625]);
    let mut ans = Vertex::new(&[16 * 25 + 32 * 125 + 64 * 625, 3 * 25 + 9 * 125 + 27 * 625]);

    v.t();
    ans.t();

    assert_eq!(m * v, ans);
}

#[test]
#[should_panic]
fn mul_vector_invalid_vector_not_transposed() {
    let m = Matrix::<i64>::new(&[&[16, 32, 64], &[3, 9, 27]]).unwrap();
    let v = Vertex::<i64>::new(&[25, 125, 625]);

    let _ = m * v;
}

#[test]
#[should_panic]
fn mul_vector_invalid_vector_size_invalid() {
    let m = Matrix::<i64>::new(&[&[16, 32, 64], &[3, 9, 27]]).unwrap();
    let mut v = Vertex::<i64>::new(&[25, 125, 625, 3125]);

    v.t();

    let _ = m * v;
}

#[test]
fn mul_with_vector() {
    let m = Matrix::<i64>::new(&[&[16, 32, 64], &[3, 9, 27]]).unwrap();
    let v = Vertex::<i64>::new(&[25, 125]);
    let ans = Vertex::new(&[16 * 25 + 3 * 125, 32 * 25 + 9 * 125, 64 * 25 + 27 * 125]);

    assert_eq!(v * m, ans);
}

#[test]
#[should_panic]
fn mul_with_vector_invalid_vector_not_transposed() {
    let m = Matrix::<i64>::new(&[&[16, 32, 64], &[3, 9, 27]]).unwrap();
    let mut v = Vertex::<i64>::new(&[25, 125]);

    v.t();

    let _ = v * m;
}

#[test]
#[should_panic]
fn mul_with_vector_invalid_vector_size_invalid() {
    let m = Matrix::<i64>::new(&[&[16, 32, 64], &[3, 9, 27]]).unwrap();
    let v = Vertex::<i64>::new(&[25, 125, 625]);

    let _ = v * m;
}
