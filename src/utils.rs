pub trait Round {
    /**
    Rounds all floating numbers in matrix/vertex
     */
    fn round(&mut self);
}

pub trait RPY {
    type Output;

    /**
    Outputs the position when p0 (x, y, z, q) is rotated based on its X, Y, Z axis.
    For angular rotation, radian is used.

    # Example
    ```
    ```
     */
    fn rpy(&self, rpy: [f64; 3]) -> Self::Output;
}
