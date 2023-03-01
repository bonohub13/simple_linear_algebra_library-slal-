/**
   Calculation of dot product for vertices/matrices
*/
pub trait Dot<T>
where
    Self: Sized,
{
    type Output;

    fn dot(&self, other: &T) -> Self::Output;
}

/**
   Calculation of cross product for vertices/matrices
*/
pub trait Cross<T>
where
    Self: Sized,
{
    type Output;

    fn cross(&self, other: &T) -> Self::Output;
}

/**
   Calculation of magnitude for vertex
*/
pub trait Magnitude {
    type Output;

    fn magnitude(&self) -> Self::Output;
}
