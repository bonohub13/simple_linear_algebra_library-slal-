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
