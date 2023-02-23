use crate::vertex::Vertex;

#[test]
fn add_i8() {
    let v = Vertex::<i8>::new(&[1, 2, 3]);
    let w = Vertex::<i8>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<i8>::new(&[5, 7, 9]));
}

#[test]
fn add_u8() {
    let v = Vertex::<u8>::new(&[1, 2, 3]);
    let w = Vertex::<u8>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<u8>::new(&[5, 7, 9]));
}

#[test]
fn add_i16() {
    let v = Vertex::<i16>::new(&[1, 2, 3]);
    let w = Vertex::<i16>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<i16>::new(&[5, 7, 9]));
}

#[test]
fn add_u16() {
    let v = Vertex::<u16>::new(&[1, 2, 3]);
    let w = Vertex::<u16>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<u16>::new(&[5, 7, 9]));
}

#[test]
fn add_i32() {
    let v = Vertex::<i32>::new(&[1, 2, 3]);
    let w = Vertex::<i32>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<i32>::new(&[5, 7, 9]));
}

#[test]
fn add_u32() {
    let v = Vertex::<u32>::new(&[1, 2, 3]);
    let w = Vertex::<u32>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<u32>::new(&[5, 7, 9]));
}

#[test]
fn add_i64() {
    let v = Vertex::<i64>::new(&[1, 2, 3]);
    let w = Vertex::<i64>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<i64>::new(&[5, 7, 9]));
}

#[test]
fn add_u64() {
    let v = Vertex::<u64>::new(&[1, 2, 3]);
    let w = Vertex::<u64>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<u64>::new(&[5, 7, 9]));
}

#[test]
fn add_i128() {
    let v = Vertex::<i128>::new(&[1, 2, 3]);
    let w = Vertex::<i128>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<i128>::new(&[5, 7, 9]));
}

#[test]
fn add_u128() {
    let v = Vertex::<u128>::new(&[1, 2, 3]);
    let w = Vertex::<u128>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<u128>::new(&[5, 7, 9]));
}

#[test]
fn add_isize() {
    let v = Vertex::<isize>::new(&[1, 2, 3]);
    let w = Vertex::<isize>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<isize>::new(&[5, 7, 9]));
}

#[test]
fn add_usize() {
    let v = Vertex::<usize>::new(&[1, 2, 3]);
    let w = Vertex::<usize>::new(&[4, 5, 6]);

    assert_eq!(v + w, Vertex::<usize>::new(&[5, 7, 9]));
}

#[test]
fn add_f32() {
    let v = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);
    let w = Vertex::<f32>::new(&[0.4, 0.5, 0.6]);

    assert_eq!(v + w, Vertex::<f32>::new(&[1.4, 2.5, 3.6]));
}

#[test]
fn add_f64() {
    let v = Vertex::<f64>::new(&[1.0, 2.0, 3.0]);
    let w = Vertex::<f64>::new(&[0.4, 0.5, 0.6]);

    assert_eq!(v + w, Vertex::<f64>::new(&[1.4, 2.5, 3.6]));
}

#[test]
fn sub_i8() {
    let v = Vertex::<i8>::new(&[1, 2, 3]);
    let w = Vertex::<i8>::new(&[4, 5, 6]);

    assert_eq!(v - w, Vertex::<i8>::new(&[-3, -3, -3]));
}

#[test]
fn sub_u8() {
    let v = Vertex::<u8>::new(&[1, 2, 3]);
    let w = Vertex::<u8>::new(&[4, 5, 6]);

    assert_eq!(w - v, Vertex::<u8>::new(&[3, 3, 3]));
}

#[test]
fn sub_i16() {
    let v = Vertex::<i16>::new(&[1, 2, 3]);
    let w = Vertex::<i16>::new(&[4, 5, 6]);

    assert_eq!(v - w, Vertex::<i16>::new(&[-3, -3, -3]));
}

#[test]
fn sub_u16() {
    let v = Vertex::<u16>::new(&[1, 2, 3]);
    let w = Vertex::<u16>::new(&[4, 5, 6]);

    assert_eq!(w - v, Vertex::<u16>::new(&[3, 3, 3]));
}

#[test]
fn sub_i32() {
    let v = Vertex::<i32>::new(&[1, 2, 3]);
    let w = Vertex::<i32>::new(&[4, 5, 6]);

    assert_eq!(v - w, Vertex::<i32>::new(&[-3, -3, -3]));
}

#[test]
fn sub_u32() {
    let v = Vertex::<u32>::new(&[1, 2, 3]);
    let w = Vertex::<u32>::new(&[4, 5, 6]);

    assert_eq!(w - v, Vertex::<u32>::new(&[3, 3, 3]));
}

#[test]
fn sub_i64() {
    let v = Vertex::<i64>::new(&[1, 2, 3]);
    let w = Vertex::<i64>::new(&[4, 5, 6]);

    assert_eq!(v - w, Vertex::<i64>::new(&[-3, -3, -3]));
}

#[test]
fn sub_u64() {
    let v = Vertex::<u64>::new(&[1, 2, 3]);
    let w = Vertex::<u64>::new(&[4, 5, 6]);

    assert_eq!(w - v, Vertex::<u64>::new(&[3, 3, 3]));
}

#[test]
fn sub_i128() {
    let v = Vertex::<i128>::new(&[1, 2, 3]);
    let w = Vertex::<i128>::new(&[4, 5, 6]);

    assert_eq!(v - w, Vertex::<i128>::new(&[-3, -3, -3]));
}

#[test]
fn sub_u128() {
    let v = Vertex::<u128>::new(&[1, 2, 3]);
    let w = Vertex::<u128>::new(&[4, 5, 6]);

    assert_eq!(w - v, Vertex::<u128>::new(&[3, 3, 3]));
}

#[test]
fn sub_isize() {
    let v = Vertex::<isize>::new(&[1, 2, 3]);
    let w = Vertex::<isize>::new(&[4, 5, 6]);

    assert_eq!(v - w, Vertex::<isize>::new(&[-3, -3, -3]));
}

#[test]
fn sub_usize() {
    let v = Vertex::<usize>::new(&[1, 2, 3]);
    let w = Vertex::<usize>::new(&[4, 5, 6]);

    assert_eq!(w - v, Vertex::<usize>::new(&[3, 3, 3]));
}

#[test]
fn sub_f32() {
    let v = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);
    let w = Vertex::<f32>::new(&[0.4, 0.5, 0.6]);

    assert_eq!(v - w, Vertex::<f32>::new(&[0.6, 1.5, 2.4]));
}

#[test]
fn sub_f64() {
    let v = Vertex::<f64>::new(&[1.0, 2.0, 3.0]);
    let w = Vertex::<f64>::new(&[0.4, 0.5, 0.6]);

    assert_eq!(w - v, Vertex::<f64>::new(&[-0.6, -1.5, -2.4]));
}
