macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl std::ops::Add for super::Vertex<$t> {
            type Output = super::Vertex<$t>;

            fn add(self, other: super::Vertex<$t>) -> super::Vertex<$t> {
                use super::Vertex;

                if self.len() == other.len() && self.is_transposed() == other.is_transposed() {
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
                } else {
                    panic!(
                        "Cannot add {:?} and {:?}.",
                        self.type_name(),
                        other.type_name()
                    );
                }
            }
        }
    )*)
}

add_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        impl std::ops::Sub for super::Vertex<$t>
        {
            type Output = super::Vertex<$t>;

            fn sub(self, other: super::Vertex<$t>) -> Self {
                use super::Vertex;

                if self.len() == other.len() && self.is_transposed() == other.is_transposed() {
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
                } else {
                    panic!(
                        "Cannot substract {:?} from, {:?}.",
                        other.type_name(),
                        self.type_name()
                    );
                }
            }
        }
    )*)
}

sub_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! mul_scala_impl {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<$t> for super::Vertex<$t>
        {
            type Output = super::Vertex<$t>;

            fn mul(self, other: $t) -> super::Vertex<$t> {
                use super::Vertex;

                let retval: Vec<$t> = self
                    .to_vec()
                    .iter()
                    .map(|&v_i| v_i * other)
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

mul_scala_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! mul_with_scala_impl {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Vertex<$t>> for $t
        {
            type Output = super::Vertex<$t>;

            fn mul(self, other: super::Vertex<$t>) -> super::Vertex<$t> {
                use super::Vertex;

                let retval: Vec<$t> = other
                    .to_vec()
                    .iter()
                    .map(|&v_i| v_i * self)
                    .collect();
                let mut retval_vertex = Vertex::<$t>::new(retval.as_slice());

                if other.is_transposed() {
                    retval_vertex.t();
                }

                retval_vertex
            }
        }
    )*)
}

mul_with_scala_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl std::ops::Mul<super::Vertex<$t>> for super::Vertex<$t>
        {
            type Output = crate::matrix::Matrix<$t>;

            fn mul(self, other: Self) -> crate::matrix::Matrix<$t> {
                use crate::matrix::Matrix;

                if self.is_transposed() && !other.is_transposed() {
                    let rv: Vec<Vec<$t>> = self
                        .to_vec()
                        .iter()
                        .map(|&v_i| (v_i * other.clone()).to_vec())
                        .collect();
                    let rv_slice: Vec<&[$t]> = rv
                        .iter()
                        .map(|rv_i| rv_i.as_slice())
                        .collect();

                    Matrix::<$t>::new(rv_slice.as_slice()).unwrap()
                } else if !self.is_transposed() && other.is_transposed() {
                    if self.len() == other.len() {
                        let v1 = self.to_vec();
                        let v2 = other.to_vec();

                        let rv: Vec<$t> = v1
                            .iter()
                            .enumerate()
                            .map(|(idx, &v1_i)| v1_i * v2[idx])
                            .collect();

                        Matrix::<$t>::new(&[&[rv.iter().sum()]]).unwrap()
                    } else {
                        panic!("Size of both vectors must match when in multiplication of V.t x W");
                    }
                } else {
                    panic!("Cannot add two vectors when either is not transposed");
                }
            }
        }
    )*)
}

mul_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 }
