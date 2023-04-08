use super::Matrix;
use crate::error::SlalError;
use crate::linear::Dot;
use crate::vertex::Vertex;

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

#[test]
fn mul_vertex() {
    let v = Vertex::new(&[1, 2, 3]);
    let m = Matrix::new(&[&[1, 4, 9], &[1, 8, 27], &[1, 16, 81]]).unwrap();

    assert_eq!(v * m, Vertex::new(&[1 + 2 + 3, 4 + 16 + 48, 9 + 54 + 243]));
}

#[test]
#[should_panic]
fn mul_vertex_transposed() {
    let mut v = Vertex::<u32>::new(&[1, 2, 3]);
    let m = Matrix::<u32>::new(&[&[1, 4, 9], &[1, 8, 27], &[1, 16, 81]]).unwrap();

    v.t();

    let _ = v * m;
}

#[test]
#[should_panic]
fn mul_vertex_invalid_size() {
    let v = Vertex::<i128>::new(&[1, 2, 3, 4]);
    let m = Matrix::<i128>::new(&[&[1, 4, 9], &[1, 8, 27], &[1, 16, 81]]).unwrap();

    let _ = v * m;
}

#[test]
fn dot_vertex() {
    let v = Vertex::<u128>::new(&[0, 2, 4]);
    let m = Matrix::<u128>::new(&[&[1, 3, 5], &[2, 4, 6], &[3, 5, 7]]).unwrap();

    assert_eq!(
        v.dot(&m),
        Ok(Vertex::new(&[0 + 4 + 12, 0 + 8 + 20, 0 + 12 + 28]))
    );
}

#[test]
fn dot_vertex_transposed() {
    let mut v = Vertex::<isize>::new(&[0, 2, 4]);
    let m = Matrix::<isize>::new(&[&[1, 3, 5], &[2, 4, 6], &[3, 5, 7]]).unwrap();

    v.t();

    assert_eq!(
        v.dot(&m),
        Err(SlalError::VertexStateError(format!(
            "Vertex must not be transposed when computing product of vertex {:?} and matrix {:?}",
            v, m,
        )))
    );
}

#[test]
fn dot_vertex_invalid() {
    let v = Vertex::<usize>::new(&[0, 2, 4, 6]);
    let m = Matrix::<usize>::new(&[&[1, 3, 5], &[2, 4, 6], &[3, 5, 7]]).unwrap();

    assert_eq!(
        v.dot(&m),
        Err(SlalError::VertexLengthAndMatrixHeightNotMatch(
            format!("{:?}", v),
            format!("{:?}", m),
            String::from("while computing product of vertex and matrix"),
        ))
    );
}

#[test]
fn mul_with_vertex() {
    let mut v = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);
    let m = Matrix::<f32>::new(&[&[0.1, 0.4, 0.9], &[0.1, 0.8, 2.7], &[0.1, 1.6, 8.1]]).unwrap();
    v.t();

    let mut prod = Vertex::new(&[
        f32::from(1.0 * 0.1 + 2.0 * 0.4 + 3.0 * 0.9).round(),
        f32::from(1.0 * 0.1 + 2.0 * 0.8 + 3.0 * 2.7).round(),
        f32::from(1.0 * 0.1 + 2.0 * 1.6 + 3.0 * 8.1).round(),
    ]);
    prod.t();

    let mut ans = m * v;
    (0..ans.len()).for_each(|idx| ans[idx] = ans[idx].round());

    assert!(ans == prod);
}

#[test]
#[should_panic]
fn mul_with_vertex_not_transposed() {
    let v = Vertex::<f64>::new(&[1.0, -2.0, 3.0]);
    let m =
        Matrix::<f64>::new(&[&[0.1, -0.4, 0.9], &[-0.1, 0.8, -2.7], &[0.1, -1.6, 8.1]]).unwrap();

    let _ = m * v;
}

#[test]
#[should_panic]
fn mul_with_vertex_invalid_size() {
    let mut v = Vertex::<i8>::new(&[1, 2, 3, 4]);
    let m = Matrix::<i8>::new(&[&[1, 4, 9], &[1, 8, 27], &[1, 16, 81]]).unwrap();
    v.t();

    let _ = m * v;
}

