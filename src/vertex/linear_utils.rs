macro_rules! impl_magnitude_vertex {
    ($($t:ty)*) => ($(
        impl crate::linear::Magnitude for super::Vertex<$t> {
            type Output = f64;

            fn magnitude(&self) -> Self::Output {
                use rayon::prelude::*;

                (0..self.len())
                    .into_par_iter()
                    .map(|idx| (self[idx] * self[idx]) as f64)
                    .sum::<f64>()
                    .sqrt()
            }
        }
    )*)
}

impl_magnitude_vertex! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_random_signed {
    ($($t:ty)*) => ($(
        impl crate::linear::Random for super::Vertex<$t> {
            type Output = super::Vertex<$t>;
            type Size = usize;

            fn rand(size: Self::Size) -> Self::Output {
                use rand::{Rng, self};
                use rayon::prelude::*;

                const DELTA: f64 = 1e-6;

                let mut v = vec![0 as $t; size];
                v.par_iter_mut().for_each(|v_i| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *v_i = thread_rng.gen::<$t>() * if thread_rng.gen::<i8>() % 2 == 1 {
                            -1 as $t
                        } else {
                            1 as $t
                        };

                        if (*v_i as f64).abs() > DELTA {
                            return;
                        }
                    }
                });

                Self::Output {
                    v,
                    vertical: false,
                }
            }

            fn rand_transposed(size: Self::Size) -> Self::Output {
                use rand::{Rng, self};
                use rayon::prelude::*;

                const DELTA: f64 = 1e-6;

                let mut v = vec![0 as $t; size];
                v.par_iter_mut().for_each(|v_i| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *v_i = thread_rng.gen::<$t>() * if thread_rng.gen::<i8>() % 2 == 1 {
                            -1 as $t
                        } else {
                            1 as $t
                        };

                        if (*v_i as f64).abs() > DELTA {
                            return;
                        }
                    }
                });

                Self::Output {
                    v,
                    vertical: true,
                }
            }
        }
    )*)
}

impl_random_signed! { i8 i16 i32 i64 i128 isize f32 f64 }

macro_rules! impl_random_unsigned {
    ($($t:ty)*) => ($(
        impl crate::linear::Random for super::Vertex<$t> {
            type Output = super::Vertex<$t>;
            type Size = usize;

            fn rand(size: Self::Size) -> Self::Output {
                use rand::{Rng, self};
                use rayon::prelude::*;

                let mut v = vec![0 as $t; size];
                v.par_iter_mut().for_each(|v_i| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *v_i = thread_rng.gen::<$t>();

                        if *v_i > 0 {
                            return;
                        }
                    }
                });

                Self::Output {
                    v,
                    vertical: false,
                }
            }

            fn rand_transposed(size: Self::Size) -> Self::Output {
                use rand::{Rng, self};
                use rayon::prelude::*;

                let mut v = vec![0 as $t; size];
                v.par_iter_mut().for_each(|v_i| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *v_i = thread_rng.gen::<$t>();

                        if *v_i > 0 {
                            return;
                        }
                    }
                });

                Self::Output {
                    v,
                    vertical: true,
                }
            }
        }
    )*)
}

impl_random_unsigned! { u8 u16 u32 u64 u128 usize }

macro_rules! impl_normalize {
    ($($t:ty)*) => ($(
        impl crate::linear::Normalize for super::Vertex<$t> {
            type Output = super::Vertex<f64>;

            fn norm(&self) -> Self::Output {
                use rayon::prelude::*;

                let norm_scala = self
                    .v
                    .par_iter()
                    .map(|v_i| (*v_i as f64).powi(2))
                    .sum::<f64>()
                    .sqrt();

                let mut v = vec![0.; self.len()];
                v.par_iter_mut().enumerate().for_each(|(idx, v_i)| {
                    *v_i = self[idx] as f64 / norm_scala;
                });

                Self::Output {
                    v,
                    vertical: self.vertical,
                }
            }
        }
    )*)
}

impl_normalize! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_inner_prod {
    ($($t:ty)*) => ($(
        impl crate::linear::InnerProduct for super::Vertex<$t> {
            type Output = $t;

            fn inner(&self) -> Self::Output {
                use rayon::prelude::*;

                self.v.par_iter().map(|v_i| *v_i * *v_i).sum::<$t>()
            }
        }
    )*)
}

impl_inner_prod! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
