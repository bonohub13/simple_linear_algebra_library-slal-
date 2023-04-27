macro_rules! impl_magnitude_vertex {
    ($t:ty, $doc:expr) => {
        impl crate::linear::Magnitude for super::Vertex<$t> {
            type Output = f64;

            #[doc=$doc]
            fn magnitude(&self) -> Self::Output {
                use rayon::prelude::*;

                (0..self.len())
                    .into_par_iter()
                    .map(|idx| (self[idx] * self[idx]) as f64)
                    .sum::<f64>()
                    .sqrt()
            }
        }
    };

    ($t:ty) => {
        impl_magnitude_vertex!(
            $t,
            concat!(
                r"Computes magnitude of vertex (`Vertex::<",
                stringify!($t),
                r">`)\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Magnitude;\n\nlet v = Vertex::<",stringify!($t),r">::new(&[1, 2, 3]);\n\nassert!(v.magnitude() == (14. as f64).sqrt());\n```"
            )
        );
    };
}

impl_magnitude_vertex! { i8 }
impl_magnitude_vertex! { u8 }
impl_magnitude_vertex! { i16 }
impl_magnitude_vertex! { u16 }
impl_magnitude_vertex! { i32 }
impl_magnitude_vertex! { u32 }
impl_magnitude_vertex! { i64 }
impl_magnitude_vertex! { u64 }
impl_magnitude_vertex! { i128 }
impl_magnitude_vertex! { u128 }
impl_magnitude_vertex! { isize }
impl_magnitude_vertex! { usize }
impl_magnitude_vertex! { f32, concat!(
    r"Computes magnitude of vertex (`Vertex::<f32>`)\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Magnitude;\n\nlet v = Vertex::<f32>::new(&[0.1, 0.2, 0.3]);\n\nassert!(v.magnitude() == 0.14.sqrt());\n```"
) }
impl_magnitude_vertex! { f64, concat!(
    r"Computes magnitude of vertex (`Vertex::<f64>`)\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Magnitude;\n\nlet v = Vertex::<f64>::new(&[0.1, 0.2, 0.3]);\n\nassert!(v.magnitude() == 0.14.sqrt());\n```"
) }

macro_rules! impl_random_signed {
    ($t:ty, $doc1:expr, $doc2:expr) => {
        impl crate::linear::Random for super::Vertex<$t> {
            type Output = super::Vertex<$t>;
            type Size = usize;

            #[doc=$doc1]
            fn rand(size: Self::Size) -> Self::Output {
                use rand::{self, Rng};
                use rayon::prelude::*;

                const DELTA: f64 = 1e-6;

                let mut v = vec![0 as $t; size];
                v.par_iter_mut().for_each(|v_i| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *v_i = thread_rng.gen::<$t>()
                            * if thread_rng.gen::<i8>() % 2 == 1 {
                                -1 as $t
                            } else {
                                1 as $t
                            };

                        if (*v_i as f64).abs() > DELTA {
                            return;
                        }
                    }
                });

                Self::Output { v, vertical: false }
            }

            #[doc=$doc2]
            fn rand_transposed(size: Self::Size) -> Self::Output {
                use rand::{self, Rng};
                use rayon::prelude::*;

                const DELTA: f64 = 1e-6;

                let mut v = vec![0 as $t; size];
                v.par_iter_mut().for_each(|v_i| {
                    let mut thread_rng = rand::thread_rng();

                    loop {
                        *v_i = thread_rng.gen::<$t>()
                            * if thread_rng.gen::<i8>() % 2 == 1 {
                                -1 as $t
                            } else {
                                1 as $t
                            };

                        if (*v_i as f64).abs() > DELTA {
                            return;
                        }
                    }
                });

                Self::Output { v, vertical: true }
            }
        }
    };

    ($t:ty) => {
        impl_random_signed!(
            $t,
            concat!(
                r"Creates a new vertex (`slal::vertex::Vertex::<",
                stringify!($t),
                r">`) with random values.\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Random;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::rand(3);\n```",
            ),
            concat!(
                r"Creates a new transposed vertex (`slal::vertex::Vertex::<",
                stringify!($t),
                r">`) with random values.\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Random;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::rand_transposed(3);\n```",
            )
        );
    };
}

impl_random_signed! { i8 }
impl_random_signed! { i16 }
impl_random_signed! { i32 }
impl_random_signed! { i64 }
impl_random_signed! { i128 }
impl_random_signed! { isize }
impl_random_signed! { f32 }
impl_random_signed! { f64 }

