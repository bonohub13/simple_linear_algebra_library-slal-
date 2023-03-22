use super::Matrix;

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