#[test]
fn dot_with_vertex() {
    let mut v = Vertex::<u8>::new(&[0, 2, 4]);
    let m = Matrix::<u8>::new(&[&[1, 3, 5], &[2, 4, 6], &[3, 5, 7]]).unwrap();
    v.t();

    let mut prod = Vertex::new(&[0 + 6 + 20, 0 + 8 + 24, 0 + 10 + 28]);
    prod.t();

    assert_eq!(m.dot(&v), Ok(prod));
}

#[test]
fn dot_with_vertex_not_transposed() {
    let v = Vertex::<i16>::new(&[0, -2, 4]);
    let m = Matrix::<i16>::new(&[&[-1, 3, -5], &[2, -4, 6], &[-3, 5, -7]]).unwrap();

    assert_eq!(
        m.dot(&v),
        Err(SlalError::VertexStateError(format!(
            "Vertex must be transposed when computing product of matrix {:?} and vertex {:?}",
            m, v,
        )))
    );
}

#[test]
fn dot_with_vertex_invalid() {
    let mut v = Vertex::<u16>::new(&[0, 2, 4, 6]);
    let m = Matrix::<u16>::new(&[&[1, 3, 5], &[2, 4, 6], &[3, 5, 7]]).unwrap();
    v.t();

    assert_eq!(
        m.dot(&v),
        Err(SlalError::VertexLengthAndMatrixWidthNotMatch(
            format!("{:?}", v),
            format!("{:?}", m),
            String::from("while computing product of matrix and vertex"),
        ))
    );
}

#[test]
fn mul_matrix() {
    let m = Matrix::new(&[&[1, -2, 3], &[-4, 5, -6], &[7, -8, 9]]).unwrap();
    let n = Matrix::new(&[&[1, -8, 27], &[-64, 125, -216], &[343, -512, 729]]).unwrap();

    assert!(
        m * n
            == Matrix::new(&[
                &[1 + 128 + 1029, -8 + -250 + -1536, 27 + 432 + 2187],
                &[
                    -4 + 5 * -64 + -6 * 343,
                    32 + 5 * 125 + 6 * 512,
                    -4 * 27 + 5 * -216 + -6 * 729
                ],
                &[
                    7 + -8 * -64 + 9 * 343,
                    7 * -8 + -8 * 125 + 9 * -512,
                    7 * 27 + -8 * -216 + 9 * 729,
                ]
            ])
            .unwrap()
    )
}

#[test]
#[should_panic]
fn mul_matrix_invalid() {
    let m = Matrix::<u32>::new(&[&[0, 1], &[2, 3]]).unwrap();
    let n = Matrix::<u32>::new(&[&[0, 1], &[4, 8], &[16, 25]]).unwrap();

    let _ = m * n;
}

#[test]
fn dot_matrix() {
    let m = Matrix::<i64>::new(&[&[1, -2, 3], &[-4, 5, -6], &[7, -8, 9]]).unwrap();
    let n = Matrix::<i64>::new(&[&[1, -8, 27], &[-64, 125, -216], &[343, -512, 729]]).unwrap();

    assert!(match m.dot(&n) {
        Ok(prod) =>
            prod == Matrix::new(&[
                &[1 + 128 + 1029, -8 + -250 + -1536, 27 + 432 + 2187],
                &[
                    -4 + 5 * -64 + -6 * 343,
                    32 + 5 * 125 + 6 * 512,
                    -4 * 27 + 5 * -216 + -6 * 729
                ],
                &[
                    7 + -8 * -64 + 9 * 343,
                    7 * -8 + -8 * 125 + 9 * -512,
                    7 * 27 + -8 * -216 + 9 * 729,
                ]
            ])
            .unwrap(),
        Err(_) => false,
    })
}

#[test]
fn dot_matrix_invalid() {
    let m = Matrix::<u32>::new(&[&[0, 1], &[2, 3]]).unwrap();
    let n = Matrix::<u32>::new(&[&[0, 1], &[4, 8], &[16, 25]]).unwrap();

    assert!(match m.dot(&n) {
        Ok(_) => false,
        Err(_) => true,
    })
}