macro_rules! impl_random_unsigned {
    ($t:ty, $doc1:expr, $doc2:expr) => {
        impl crate::linear::Random for super::Vertex<$t> {
            type Output = super::Vertex<$t>;
            type Size = usize;

            #[doc=$doc1]
            fn rand(size: Self::Size) -> Self::Output {
                use rand::{self, Rng};
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

                Self::Output { v, vertical: false }
            }

            #[doc=$doc2]
            fn rand_transposed(size: Self::Size) -> Self::Output {
                use rand::{self, Rng};
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

                Self::Output { v, vertical: true }
            }
        }
    };

    ($t:ty) => {
        impl_random_unsigned!(
            $t,
            concat!(
                r"Creates a new vertex (`slal::vertex::Vertex::<",
                stringify!($t),
                r">`) with random values.\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Random;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::rand(3);\n```",
            ),
            concat!(
                r"Creates a new transposed vertex (`slal::vertex::Vertex::<",
                stringify!($t),
                r">`) with random values.\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Random;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::rand_transposed(3);\n```",
            )
        );
    };
}

impl_random_unsigned! { u8 }
impl_random_unsigned! { u16 }
impl_random_unsigned! { u32 }
impl_random_unsigned! { u64 }
impl_random_unsigned! { u128 }
impl_random_unsigned! { usize }

