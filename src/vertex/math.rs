macro_rules! impl_add {
    ($($t:ty)*) => ($(
        impl std::ops::Add for super::Vertex<$t> {
            type Output = super::Vertex<$t>;

            fn add(self, other: super::Vertex<$t>) -> super::Vertex<$t> {
                use super::Vertex;

                if self.len() != other.len()
                    || self.is_transposed() != other.is_transposed() {
                    panic!("Cannot add {:?} and {:?}.", self, other);

                }

                let retval: Vec<$t> = self
                    .to_vec()
                    .iter()
                    .enumerate()
                    .map(|(idx, &v_i)| v_i + other.to_vec()[idx])
                    .collect();
                let mut retval_vertex = Vertex::<$t>::new(retval.as_slice());

                if self.is_transposed() {
                    retval_vertex.t();
                }

                retval_vertex
            }
        }
    )*)
}

impl_add! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! impl_sub {
    ($($t:ty)*) => ($(
        impl std::ops::Sub for super::Vertex<$t>
        {
            type Output = super::Vertex<$t>;

            fn sub(self, other: super::Vertex<$t>) -> Self {
                use super::Vertex;

                if self.len() != other.len()
                    || self.is_transposed() != other.is_transposed() {
                    panic!("Cannot substract {:?} from, {:?}.", other, self);
                }

                let retval: Vec<$t> = self
                    .to_vec()
                    .iter()
                    .enumerate()
                    .map(|(idx, &v_i)| v_i - other.to_vec()[idx])
                    .collect();
                let mut retval_vertex = Vertex::<$t>::new(retval.as_slice());

                if self.is_transposed() {
                    retval_vertex.t();
                }

                retval_vertex
            }
        }
    )*)
}

impl_sub! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
