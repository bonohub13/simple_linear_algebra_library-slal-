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

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Vertex<$t>> for super::Vertex<$t>
        {
            type Output = crate::matrix::Matrix<$t>;

            fn mul(self, other: Self) -> crate::matrix::Matrix<$t> {
                use crate::matrix::Matrix;

                if self.is_transposed() && !other.is_transposed() {
                    let rv: Vec<Vec<$t>> = self
                        .to_vec()
                        .iter()
                        .map(|&v_i| (v_i * other.clone()).to_vec())
                        .collect();
                    let rv_slice: Vec<&[$t]> = rv
                        .iter()
                        .map(|rv_i| rv_i.as_slice())
                        .collect();

                    Matrix::<$t>::new(rv_slice.as_slice()).unwrap()
                } else if !self.is_transposed() && other.is_transposed() {
                    if self.len() == other.len() {
                        let v1 = self.to_vec();
                        let v2 = other.to_vec();

                        let rv: Vec<$t> = v1
                            .iter()
                            .enumerate()
                            .map(|(idx, &v1_i)| v1_i * v2[idx])
                            .collect();

                        Matrix::<$t>::new(&[&[rv.iter().sum()]]).unwrap()
                    } else {
                        panic!("Size of both vectors must match when in multiplication of V.t x W");
                    }
                } else {
                    panic!("Cannot add two vectors when either is not transposed");
                }
            }
        }
    )*)
}