macro_rules! impl_normalize {
    ($t:ty, $doc:expr) => {
        impl crate::linear::Normalize for super::Vertex<$t> {
            type Output = super::Vertex<f64>;

            #[doc=$doc]
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
    };

    ($t:ty) => {
        impl_normalize!(
            $t,
            concat!(
                r"Outputs a normalized vertex of the vertex.\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Normalize;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::new(&[1, 2, 3, 4]);\nlet norm = v.norm();\n\n/// | 1./(30.).sqrt() 2./(30.).sqrt() 3./(30.).sqrt() 4./(30.).sqrt() |\n```"
            )
        );
    };
}

impl_normalize! { i8 }
impl_normalize! { u8 }
impl_normalize! { i16 }
impl_normalize! { u16 }
impl_normalize! { i32 }
impl_normalize! { u32 }
impl_normalize! { i64 }
impl_normalize! { u64 }
impl_normalize! { i128 }
impl_normalize! { u128 }
impl_normalize! { isize }
impl_normalize! { usize }
impl_normalize! { f32, concat!(
    r"Outputs a normalized vertex of the vertex.\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Normalize;\n\nlet v = Vertex::<f32>::new(&[1., 2., 3., 4.]);\nlet norm = v.norm();\n\n/// | 1./(30.).sqrt() 2./(30.).sqrt() 3./(30.).sqrt() 4./(30.).sqrt() |\n```"
) }
impl_normalize! { f64, concat!(
    r"Outputs a normalized vertex of the vertex.\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::Normalize;\n\nlet v = Vertex::<f64>::new(&[1., 2., 3., 4.]);\nlet norm = v.norm();\n\n/// | 1./(30.).sqrt() 2./(30.).sqrt() 3./(30.).sqrt() 4./(30.).sqrt() |\n```"
) }

macro_rules! impl_inner_prod_float {
    ($t:ty, $doc:expr) => {
        impl crate::linear::InnerProduct for super::Vertex<$t> {
            type Output = f64;

            #[doc=$doc]
            fn inner(&self) -> Self::Output {
                use rayon::prelude::*;

                self.v.par_iter().map(|v_i| (*v_i as f64).powi(2)).sum()
            }
        }
    };

    ($t:ty) => {
        impl_inner_prod_float!(
            $t,
            concat!(
                r"Calculates the inner product of a vertex (`slal::vertex::Vertex<",
                stringify!($t),
                r">`)\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::InnerProduct;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::new(&[1., 2., 0.3]);\nlet inner_prod = v.inner();\n```"
            )
        );
    };
}

impl_inner_prod_float! { f32 }
impl_inner_prod_float! { f64 }

macro_rules! impl_inner_prod_i32_or_smaller {
    ($t:ty, $doc:expr) => {
        impl crate::linear::InnerProduct for super::Vertex<$t> {
            type Output = i32;

            #[doc=$doc]
            fn inner(&self) -> Self::Output {
                use rayon::prelude::*;

                self.v.par_iter().map(|v_i| (*v_i as i32).pow(2)).sum()
            }
        }
    };

    ($t:ty) => {
        impl_inner_prod_i32_or_smaller!(
            $t,
            concat!(
                r"Calculates the inner product of a vertex (`slal::vertex::Vertex<",
                stringify!($t),
                r">`)\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::InnerProduct;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::new(&[1, -2, 3]);\nlet inner_prod = v.inner();\n```"
            )
        );
    }
}

impl_inner_prod_i32_or_smaller! { i8 }
impl_inner_prod_i32_or_smaller! { i16 }
impl_inner_prod_i32_or_smaller! { i32 }

macro_rules! impl_inner_prod_u32_or_smaller {
    ($t:ty, $doc:expr) => {
        impl crate::linear::InnerProduct for super::Vertex<$t> {
            type Output = u32;

            #[doc=$doc]
            fn inner(&self) -> Self::Output {
                use rayon::prelude::*;

                self.v.par_iter().map(|v_i| (*v_i as u32)).sum()
            }
        }
    };

    ($t:ty) => {
        impl_inner_prod_u32_or_smaller!(
            $t,
            concat!(
                r"Calculates the inner product of a vertex (`slal::vertex::Vertex<",
                stringify!($t),
                r">`)\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::InnerProduct;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::new(&[1, 2, 3]);\nlet inner_prod = v.inner();\n```"
            )
        );
    };
}

impl_inner_prod_u32_or_smaller! { u8 }
impl_inner_prod_u32_or_smaller! { u16 }
impl_inner_prod_u32_or_smaller! { u32 }

macro_rules! impl_inner_prod_large_signed_int {
    ($t:ty, $doc:expr) => {
        impl crate::linear::InnerProduct for super::Vertex<$t> {
            type Output = i128;

            #[doc=$doc]
            fn inner(&self) -> Self::Output {
                use rayon::prelude::*;

                self.v.par_iter().map(|v_i| (*v_i as i128).pow(2)).sum()
            }
        }
    };

    ($t:ty) => {
        impl_inner_prod_large_signed_int!(
            $t,
            concat!(
                r"Calculates the inner product of a vertex (`slal::vertex::Vertex<",
                stringify!($t),
                r">`)\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::InnerProduct;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::new(&[1, -2, 3]);\nlet inner_prod = v.inner();\n```"
            )
        );
    }
}

impl_inner_prod_large_signed_int! { i64 }
impl_inner_prod_large_signed_int! { i128 }

macro_rules! impl_inner_prod_large_unsigned_int {
    ($t:ty, $doc:expr) => {
        impl crate::linear::InnerProduct for super::Vertex<$t> {
            type Output = u128;

            #[doc=$doc]
            fn inner(&self) -> Self::Output {
                use rayon::prelude::*;

                self.v.par_iter().map(|v_i| (*v_i as u128).pow(2)).sum()
            }
        }
    };

    ($t:ty) => {
        impl_inner_prod_large_unsigned_int!(
            $t,
            concat!(
                r"Calculates the inner product of a vertex (`slal::vertex::Vertex<",
                stringify!($t),
                r">`)\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::InnerProduct;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::new(&[1, 2, 3]);\nlet inner_prod = v.inner();\n```"
            )
        );
    };
}

impl_inner_prod_large_unsigned_int! { u64 }
impl_inner_prod_large_unsigned_int! { u128 }

macro_rules! impl_inner_prod_size {
    ($t:ty, $doc:expr) => {
        impl crate::linear::InnerProduct for super::Vertex<$t> {
            type Output = $t;

            #[doc=$doc]
            fn inner(&self) -> Self::Output {
                use rayon::prelude::*;

                self.v.par_iter().map(|v_i| v_i.pow(2)).sum()
            }
        }
    };

    ($t:ty) => {
        impl_inner_prod_size!(
            $t,
            concat!(
                r"Calculates the inner product of a vertex (`slal::vertex::Vertex<",
                stringify!($t),
                r">`)\n\n# Examples\n```\nuse slal::vertex::Vertex;\nuse slal::linear::InnerProduct;\n\nlet v = Vertex::<",
                stringify!($t),
                r">::new(&[1, 2, 3]);\nlet inner_prod = v.inner();\n```"
            )
        );
    };
}

impl_inner_prod_size! { isize }
impl_inner_prod_size! { usize }
