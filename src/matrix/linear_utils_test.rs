use super::Matrix;
use crate::error::SlalErr;
use crate::linear::{Cofactor, Determinant, DiagonalMatrix, Inverse, Normalize, TriangularMatrix};

#[test]
fn is_upper_triangular() {
    let m =
        Matrix::<i8>::new(&[&[1, 2, 3, 4], &[0, 5, 6, 7], &[0, 0, 8, 9], &[0, 0, 0, 10]]).unwrap();

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
        Matrix::<u16>::new(&[&[1, 0, 0, 0], &[2, 3, 0, 0], &[4, 5, 6, 0], &[7, 8, 9, 10]]).unwrap();

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

#[test]
fn lower_triangular() {
    let m = Matrix::<f32>::new(&[
        &[1., 2., 3., 4.],
        &[1.5, 3.5, 5., 6.5],
        &[2., 4.5, 6.5, 8.5],
        &[2.5, 5.5, 8., 10.5],
    ])
    .unwrap();

    match m.lower_triangular() {
        Ok(l) => {
            println!("{:?}", l);

            assert!(l.is_lower_triangular());
        }
        Err(err) => {
            println!("{:?}", err);

            assert!(false);
        }
    }
}

#[test]
fn lower_triangular_not_square() {
    let m = Matrix::new(&[
        &[1., 2., 3., 4.],
        &[5., 6., 7., 8.],
        &[9., 10., 11., 12.],
        &[13., 14., 15., 16.],
        &[17., 18., 19., 20.],
    ])
    .unwrap();

    match m.lower_triangular() {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn lower_triangular_noncomputable() {
    let m = Matrix::<i8>::new(&[
        &[0, 1, 2, 3],
        &[3, 4, 5, 7],
        &[8, 9, 10, 11],
        &[12, 13, 14, 15],
    ])
    .unwrap();

    match m.lower_triangular() {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn upper_triangular() {
    let m = Matrix::<u8>::new(&[
        &[1, 2, 3, 4],
        &[2, 5, 7, 9],
        &[3, 7, 10, 13],
        &[4, 9, 13, 17],
    ])
    .unwrap();

    match m.upper_triangular() {
        Ok(u) => {
            println!("{:?}", u);

            assert!(u.is_upper_triangular());
        }
        Err(err) => {
            println!("{:?}", err);

            assert!(false);
        }
    }
}

#[test]
fn upper_triangular_not_square() {
    let m = Matrix::<i16>::new(&[
        &[1, 2, 3, 4],
        &[5, 6, 7, 8],
        &[9, 10, 11, 12],
        &[13, 14, 15, 16],
        &[17, 18, 19, 20],
    ])
    .unwrap();

    match m.upper_triangular() {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn upper_triangular_noncomputable() {
    let m = Matrix::<u16>::new(&[
        &[0, 1, 2, 3],
        &[3, 4, 5, 7],
        &[8, 9, 10, 11],
        &[12, 13, 14, 15],
    ])
    .unwrap();

    match m.upper_triangular() {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn determinant_2d() {
    let m = Matrix::new(&[&[1, 2], &[3, 4]]).unwrap();

    match m.det() {
        Ok(det) => assert!(det == -2.),
        Err(_) => assert!(false),
    }
}

#[test]
fn determinant_3d() {
    let m = Matrix::<u32>::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]).unwrap();

    match m.det() {
        Ok(det) => {
            let excepted = ((45 - 48) - 4 * (18 - 24) + 7 * (12 - 15)) as f64;

            assert!(det == excepted);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn determinant_4d_and_above() {
    let m = Matrix::<f32>::new(&[
        &[1., 2., 3., 4.],
        &[4., 5., 6., 7.],
        &[7., 8., 9., 10.],
        &[10., 11., 12., 13.],
    ])
    .unwrap();

    let triangular = m.upper_triangular().unwrap();
    let triangular_vec = triangular.to_vec();

    match m.det() {
        Ok(det) => {
            let mut excepted = 1.;
            let size = triangular.size().1;
            (0..size).for_each(|i| excepted *= triangular_vec[i][i]);

            assert!(det == excepted);
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn determinant_triangular() {
    let m = Matrix::new(&[&[1., 2.], &[0., 2.]]).unwrap();

    match m.det() {
        Ok(det) => assert!(det == 2.),
        Err(_) => assert!(false),
    }
}

#[test]
fn determinant_1d() {
    let m = Matrix::<i8>::new(&[&[9]]).unwrap();

    match m.det() {
        Ok(det) => assert!(det == 9.),
        Err(_) => assert!(false),
    }
}

#[test]
fn determinant_empty() {
    let m = Matrix::<u8>::empty();

    match m.det() {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn determinant_not_square() {
    let m = Matrix::<i16>::new(&[&[1], &[2]]).unwrap();

    assert!(m.det().is_err());
}

#[test]
fn cofactor_1d() {
    let m = Matrix::<u16> {
        m: vec![1],
        size: [1, 1],
    };

    match m.cofactor() {
        Ok(cofactor) => assert!(
            cofactor
                == Matrix::<f64> {
                    m: vec![1.],
                    size: [1, 1],
                }
        ),
        Err(_) => assert!(false),
    }
}

#[test]
fn cofactor_2d() {
    let m = Matrix::<i32> {
        m: vec![1, 2, 3, 4],
        size: [2, 2],
    };

    match m.cofactor() {
        Ok(cofactor) => assert!(
            cofactor
                == Matrix::<f64> {
                    m: vec![1., -3., -2., 4.],
                    size: [2, 2],
                }
        ),
        Err(_) => assert!(false),
    }
}

#[test]
fn cofactor_3d() {
    let m = Matrix::<u32> {
        m: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        size: [3, 3],
    };

    match m.cofactor() {
        Ok(cofactor) => {
            assert!(
                cofactor
                    == Matrix::<f64> {
                        m: vec![
                            (5 * 9 - 6 * 8) as f64,
                            -(4 * 9 - 6 * 7) as f64,
                            (4 * 8 - 5 * 7) as f64,
                            -(2 * 9 - 3 * 8) as f64,
                            (1 * 9 - 3 * 7) as f64,
                            -(1 * 8 - 2 * 7) as f64,
                            (2 * 6 - 3 * 5) as f64,
                            -(1 * 6 - 3 * 4) as f64,
                            (1 * 5 - 2 * 4) as f64,
                        ],
                        size: [3, 3],
                    }
            )
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn cofactor_4d() -> SlalErr<(), f32> {
    let m = Matrix::<f32> {
        m: vec![
            100., 110., 120., 130., 200., 220., 240., 260., 300., 330., 360., 390., 400., 440.,
            480., 520.,
        ],
        size: [4, 4],
    };

    match m.cofactor() {
        Ok(cofactor) => {
            let c_11 = Matrix::<f32> {
                m: vec![220., 240., 260., 330., 360., 390., 440., 480., 520.],
                size: [3, 3],
            }
            .det()?;
            let c_12 = Matrix::<f32> {
                m: vec![200., 240., 260., 300., 360., 390., 400., 480., 520.],
                size: [3, 3],
            }
            .det()?;
            let c_13 = Matrix::<f32> {
                m: vec![200., 220., 260., 300., 330., 390., 400., 440., 520.],
                size: [3, 3],
            }
            .det()?;
            let c_14 = Matrix::<f32> {
                m: vec![200., 220., 240., 300., 330., 360., 400., 440., 480.],
                size: [3, 3],
            }
            .det()?;
            let c_21 = Matrix::<f32> {
                m: vec![110., 120., 130., 330., 360., 390., 440., 480., 520.],
                size: [3, 3],
            }
            .det()?;
            let c_22 = Matrix::<f32> {
                m: vec![100., 120., 130., 300., 360., 390., 400., 480., 520.],
                size: [3, 3],
            }
            .det()?;
            let c_23 = Matrix::<f32> {
                m: vec![100., 110., 130., 300., 330., 390., 400., 440., 520.],
                size: [3, 3],
            }
            .det()?;
            let c_24 = Matrix::<f32> {
                m: vec![100., 110., 120., 300., 330., 360., 400., 440., 480.],
                size: [3, 3],
            }
            .det()?;
            let c_31 = Matrix::<f32> {
                m: vec![110., 120., 130., 220., 240., 260., 440., 480., 520.],
                size: [3, 3],
            }
            .det()?;
            let c_32 = Matrix::<f32> {
                m: vec![100., 120., 130., 200., 240., 260., 400., 480., 520.],
                size: [3, 3],
            }
            .det()?;
            let c_33 = Matrix::<f32> {
                m: vec![100., 110., 130., 200., 220., 260., 400., 440., 520.],
                size: [3, 3],
            }
            .det()?;
            let c_34 = Matrix::<f32> {
                m: vec![100., 110., 120., 200., 220., 240., 400., 440., 480.],
                size: [3, 3],
            }
            .det()?;
            let c_41 = Matrix::<f32> {
                m: vec![110., 120., 130., 220., 240., 260., 330., 360., 390.],
                size: [3, 3],
            }
            .det()?;
            let c_42 = Matrix::<f32> {
                m: vec![100., 120., 130., 200., 240., 260., 300., 360., 390.],
                size: [3, 3],
            }
            .det()?;
            let c_43 = Matrix::<f32> {
                m: vec![100., 110., 130., 200., 220., 260., 300., 330., 390.],
                size: [3, 3],
            }
            .det()?;
            let c_44 = Matrix::<f32> {
                m: vec![100., 110., 120., 200., 220., 240., 300., 330., 360.],
                size: [3, 3],
            }
            .det()?;
            Ok(assert!(
                cofactor
                    == Matrix::<f64> {
                        m: vec![
                            c_11, -c_12, c_13, -c_14, -c_21, c_22, -c_23, c_24, c_31, -c_32, c_33,
                            -c_34, -c_41, c_42, -c_43, c_44,
                        ],
                        size: [4, 4],
                    }
            ))
        }
        Err(_) => Ok(assert!(false)),
    }
}

#[test]
fn cofactor_not_square() {
    let m = Matrix::<f64> {
        m: vec![1., 2., 3., 1., 4., 9., 1., 16., 81., 1., 32., 241.],
        size: [3, 4],
    };

    assert!(m.cofactor().is_err())
}

#[test]
fn cofactor_empty() {
    let m = Matrix::<i8>::empty();

    assert!(m.cofactor().is_err())
}

#[test]
fn cofactor_determinant_not_exist() {
    let m = Matrix::<u8> {
        m: vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24,
        ],
        size: [5, 5],
    };

    assert!(m.cofactor().is_err())
}

#[test]
fn inverse() {
    let m = Matrix::<i16> {
        m: vec![1, 2, 3, 4],
        size: [2, 2],
    };

    match m.inverse() {
        Ok(_) => assert!(true),
        Err(err) => {
            println!("{:?}", err);
            assert!(false)
        }
    }
}

#[test]
fn inverse_not_square() {
    let m = Matrix::<u16> {
        m: vec![1, 2, 3, 4],
        size: [1, 4],
    };

    match m.inverse() {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn inverse_determinant_zero_value() {
    let m = Matrix::<i32> {
        m: vec![0, 1, 0, 3],
        size: [2, 2],
    };

    match m.inverse() {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn norm() {
    let m = Matrix::<u32> {
        m: vec![1, 2, 3, 4, 5, 6],
        size: [2, 3],
    };
    let norm = [
        (1..=2)
            .into_iter()
            .map(|x| (x as f64).powi(2))
            .sum::<f64>()
            .sqrt(),
        (3..=4)
            .into_iter()
            .map(|x| (x as f64).powi(2))
            .sum::<f64>()
            .sqrt(),
        (5..=6)
            .into_iter()
            .map(|x| (x as f64).powi(2))
            .sum::<f64>()
            .sqrt(),
    ];

    let mut expected_m = Matrix::<f64>::from(m.clone());
    expected_m.m.iter_mut().enumerate().for_each(|(idx, m_ji)| {
        *m_ji = *m_ji / norm[idx / 2];
    });

    assert!(m.norm() == expected_m)
}
