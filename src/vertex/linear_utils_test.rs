use super::Vertex;
use crate::linear::{InnerProduct, Magnitude, Normalize};
use crate::utils::Round;

#[test]
fn magnitude() {
    let v = Vertex::<usize>::new(&[0, 1, 2, 3]);

    assert!(v.magnitude() == f64::from(1.0 + 4.0 + 9.0).sqrt())
}

#[test]
fn round() {
    let mut v = Vertex::<f32> {
        v: vec![1.0000000000000001, 2.0000000000000002, 3.0000000000000003],
        vertical: false,
    };

    v.round();

    assert!(
        v == Vertex::<f32> {
            v: vec![1., 2., 3.],
            vertical: false,
        }
    )
}

#[test]
fn norm() {
    let norm = (1..=3)
        .into_iter()
        .map(|v| (v as f64).powi(2))
        .sum::<f64>()
        .sqrt();
    let v = Vertex::<f64> {
        v: vec![1., 2., 3.],
        vertical: false,
    };

    assert!(v.norm() == v * (1. / norm))
}

#[test]
fn inner() {
    let v = Vertex::<i8> {
        v: vec![1, 2, 3, 4],
        vertical: false,
    };

    assert!(v.inner() == 1 + 4 + 9 + 16)
}
