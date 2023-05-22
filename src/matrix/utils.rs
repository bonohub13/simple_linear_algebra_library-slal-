macro_rules! matrix_indexing {
    ($($t:ty)*) => ($(
        impl std::ops::Index<usize> for super::Matrix<$t> {
            type Output = [$t];

            fn index(&self, index: usize) -> &Self::Output {
                &self.m[index * self.size[0]..(index + 1) * self.size[0]]
            }
        }

        impl std::ops::IndexMut<usize> for super::Matrix<$t> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.m[index * self.size[0]..(index + 1) * self.size[0]]
            }
        }

        impl std::ops::Index<usize> for &super::Matrix<$t> {
            type Output = [$t];

            fn index(&self, index: usize) -> &Self::Output {
                &self.m[index * self.size[0]..(index + 1) * self.size[0]]
            }
        }

        impl std::ops::Index<usize> for &mut super::Matrix<$t> {
            type Output = [$t];

            fn index(&self, index: usize) -> &Self::Output {
                &self.m[index * self.size[0]..(index + 1) * self.size[0]]
            }
        }

        impl std::ops::IndexMut<usize> for &mut super::Matrix<$t> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.m[index * self.size[0]..(index + 1) * self.size[0]]
            }
        }
    )*)
}

matrix_indexing! {i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64}

macro_rules! impl_round {
    ($t:ty) => {
        impl crate::utils::Round for super::Matrix<$t> {
            #[doc=concat!(
                                                        r"Outputs matrix (`slal::matrix::Matrix<`",
                                                        stringify!($t),
                                                        r"`) with rounded values.\n",
                                                        r"\n# Examples\n",
                                                        r"```\n",
                                                        r"use slal::matrix::*;\n\n",
                                                        r"let m = Matrix::<",
                                                        stringify!($t),
                                                        r">::new(&[10.0, 1., 0.1]).unwrap();\n",
                                                        r"\nassert!(m.round() == Matrix::<",
                                                        stringify!($t),
                                                        r">::new(&[10., 1., 0.]).unwrap());\n",
                                                        r"```\n",
                                                    )]
            fn round(&mut self) {
                use rayon::prelude::*;

                self.m.par_iter_mut().for_each(|val| {
                    *val = val.round();
                });
            }
        }
    };
}

impl_round! {f32}
impl_round! {f64}