mul_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;
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

    #[test]
    fn mul_i8() {
        let v1 = Vertex::<i8>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i8>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<i8>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_i8_trasposed() {
        let v1 = Vertex::<i8>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i8>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<i8>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_i8_none_transposed() {
        let v1 = Vertex::<i8>::new(&[1, 2, 3]);
        let v2 = Vertex::<i8>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i8_both_transposed() {
        let mut v1 = Vertex::<i8>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i8>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i8_non_matching_size() {
        let mut v1 = Vertex::<i8>::new(&[1, 2, 3]);
        let v2 = Vertex::<i8>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_u8() {
        let v1 = Vertex::<u8>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u8>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<u8>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_u8_trasposed() {
        let v1 = Vertex::<u8>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u8>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<u8>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_u8_none_transposed() {
        let v1 = Vertex::<u8>::new(&[1, 2, 3]);
        let v2 = Vertex::<u8>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u8_both_transposed() {
        let mut v1 = Vertex::<u8>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u8>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u8_non_matching_size() {
        let mut v1 = Vertex::<u8>::new(&[1, 2, 3]);
        let v2 = Vertex::<u8>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    #[should_panic]
    fn mul_i16_none_transposed() {
        let v1 = Vertex::<i16>::new(&[1, 2, 3]);
        let v2 = Vertex::<i16>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i16_both_transposed() {
        let mut v1 = Vertex::<i16>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i16>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i16_non_matching_size() {
        let mut v1 = Vertex::<i16>::new(&[1, 2, 3]);
        let v2 = Vertex::<i16>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_u16() {
        let v1 = Vertex::<u16>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u16>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<u16>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_u16_trasposed() {
        let v1 = Vertex::<u16>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u16>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<u16>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_u16_none_transposed() {
        let v1 = Vertex::<u16>::new(&[1, 2, 3]);
        let v2 = Vertex::<u16>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u16_both_transposed() {
        let mut v1 = Vertex::<u16>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u16>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u16_non_matching_size() {
        let mut v1 = Vertex::<u16>::new(&[1, 2, 3]);
        let v2 = Vertex::<u16>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_i32() {
        let v1 = Vertex::<i32>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i32>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<i32>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_i32_trasposed() {
        let v1 = Vertex::<i32>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i32>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<i32>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_i32_none_transposed() {
        let v1 = Vertex::<i32>::new(&[1, 2, 3]);
        let v2 = Vertex::<i32>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i32_both_transposed() {
        let mut v1 = Vertex::<i32>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i32>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i32_non_matching_size() {
        let mut v1 = Vertex::<i32>::new(&[1, 2, 3]);
        let v2 = Vertex::<i32>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_u32() {
        let v1 = Vertex::<u32>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u32>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<u32>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_u32_trasposed() {
        let v1 = Vertex::<u32>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u32>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<u32>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_u32_none_transposed() {
        let v1 = Vertex::<u32>::new(&[1, 2, 3]);
        let v2 = Vertex::<u32>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u32_both_transposed() {
        let mut v1 = Vertex::<u32>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u32>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u32_non_matching_size() {
        let mut v1 = Vertex::<u32>::new(&[1, 2, 3]);
        let v2 = Vertex::<u32>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_i64() {
        let v1 = Vertex::<i64>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i64>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<i64>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_i64_trasposed() {
        let v1 = Vertex::<i64>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i64>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<i64>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_i64_none_transposed() {
        let v1 = Vertex::<i64>::new(&[1, 2, 3]);
        let v2 = Vertex::<i64>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i64_both_transposed() {
        let mut v1 = Vertex::<i64>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i64>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i64_non_matching_size() {
        let mut v1 = Vertex::<i64>::new(&[1, 2, 3]);
        let v2 = Vertex::<i64>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_u64() {
        let v1 = Vertex::<u64>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u64>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<u64>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_u64_trasposed() {
        let v1 = Vertex::<u64>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u64>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<u64>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_u64_none_transposed() {
        let v1 = Vertex::<u64>::new(&[1, 2, 3]);
        let v2 = Vertex::<u64>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u64_both_transposed() {
        let mut v1 = Vertex::<u64>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u64>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u64_non_matching_size() {
        let mut v1 = Vertex::<u64>::new(&[1, 2, 3]);
        let v2 = Vertex::<u64>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_i128() {
        let v1 = Vertex::<i128>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i128>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<i128>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_i128_trasposed() {
        let v1 = Vertex::<i128>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i128>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<i128>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_i128_none_transposed() {
        let v1 = Vertex::<i128>::new(&[1, 2, 3]);
        let v2 = Vertex::<i128>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i128_both_transposed() {
        let mut v1 = Vertex::<i128>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<i128>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_i128_non_matching_size() {
        let mut v1 = Vertex::<i128>::new(&[1, 2, 3]);
        let v2 = Vertex::<i128>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_u128() {
        let v1 = Vertex::<u128>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u128>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<u128>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_u128_trasposed() {
        let v1 = Vertex::<u128>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u128>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<u128>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_u128_none_transposed() {
        let v1 = Vertex::<u128>::new(&[1, 2, 3]);
        let v2 = Vertex::<u128>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u128_both_transposed() {
        let mut v1 = Vertex::<u128>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<u128>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_u128_non_matching_size() {
        let mut v1 = Vertex::<u128>::new(&[1, 2, 3]);
        let v2 = Vertex::<u128>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_isize() {
        let v1 = Vertex::<isize>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<isize>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<isize>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_isize_trasposed() {
        let v1 = Vertex::<isize>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<isize>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<isize>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_isize_none_transposed() {
        let v1 = Vertex::<isize>::new(&[1, 2, 3]);
        let v2 = Vertex::<isize>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_isize_both_transposed() {
        let mut v1 = Vertex::<isize>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<isize>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_isize_non_matching_size() {
        let mut v1 = Vertex::<isize>::new(&[1, 2, 3]);
        let v2 = Vertex::<isize>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_usize() {
        let v1 = Vertex::<usize>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<usize>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<usize>::new(&[&[32]]).unwrap());
    }

    #[test]
    fn mul_usize_trasposed() {
        let v1 = Vertex::<usize>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<usize>::new(&[4, 5, 6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<usize>::new(&[&[4, 8, 12], &[5, 10, 15], &[6, 12, 18]]).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_usize_none_transposed() {
        let v1 = Vertex::<usize>::new(&[1, 2, 3]);
        let v2 = Vertex::<usize>::new(&[4, 5, 6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_usize_both_transposed() {
        let mut v1 = Vertex::<usize>::new(&[1, 2, 3]);
        let mut v2 = Vertex::<usize>::new(&[4, 5, 6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_usize_non_matching_size() {
        let mut v1 = Vertex::<usize>::new(&[1, 2, 3]);
        let v2 = Vertex::<usize>::new(&[4, 5, 6, 7]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_f32() {
        let v1 = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);
        let mut v2 = Vertex::<f32>::new(&[4.0, 5.0, 6.0]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<f32>::new(&[&[32.0]]).unwrap());
    }

    #[test]
    fn mul_f32_trasposed() {
        let v1 = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);
        let mut v2 = Vertex::<f32>::new(&[4.0, 5.0, 6.0]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<f32>::new(&[&[4.0, 8.0, 12.0], &[5.0, 10.0, 15.0], &[6.0, 12.0, 18.0]])
                .unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_f32_none_transposed() {
        let v1 = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);
        let v2 = Vertex::<f32>::new(&[4.0, 5.0, 6.0]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_f32_both_transposed() {
        let mut v1 = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);
        let mut v2 = Vertex::<f32>::new(&[4.0, 5.0, 6.0]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_f32_non_matching_size() {
        let mut v1 = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);
        let v2 = Vertex::<f32>::new(&[4.0, 5.0, 6.0, 7.0]);

        v1.t();

        let _ = v2 * v1;
    }

    #[test]
    fn mul_f64() {
        let v1 = Vertex::<f64>::new(&[0.1, 0.2, 0.3]);
        let mut v2 = Vertex::<f64>::new(&[0.4, 0.5, 0.6]);

        v2.t();

        assert_eq!(v1 * v2, Matrix::<f64>::new(&[&[0.32]]).unwrap());
    }

    #[test]
    fn mul_f64_trasposed() {
        let v1 = Vertex::<f64>::new(&[0.1, 0.2, 0.3]);
        let mut v2 = Vertex::<f64>::new(&[0.4, 0.5, 0.6]);

        v2.t();

        assert_eq!(
            v2 * v1,
            Matrix::<f64>::new(&[
                &[0.1 * 0.4, 0.2 * 0.4, 0.3 * 0.4],
                &[0.1 * 0.5, 0.2 * 0.5, 0.3 * 0.5],
                &[0.1 * 0.6, 0.2 * 0.6, 0.3 * 0.6]
            ])
            .unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn mul_f64_none_transposed() {
        let v1 = Vertex::<f64>::new(&[0.1, 0.2, 0.3]);
        let v2 = Vertex::<f64>::new(&[0.4, 0.5, 0.6]);

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_f64_both_transposed() {
        let mut v1 = Vertex::<f64>::new(&[0.1, 0.2, 0.3]);
        let mut v2 = Vertex::<f64>::new(&[0.4, 0.5, 0.6]);

        v1.t();
        v2.t();

        let _ = v1 * v2;
    }

    #[test]
    #[should_panic]
    fn mul_f64_non_matching_size() {
        let mut v1 = Vertex::<f64>::new(&[0.1, 0.2, 0.3]);
        let v2 = Vertex::<f64>::new(&[0.4, 0.5, 0.6, 0.7]);

        v1.t();

        let _ = v2 * v1;
    }
}
