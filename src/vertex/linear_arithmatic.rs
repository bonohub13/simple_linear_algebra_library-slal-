macro_rules! impl_mul_scala {
    ($t:ty, $doc:expr) => {
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

            [doc=$doc]
            fn dot(&self, other: &$t) -> <Self as crate::linear::Dot<$t>>::Output {
                self.clone() * *other
            }
        }
    };

    ($t:ty) => {
        impl_mul_scala!(
            $t,
            concat!(
                r"Computes dot product of `slal::vec::Vertex<",
                stringify!($t),
                r">` and scala of `",
                stringify!($t),
                r"`.\n",
                r"\n# Examples\n",
                r"```\n",
                r"use slal::vec::*;\n",
                r"\nlet v = Vertex::<",
                stringify!($t),,
                r">::new(&[1, 2, 3]);\n",
                r"let scala = 2;\n",
                r"\nassert!(v.dot(&scala) == Vertex::<",
                stringify!($t),,
                r">::new(&[2, 4, 6]));\n",
                r"```",
            ),
        );
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
impl_mul_scala! { f32, concat!(
    "Computes dot product of `slal::vec::Vector<f32>` and scala of `f32`\n"
    "\n# Examples\n",
    "```\n",
    "use slal::vec::*;\n",
    "\nlet v = Vertex::<f32>::new(&[1., 0.2, 0.03]);\n",
    "let scala = 4.;\n",
    "\nassert!(v.dot(&scala) == Vertex::<f32>::new(&[4., 0.8, 0.12]));\n",
    "```",
) }
impl_mul_scala! { f64, concat!(
    "Computes dot product of `slal::vec::Vector<f64>` and scala of `f64`\n"
    "\n# Examples\n",
    "```\n",
    "use slal::vec::*;\n",
    "\nlet v = Vertex::<f64>::new(&[1., 0.2, 0.03]);\n",
    "let scala = 4.;\n",
    "\nassert!(v.dot(&scala) == Vertex::<f64>::new(&[4., 0.8, 0.12]));\n",
    "```",
) }

macro_rules! impl_mul_with_scala {
    ($t:ty, $doc:expr) => {
        impl std::ops::Mul<super::Vertex<$t>> for $t {
            type Output = super::Vertex<$t>;

            fn mul(self, other: super::Vertex<$t>) -> super::Vertex<$t> {
                other * self
            }
        }

        impl crate::linear::Dot<super::Vertex<$t>> for $t {
            type Output = super::Vertex<$t>;

            [doc=$doc]
            fn dot(
                &self,
                other: &super::Vertex<$t>,
            ) -> <Self as crate::linear::Dot<super::Vertex<$t>>>::Output {
                other.dot(self)
            }
        }
    };

    ($t:ty) => {
        impl_mul_with_scala!(
            $t,
            concat!(
                r"Computes dot product of `slal::vec::Vertex<",
                stringify!($t),
                r">` and scala of `",
                stringify!($t),
                r"`.\n",
                r"\n# Examples\n",
                r"```\n",
                r"use slal::vec::*;\n",
                r"\nlet v = Vertex::<",
                stringify!($t),,
                r">::new(&[1, 2, 3]);\n",
                r"let scala = 2;\n",
                r"\nassert!(scala.dot(&v) == Vertex::<",
                stringify!($t),,
                r">::new(&[2, 4, 6]));\n",
                r"```",
            ),
        );
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
impl_mul_with_scala! { f32, concat!(
    "Computes dot product of `slal::vec::Vector<f32>` and scala of `f32`\n"
    "\n# Examples\n",
    "```\n",
    "use slal::vec::*;\n",
    "\nlet v = Vertex::<f32>::new(&[1., 0.2, 0.03]);\n",
    "let scala = 4.;\n",
    "\nassert!(scala.dot(&v) == Vertex::<f32>::new(&[4., 0.8, 0.12]));\n",
    "```",
) }
impl_mul_with_scala! { f64, concat!(
    "Computes dot product of `slal::vec::Vector<f64>` and scala of `f64`\n"
    "\n# Examples\n",
    "```\n",
    "use slal::vec::*;\n",
    "\nlet v = Vertex::<f64>::new(&[1., 0.2, 0.03]);\n",
    "let scala = 4.;\n",
    "\nassert!(scala.dot(&v) == Vertex::<f64>::new(&[4., 0.8, 0.12]));\n",
    "```",
) }
