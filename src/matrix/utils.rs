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
    ($($t:ty)*) => ($(
        impl crate::utils::Round for super::Matrix<$t> {
            fn round(&mut self) {
                use rayon::prelude::*;

                self.m.par_iter_mut().for_each(|val| {
                    *val = val.round();
                });
            }
        }
    )*)
}

impl_round! {f32 f64}

macro_rules! impl_rpy {
    ($t:ty) => {
        impl crate::utils::RPY for super::Matrix<$t> {
            type Output = crate::error::SlalErr<super::Matrix<f64>, f64>;

            fn rpy(&self, rpy: [f64; 3]) -> Self::Output {
                use crate::linear::Dot;
                use rayon::prelude::*;

                let rotation_matrix = super::Matrix::<f64> {
                    m: vec![
                        // 1st row
                        rpy[2].cos() * rpy[1].cos(),
                        (0..3)
                            .into_par_iter()
                            .map(|i| if i == 2 { rpy[i].cos() } else { rpy[i].sin() })
                            .product::<f64>()
                            - rpy[2].sin() * rpy[0].cos(),
                        (0..3)
                            .into_par_iter()
                            .map(|i| if i == 1 { rpy[i].sin() } else { rpy[i].cos() })
                            .product::<f64>()
                            + rpy[2].sin() * rpy[0].sin(),
                        // 2nd row
                        rpy[2].sin() * rpy[1].cos(),
                        rpy.par_iter().map(|rpy_i| (*rpy_i).sin()).product::<f64>()
                            * rpy[2].cos()
                            * rpy[0].cos(),
                        (0..3)
                            .into_par_iter()
                            .map(|i| if i != 0 { rpy[i].cos() } else { rpy[i].sin() })
                            .product::<f64>()
                            - rpy[2].cos() * rpy[0].sin(),
                        // 3rd row
                        -rpy[1].sin(),
                        (0..2)
                            .into_par_iter()
                            .map(|i| if i == 0 { rpy[i].sin() } else { rpy[i].cos() })
                            .product(),
                        (0..2).into_par_iter().map(|i| rpy[i].cos()).product(),
                    ],
                    size: [3, 3],
                };

                super::Matrix::<f64>::from(self).dot(&rotation_matrix)
            }
        }
    };
}

impl crate::utils::RPY for super::Matrix<f64> {
    type Output = crate::error::SlalErr<Self, f64>;

    fn rpy(&self, rpy: [f64; 3]) -> Self::Output {
        use crate::linear::Dot;
        use rayon::prelude::*;

        let rotation_matrix = super::Matrix::<f64> {
            m: vec![
                // 1st row
                rpy[2].cos() * rpy[1].cos(),
                (0..3)
                    .into_par_iter()
                    .map(|i| if i == 2 { rpy[i].cos() } else { rpy[i].sin() })
                    .product::<f64>()
                    - rpy[2].sin() * rpy[0].cos(),
                (0..3)
                    .into_par_iter()
                    .map(|i| if i == 1 { rpy[i].sin() } else { rpy[i].cos() })
                    .product::<f64>()
                    + rpy[2].sin() * rpy[0].sin(),
                // 2nd row
                rpy[2].sin() * rpy[1].cos(),
                rpy.par_iter().map(|rpy_i| (*rpy_i).sin()).product::<f64>()
                    * rpy[2].cos()
                    * rpy[0].cos(),
                (0..3)
                    .into_par_iter()
                    .map(|i| if i != 0 { rpy[i].cos() } else { rpy[i].sin() })
                    .product::<f64>()
                    - rpy[2].cos() * rpy[0].sin(),
                // 3rd row
                -rpy[1].sin(),
                (0..2)
                    .into_par_iter()
                    .map(|i| if i == 0 { rpy[i].sin() } else { rpy[i].cos() })
                    .product(),
                (0..2).into_par_iter().map(|i| rpy[i].cos()).product(),
            ],
            size: [3, 3],
        };

        self.dot(&rotation_matrix)
    }
}

impl_rpy! {i8}
impl_rpy! {i16}
impl_rpy! {i32}
impl_rpy! {u8}
impl_rpy! {u16}
impl_rpy! {u32}
impl_rpy! {f32}
