macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl std::ops::Add for super::Vertex<$t> {
            type Output = super::Vertex<$t>;

            fn add(self, other: super::Vertex<$t>) -> super::Vertex<$t> {
                use super::Vertex;

                if self.len() == other.len() && self.is_transposed() == other.is_transposed() {
                    let retval: Vec<$t> = self
                        .to_vec()
                        .iter()
                        .enumerate()
                        .map(|(idx, &v_i)| v_i + other.to_vec()[idx])
                        .collect();
                    let mut retval_vertex = Vertex::<$t>::new(retval.as_slice());

                    if self.is_transposed() {
                        retval_vertex.t();
                    }

                    retval_vertex
                } else {
                    panic!(
                        "Cannot add {:?} and {:?}.",
                        self.type_name(),
                        other.type_name()
                    );
                }
            }
        }
    )*)
}

add_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl std::ops::Sub for super::Vertex<$t>
        {
            type Output = super::Vertex<$t>;

            fn sub(self, other: super::Vertex<$t>) -> Self {
                use super::Vertex;

                if self.len() == other.len() && self.is_transposed() == other.is_transposed() {
                    let retval: Vec<$t> = self
                        .to_vec()
                        .iter()
                        .enumerate()
                        .map(|(idx, &v_i)| v_i - other.to_vec()[idx])
                        .collect();
                    let mut retval_vertex = Vertex::<$t>::new(retval.as_slice());

                    if self.is_transposed() {
                        retval_vertex.t();
                    }

                    retval_vertex
                } else {
                    panic!(
                        "Cannot substract {:?} from, {:?}.",
                        other.type_name(),
                        self.type_name()
                    );
                }
            }
        }
    )*)
}

sub_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! mul_scala_impl {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Vertex<$t>> for $t
        {
            type Output = super::Vertex<$t>;

            fn mul(self, other: super::Vertex<$t>) -> super::Vertex<$t> {
                use super::Vertex;

                let retval: Vec<$t> = other
                    .to_vec()
                    .iter()
                    .map(|&v_i| v_i * self)
                    .collect();
                let mut retval_vertex = Vertex::<$t>::new(retval.as_slice());

                if other.is_transposed() {
                    retval_vertex.t();
                }

                retval_vertex
            }
        }
    )*)
}

mul_scala_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

#[cfg(test)]
mod test {
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

    #[test]
    fn mul_scala_i8() {
        let v = Vertex::<i8>::new(&[1, 2, 3]);

        assert_eq!(-3 * v, Vertex::<i8>::new(&[-3, -6, -9]));
    }

    #[test]
    fn mul_scala_u8() {
        let v = Vertex::<u8>::new(&[1, 2, 3]);

        assert_eq!(3 * v, Vertex::<u8>::new(&[3, 6, 9]));
    }

    #[test]
    fn mul_scala_i16() {
        let v = Vertex::<i16>::new(&[1, 2, 3]);

        assert_eq!(-3 * v, Vertex::<i16>::new(&[-3, -6, -9]));
    }

    #[test]
    fn mul_scala_u16() {
        let v = Vertex::<u16>::new(&[1, 2, 3]);

        assert_eq!(3 * v, Vertex::<u16>::new(&[3, 6, 9]));
    }

    #[test]
    fn mul_scala_i32() {
        let v = Vertex::<i32>::new(&[1, 2, 3]);

        assert_eq!(-3 * v, Vertex::<i32>::new(&[-3, -6, -9]));
    }

    #[test]
    fn mul_scala_u32() {
        let v = Vertex::<u32>::new(&[1, 2, 3]);

        assert_eq!(3 * v, Vertex::<u32>::new(&[3, 6, 9]));
    }

    #[test]
    fn mul_scala_i64() {
        let v = Vertex::<i64>::new(&[1, 2, 3]);

        assert_eq!(-3 * v, Vertex::<i64>::new(&[-3, -6, -9]));
    }

    #[test]
    fn mul_scala_u64() {
        let v = Vertex::<u64>::new(&[1, 2, 3]);

        assert_eq!(3 * v, Vertex::<u64>::new(&[3, 6, 9]));
    }

    #[test]
    fn mul_scala_i128() {
        let v = Vertex::<i128>::new(&[1, 2, 3]);

        assert_eq!(-3 * v, Vertex::<i128>::new(&[-3, -6, -9]));
    }

    #[test]
    fn mul_scala_u128() {
        let v = Vertex::<u128>::new(&[1, 2, 3]);

        assert_eq!(3 * v, Vertex::<u128>::new(&[3, 6, 9]));
    }

    #[test]
    fn mul_scala_isize() {
        let v = Vertex::<isize>::new(&[1, 2, 3]);

        assert_eq!(-3 * v, Vertex::<isize>::new(&[-3, -6, -9]));
    }

    #[test]
    fn mul_scala_usize() {
        let v = Vertex::<usize>::new(&[1, 2, 3]);

        assert_eq!(3 * v, Vertex::<usize>::new(&[3, 6, 9]));
    }

    #[test]
    fn mul_scala_f32() {
        let v = Vertex::<f32>::new(&[0.1, 0.2, 0.3]);

        assert_eq!(
            -3.0 * v,
            Vertex::<f32>::new(&[-3.0 * 0.1, -3.0 * 0.2, -3.0 * 0.3])
        );
    }

    #[test]
    fn mul_scala_f64() {
        let v = Vertex::<f64>::new(&[1.1, 2.1, 3.1]);

        assert_eq!(
            3.0 * v,
            Vertex::<f64>::new(&[3.0 * 1.1, 3.0 * 2.1, 3.0 * 3.1])
        );
    }
}
