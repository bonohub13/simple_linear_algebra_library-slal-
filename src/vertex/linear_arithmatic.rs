macro_rules! impl_mul_scala {
    ($t:ty) => {
        impl std::ops::Mul<$t> for super::Vertex<$t> {
            type Output = Self;

            fn mul(self, other: $t) -> super::Vertex<$t> {
                use rayon::prelude::*;

                let rv_vec: Vec<$t> = (0..self.len())
                    .into_par_iter()
                    .map(|i| self[i] * other)
                    .collect();

                Self::Output {
                    v: rv_vec,
                    vertical: self.vertical,
                }
            }
        }

        impl crate::linear::Dot<$t> for super::Vertex<$t> {
            type Output = super::Vertex<$t>;

            fn dot(&self, other: &$t) -> <Self as crate::linear::Dot<$t>>::Output {
                self.clone() * *other
            }
        }
    };
}

impl_mul_scala! { i8 }
impl_mul_scala! { u8 }
impl_mul_scala! { i16 }
impl_mul_scala! { u16 }
impl_mul_scala! { i32 }
impl_mul_scala! { u32 }
impl_mul_scala! { i64 }
impl_mul_scala! { u64 }
impl_mul_scala! { i128 }
impl_mul_scala! { u128 }
impl_mul_scala! { isize }
impl_mul_scala! { usize }
impl_mul_scala! { f32 }
impl_mul_scala! { f64 }

macro_rules! impl_mul_with_scala {
    ($t:ty) => {
        impl std::ops::Mul<super::Vertex<$t>> for $t {
            type Output = super::Vertex<$t>;

            fn mul(self, other: super::Vertex<$t>) -> super::Vertex<$t> {
                other * self
            }
        }

        impl crate::linear::Dot<super::Vertex<$t>> for $t {
            type Output = super::Vertex<$t>;

            fn dot(
                &self,
                other: &super::Vertex<$t>,
            ) -> <Self as crate::linear::Dot<super::Vertex<$t>>>::Output {
                other.dot(self)
            }
        }
    };
}

impl_mul_with_scala! { i8 }
impl_mul_with_scala! { u8 }
impl_mul_with_scala! { i16 }
impl_mul_with_scala! { u16 }
impl_mul_with_scala! { i32 }
impl_mul_with_scala! { u32 }
impl_mul_with_scala! { i64 }
impl_mul_with_scala! { u64 }
impl_mul_with_scala! { i128 }
impl_mul_with_scala! { u128 }
impl_mul_with_scala! { isize }
impl_mul_with_scala! { usize }
impl_mul_with_scala! { f32 }
impl_mul_with_scala! { f64 }
