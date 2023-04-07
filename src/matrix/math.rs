macro_rules! impl_add {
    ($($t:ty)*) => ($(
        impl std::ops::Add for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn add(self, other: super::Matrix<$t>) -> Self::Output {
                use rayon::prelude::*;

                if self.size() != other.size() {
                    panic!("Failed to add two matrices with differing size");
                }

                let mut rv: Vec<$t> = vec![0 as $t; self.size[0] * self.size[1]];
                rv.par_iter_mut().enumerate().for_each(|(idx, val)| {
                    *val = self.m[idx] + other.m[idx];
                });

                Self::Output {
                    m: rv,
                    size: self.size
                }
            }
        }
    )*)
}

impl_add! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_sub {
    ($($t:ty)*) => ($(
        impl std::ops::Sub for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn sub(self, other: super::Matrix<$t>) -> Self::Output {
                use rayon::prelude::*;

                if self.size() != other.size() {
                    panic!("Failed to substract two matrices with differing size");
                }

                let mut rv: Vec<$t> = vec![0 as $t; self.size[0] * self.size[1]];
                rv.par_iter_mut().enumerate().for_each(|(idx, val)| {
                    *val = self.m[idx] - other.m[idx];
                });

                Self::Output {
                    m: rv,
                    size: self.size,
                }
            }
        }
    )*)
}

impl_sub! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
