macro_rules! impl_mul_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<$t> for super::Vertex<$t> {
            type Output = Self;

            fn mul(self, other: $t) -> super::Vertex<$t> {
                let rv_vec: Vec<$t> = (0..self.len())
                    .into_iter()
                    .map(|i| self[i] * other)
                    .collect();

                if self.is_transposed() {
                    Self::Output {
                        v: rv_vec,
                        vertical: true,
                    }
                } else {
                    Self::Output {
                        v: rv_vec,
                        vertical: false,
                    }
                }
            }
        }

        impl crate::linear::Dot<$t> for super::Vertex<$t> {
            type Output = super::Vertex<$t>;

            fn dot(&self, other: &$t) -> <Self as crate::linear::Dot<$t>>::Output {
                self.clone() * *other
            }
        }
    )*)
}

impl_mul_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_mul_with_scala {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Vertex<$t>> for $t {
            type Output = super::Vertex<$t>;

            fn mul(self, other: super::Vertex<$t>) -> super::Vertex<$t> {
                other * self
            }
        }

        impl crate::linear::Dot<super::Vertex<$t>> for $t {
            type Output = super::Vertex<$t>;

            fn dot(&self, other: &super::Vertex<$t>) -> <Self as crate::linear::Dot<super::Vertex<$t>>>::Output {
                other.dot(self)
            }
        }
    )*)
}

impl_mul_with_scala! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
