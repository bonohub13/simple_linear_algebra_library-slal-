/**
Trait for calculating dot product for vertices/matrices
 */
pub trait Dot<T>
where
    Self: Sized,
{
    type Output;

    fn dot(&self, other: &T) -> Self::Output;
}
