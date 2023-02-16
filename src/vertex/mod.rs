pub mod utils;

#[derive(Debug, PartialEq)]
pub struct Vertex<T> {
    v: Vec<T>,
    vertical: bool,
}

impl<T: Copy + std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div> Vertex<T> {
    pub fn new(vertex: &[T]) -> Self {
        Self {
            v: vertex.to_vec(),
            vertical: false,
        }
    }

    /**
    Returns empty Vertex.
     */
    pub fn empty() -> Self {
        Self {
            v: vec![],
            vertical: false,
        }
    }

    /**
    Performs vertex transpose.
     */
    pub fn t(&mut self) {
        self.vertical = !self.vertical;
    }

    /**
    Convert vertex with values of type A into vertex with values of type B.

    # Examples
    ```
    use slal::vertex::Vertex;

    let v = Vertex::new(&[1, 2, 3]);    // Vertex::<i32>
    let v_f64 = v.convert::<f64>();     // Vertex::<f64>
    ```
     */
    pub fn convert<U>(self) -> Vertex<U>
    where
        U: Copy
            + std::ops::Add
            + std::ops::Sub
            + std::ops::Mul
            + std::ops::Div
            + std::convert::From<T>,
    {
        let converted: Vec<U> = self.to_vec().iter().map(|&value| value.into()).collect();

        Vertex::<U> {
            v: converted,
            vertical: self.vertical,
        }
    }

    /**
    Check if vertex is transposed.
     */
    pub fn is_transposed(&self) -> bool {
        self.vertical
    }

    /**
    Check if vertex is empty.
     */
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    /**
    Clones `self` into `Vec<T>`.

    # Examples
    ```
    use slal::vertex::Vertex;

    let v = Vertex::<u32>::new(&[1, 2, 3]);
    let v_vec = v.to_vec(); // returns vec![1, 2, 3]
    ```
    */
    pub fn to_vec(&self) -> Vec<T> {
        self.v.clone()
    }

    /**
    Set vertex to new value.

    # Examples
    ```
    use slal::vertex::Vertex;

    let mut v = Vertex::new(&[1, 2, 3]);
    v.set_vertex(&[100, 200, 300]); // sets the value to [100, 200, 300]
    ```
     */
    pub fn set_vertex(&mut self, vertex: &[T]) {
        self.v = vertex.to_vec();
    }

    /**
    Get the length of a vertex.

    # Examples
    ```
    use slal::vertex::Vertex;

    let v = Vertex::new(&[1, 2, 3]);
    let v_len = v.len(); // 3
    ```
     */
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /**
    Get the type name of vertex

    # Examples
    ```
    use slal::vertex::Vertex;

    let v = Vertex::new(&[1, 2, 3]);
    let v_type_name = v.type_name(); // "slal::vertex::Vertex"
    ```
     */
    pub fn type_name(&self) -> &str {
        use std::any::type_name;

        type_name::<Vertex<T>>()
    }
}

#[cfg(test)]
mod test {
    use super::Vertex;

    #[test]
    fn i8_vertex() {
        let v = Vertex::<i8>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<i8>::new(&[1, 2, 3]));
    }

    #[test]
    fn u8_vertex() {
        let v = Vertex::<u8>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<u8>::new(&[1, 2, 3]));
    }

    #[test]
    fn i16_vertex() {
        let v = Vertex::<i16>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<i16>::new(&[1, 2, 3]));
    }

    #[test]
    fn u16_vertex() {
        let v = Vertex::<u16>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<u16>::new(&[1, 2, 3]));
    }

    #[test]
    fn i32_vertex() {
        let v = Vertex::<i32>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<i32>::new(&[1, 2, 3]));
    }

    #[test]
    fn u32_vertex() {
        let v = Vertex::<u32>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<u32>::new(&[1, 2, 3]));
    }

    #[test]
    fn i64_vertex() {
        let v = Vertex::<i64>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<i64>::new(&[1, 2, 3]));
    }

    #[test]
    fn u64_vertex() {
        let v = Vertex::<u64>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<u64>::new(&[1, 2, 3]));
    }

    #[test]
    fn i128_vertex() {
        let v = Vertex::<i128>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<i128>::new(&[1, 2, 3]));
    }

    #[test]
    fn u128_vertex() {
        let v = Vertex::<u128>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<u128>::new(&[1, 2, 3]));
    }

    #[test]
    fn isize_vertex() {
        let v = Vertex::<isize>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<isize>::new(&[1, 2, 3]));
    }

    #[test]
    fn usize_vertex() {
        let v = Vertex::<usize>::new(&[1, 2, 3]);

        assert_eq!(v, Vertex::<usize>::new(&[1, 2, 3]));
    }

    #[test]
    fn f32_vertex() {
        let v = Vertex::<f32>::new(&[1.0, 2.0, 3.0]);

        assert_eq!(v, Vertex::<f32>::new(&[1.0, 2.0, 3.0]));
    }

    #[test]
    fn f64_vertex() {
        let v = Vertex::<f64>::new(&[1.0, 2.0, 3.0]);

        assert_eq!(v, Vertex::<f64>::new(&[1.0, 2.0, 3.0]));
    }

    #[test]
    fn empty() {
        let v = Vertex::<i32>::empty();

        assert_eq!(v, Vertex::<i32>::new(&[]));
    }

    #[test]
    fn convert() {
        let v = Vertex::new(&[1, 2, 3]);
        let v_f64 = v.convert::<f64>();

        assert_eq!(v_f64, Vertex::new(&[1.0, 2.0, 3.0]));
    }

    #[test]
    fn is_empty() {
        let v = Vertex::<i32>::empty();

        assert!(v.is_empty());
    }

    #[test]
    fn is_transposed() {
        let mut v = Vertex::new(&[1, 2, 3]);
        v.t();

        assert!(v.is_transposed());
    }

    #[test]
    fn len() {
        let v = Vertex::new(&[1, 2, 3, 4, 5, 6]);

        assert_eq!(v.len(), 6);
    }

    #[test]
    fn set_vertex() {
        let mut v = Vertex::new(&[1, 2, 3]);
        v.set_vertex(&[30, 40, 50]);

        assert_eq!(v, Vertex::new(&[30, 40, 50]));
    }

    #[test]
    fn transpose() {
        let mut v = Vertex::new(&[1, 2, 3]);
        v.t();

        assert_eq!(
            v,
            Vertex::<i32> {
                v: vec![1, 2, 3],
                vertical: true
            }
        );
    }

    #[test]
    fn to_vec() {
        let v = Vertex::new(&[1 << 1, 1 << 11, 1 << 21]);

        assert_eq!(v.to_vec(), vec![1 << 1, 1 << 11, 1 << 21]);
    }

    #[test]
    fn type_name() {
        let v = Vertex::new(&[1, 2, 3]);

        assert_eq!(v.type_name(), "slal::vertex::Vertex<i32>");
    }
}
