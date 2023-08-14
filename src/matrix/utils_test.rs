use super::Matrix;
use crate::utils::{Round, RPY};

fn get_type_name<T>(_: &T) -> &'static str {
    use std::any::type_name;

    format!("{}", type_name::<T>()).as_str()
}

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

// For rpy_<type>_ok(), perform datatype check as well

#[test]
fn rpy_i8_ok() {
    let p0 = Matrix::<i8>::new(&[&[2, 3, 5], &[4, 9, 25], &[8, 27, 125]]).unwrap();

    match p0.rpy([0., f64::to_radians(90.), f64::to_radians(180.)]) {
        Ok(p1) => assert!(get_type_name(&p1) == "slal::matrix::Matrix<f64>"),
        Err(_) => assert!(false),
    }
}

#[test]
fn rpy_i16_ok() {}

#[test]
fn rpy_i32_ok() {}

#[test]
fn rpy_u8_ok() {}

#[test]
fn rpy_u16_ok() {}

#[test]
fn rpy_u32_ok() {}

#[test]
fn rpy_f16_ok() {}

#[test]
fn rpy_f32_ok() {}

#[test]
fn rpy_f64_ok() {}

// No testing for error currently because any error would result from
// slal::linear::Dot, not from slal::utils::RPY

#[test]
fn rpy_i8_err() {}

#[test]
fn rpy_i16_err() {}

#[test]
fn rpy_i32_err() {}

#[test]
fn rpy_u8_err() {}

#[test]
fn rpy_u16_err() {}

#[test]
fn rpy_u32_err() {}

#[test]
fn rpy_f16_err() {}

#[test]
fn rpy_f32_err() {}

#[test]
fn rpy_f64_err() {}
