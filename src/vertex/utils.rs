macro_rules! vertex_index {
    ($($t:ty)*) => ($(
        impl std::ops::Index<usize> for super::Vertex<$t> {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                &self.v[index]
            }
        }

        impl std::ops::IndexMut<usize> for super::Vertex<$t> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.v[index]
            }
        }

        impl std::ops::Index<usize> for &super::Vertex<$t> {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                &self.v[index]
            }
        }

        impl std::ops::Index<usize> for &mut super::Vertex<$t> {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                &self.v[index]
            }
        }

        impl std::ops::IndexMut<usize> for &mut super::Vertex<$t> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.v[index]
            }
        }
    )*)
}

vertex_index! {i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64}

macro_rules! impl_round {
    ($t:ty, $doc:expr) => {
        impl crate::utils::Round for super::Vertex<$t> {
            #[doc=$doc]
            fn round(&mut self) {
                use rayon::prelude::*;

                self.v.par_iter_mut().for_each(|val| {
                    *val = val.round();
                });
            }
        }
    };

    ($t:ty) => {
        impl_round!(
            $t,
            concat!(
                r"Rounds floating point numbers inside `Vertex::<",
                stringify!($t),
                r">`\n\n\n# Examples\n\n```\n\nuse slal::vertex::Vertex;\n\nuse slal::utils::Round;\n\n\nlet mut v = Vertex::<",
                stringify!($t),
                r">::new(&[1.1, 2.02, 3.003]);\n\n\nv.round();\n\n\nassert!(v == Vertex::<",
                stringify!($t),
                r">::new(&[1., 2., 3.]));\n\n```"
            )
        );
    };
}

impl_round! { f32 }
impl_round! { f64 }
