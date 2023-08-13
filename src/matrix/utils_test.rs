use super::Matrix;
use crate::utils::Round;

#[test]
fn index() {
    let m: Matrix<i8> = Matrix::new(&[&[1, 2], &[-3, -4]]).unwrap();

    assert!(m[1][1] == -4);
}

#[test]
fn index_mut() {
    let mut m: Matrix<u8> = Matrix::new(&[&[1, 2, 3], &[1, 4, 9], &[1, 8, 27]]).unwrap();

    m[1][2] = 25;

    assert!(m[1][2] == 25);
}

#[test]
fn index_ref() {
    let m: Matrix<i16> = Matrix::new(&[&[1, -2], &[-1, 4]]).unwrap();

    assert!((&m)[1][1] == 4);
}

#[test]
fn index_mutref() {
    let mut m: Matrix<u16> = Matrix::new(&[&[2, 3], &[4, 9], &[8, 27]]).unwrap();

    assert!((&mut m)[2][0] == 8);
}

#[test]
fn index_mut_mutref() {
    let mut m: Matrix<i32> = Matrix::new(&[&[-2, -3, -5], &[4, 9, 25], &[-8, -27, -125]]).unwrap();

    (&mut m)[2][2] = 125;

    assert!((&mut m)[2][2] == 125);
}

#[test]
fn round() {
    let mut m = Matrix::<f64> {
        m: vec![
            1.00000000000001,
            2.00000000000002,
            3.00000000000003,
            4.00000000000004,
        ],
        size: [2, 2],
    };

    m.round();

    assert!(
        m == Matrix::<f64> {
            m: vec![1., 2., 3., 4.],
            size: [2, 2],
        }
    )
}

#[test]
fn rpy_ok() {}

#[test]
fn rpy_err() {}
