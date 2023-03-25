use super::Matrix;

#[test]
fn add() {
    let m = Matrix::<i8>::new(&[&[1, 2, 3], &[11, 12, 13]]).unwrap();
    let n = Matrix::<i8>::new(&[&[21, 22, 23], &[31, 32, 33]]).unwrap();

    assert_eq!(
        m + n,
        Matrix {
            m: vec![22, 24, 26, 42, 44, 46],
            size: [3, 2]
        }
    );
}

#[test]
#[should_panic]
fn add_invalid() {
    let m = Matrix::<u8>::new(&[&[1, 2, 3], &[11, 12, 13]]).unwrap();
    let n = Matrix::<u8>::new(&[&[21, 22, 23], &[31, 32, 33], &[41, 42, 43]]).unwrap();

    let _ = m + n;
}

#[test]
fn sub() {
    let m = Matrix::<i16>::new(&[&[1, 2, 3], &[11, 12, 13]]).unwrap();
    let n = Matrix::<i16>::new(&[&[21, 22, 23], &[31, 32, 33]]).unwrap();

    assert_eq!(
        m - n,
        Matrix {
            m: vec![-20, -20, -20, -20, -20, -20],
            size: [3, 2]
        }
    );
}

#[test]
#[should_panic]
fn sub_invalid() {
    let m = Matrix::<u16>::new(&[&[1, 2, 3], &[11, 12, 13]]).unwrap();
    let n = Matrix::<u16>::new(&[&[21, 22, 23], &[31, 32, 33], &[41, 42, 43]]).unwrap();

    let _ = m - n;
}
