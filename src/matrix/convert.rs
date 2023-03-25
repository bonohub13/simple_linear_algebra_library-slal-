macro_rules! impl_from {
    ($Small: ty, $Large: ty, $doc: expr) => {
        impl From<super::Matrix<$Small>> for super::Matrix<$Large> {
            #[doc = $doc]
            #[inline(always)]
            fn from(other: super::Matrix<$Small>) -> Self {
                let m: Vec<$Large> = other.m.iter().map(|m_ij| *m_ij as $Large).collect();

                super::Matrix::<$Large> {
                    m,
                    size: other.size,
                }
            }
        }
    };

    ($Small: ty, $Large: ty) => {
        impl_from!(
            $Small,
            $Large,
            concat!(
                "Converts `slal::matrix::Matrix<",
                stringify!($Small),
                ">` to `slal::matrix::Matrix<",
                stringify!($Large),
                ">` losslessly"
            )
        );
    };
}

// Unsigned -> Unsigned
impl_from! { u8, u16 }
impl_from! { u8, u32 }
impl_from! { u8, u64 }
impl_from! { u8, u128 }
impl_from! { u16, u32 }
impl_from! { u16, u64 }
impl_from! { u16, u128 }
impl_from! { u32, u64 }
impl_from! { u32, u128 }
impl_from! { u64, u128 }

// Signed -> Signed
impl_from! { i8, i16 }
impl_from! { i8, i32 }
impl_from! { i8, i64 }
impl_from! { i8, i128 }
impl_from! { i16, i32 }
impl_from! { i16, i64 }
impl_from! { i16, i128 }
impl_from! { i32, i64 }
impl_from! { i32, i128 }
impl_from! { i64, i128 }

// Unsigned -> Signed
impl_from! { u8, i16 }
impl_from! { u8, i32 }
impl_from! { u8, i64 }
impl_from! { u8, i128 }
impl_from! { u16, i32 }
impl_from! { u16, i64 }
impl_from! { u16, i128 }
impl_from! { u32, i64 }
impl_from! { u32, i128 }
impl_from! { u64, i128 }

// primitive integer number -> size
impl_from! { u16, usize }
impl_from! { u8, usize }
impl_from! { i16, usize }

// Signed -> Float
impl_from! { i8, f32 }
impl_from! { i8, f64 }
impl_from! { i16, f32 }
impl_from! { i16, f64 }
impl_from! { i32, f64 }

// Unsigned -> Float
impl_from! { u8, f32 }
impl_from! { u8, f64 }
impl_from! { u16, f32 }
impl_from! { u16, f64 }
impl_from! { u32, f64 }

// Float -> Float
impl_from! { f32, f64 }
