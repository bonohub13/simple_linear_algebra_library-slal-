use super::Vertex;
use crate::linear::Magnitude;

#[test]
fn magnitude() {
    let v = Vertex::<usize>::new(&[0, 1, 2, 3]);

    assert!(v.magnitude() == f64::from(1.0 + 4.0 + 9.0).sqrt())
}
