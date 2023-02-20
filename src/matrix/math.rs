macro_rules! impl_add {
    ($($t:ty)*) => ($(
        impl std::ops::Add for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn add(self, other: super::Matrix<$t>) -> super::Matrix<$t> {
                use super::Matrix;
                use std::iter::zip;

                if self.size() == other.size() {
                    let rv: Vec<Vec<$t>> = zip(self.to_vec(), other.to_vec())
                        .map(|(v, w)| {
                            let rv_i: Vec<$t> = zip(v.as_slice(), w.as_slice())
                                .map(|(v_i, w_i)| *v_i + *w_i)
                                .collect();

                            rv_i
                        })
                        .collect();
                    let rv_slices: Vec<&[$t]> = rv
                        .iter()
                        .map(|rv_i| rv_i.as_slice())
                        .collect();

                    Matrix::<$t>::new(rv_slices.as_slice()).unwrap()
                } else {
                    panic!("Failed to add two matrices with differing size");
                }
            }
        }
    )*)
}

impl_add! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64}

macro_rules! impl_sub {
    ($($t:ty)*) => ($(
        impl std::ops::Sub for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn sub(self, other: super::Matrix<$t>) -> super::Matrix<$t> {
                use super::Matrix;
                use std::iter::zip;

                if self.size() == other.size() {
                    let rv: Vec<Vec<$t>> = zip(self.to_vec(), other.to_vec())
                        .map(|(v, w)| {
                            let rv_i: Vec<$t> = zip(v.as_slice(), w.as_slice())
                                .map(|(v_i, w_i)| *v_i - *w_i)
                                .collect();

                            rv_i
                        })
                        .collect();
                    let rv_slices: Vec<&[$t]> = rv
                        .iter()
                        .map(|rv_i| rv_i.as_slice())
                        .collect();

                    Matrix::<$t>::new(rv_slices.as_slice()).unwrap()
                } else {
                    panic!("Failed to substract two matrices with differing size");
                }
            }
        }
    )*)
}

impl_sub! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64}

macro_rules! impl_mul_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<$t> for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn mul(self, other: $t) -> super::Matrix<$t> {
                use super::Matrix;

                let rv: Vec<Vec<$t>> = self
                    .to_vec()
                    .iter()
                    .map(|v| {
                        let rv_i: Vec<$t> = v
                            .iter()
                            .map(|v_i| *v_i * other)
                            .collect();

                        rv_i
                    })
                    .collect();
                let rv_slices: Vec<&[$t]> = rv
                    .iter()
                    .map(|rv_i| rv_i.as_slice())
                    .collect();

                Matrix::<$t>::new(rv_slices.as_slice()).unwrap()
            }
        }
    )*)
}

impl_mul_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64}
