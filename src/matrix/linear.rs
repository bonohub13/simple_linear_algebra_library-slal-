macro_rules! impl_dot_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Matrix<$t>> for $t {
            type Output = super::Matrix<$t>;

            fn mul(self, other: Self::Output) -> Self::Output {
                use super::Matrix;
                use crate::vertex::Vertex;
                use crate::linear::Dot;

                let rv_vec: Vec<Vec<$t>> = other
                    .to_vec()
                    .iter()
                    .map(|m_i| {
                        Vertex::<$t>::new(m_i.as_slice()).dot(&self).to_vec()
                    })
                    .collect();
                let rv: Vec<&[$t]> = rv_vec
                    .iter()
                    .map(|rv_i| rv_i.as_slice())
                    .collect();

                Matrix::<$t>::new(rv.as_slice()).unwrap()
            }
        }

        impl crate::linear::Dot<super::Matrix<$t>> for $t {
            type Output = super::Matrix<$t>;

            fn dot(&self, other: &Self::Output) -> Self::Output {
                *self * other.clone()
            }
        }
    )*)
}

impl_dot_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_dot_with_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<$t> for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn mul(self, other: $t) -> Self::Output {
                other * self
            }
        }

        impl crate::linear::Dot<$t> for super::Matrix<$t> {
            type Output = super::Matrix<$t>;

            fn dot(&self, other: &$t) -> Self::Output {
                *other * self.clone()
            }
        }
    )*)
}

impl_dot_with_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
