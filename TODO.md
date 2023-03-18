# TODO
## Features that may be added
1. Linear Equation
    - Equate the value of variables between multiple linear equations
    - linear_function(m, n) -> Matrix<T> &larr; Something like this
        ```
        | 1 2 3 4 |
        | 1 0 3 5 |
        | 1 2 0 6 |
        | 0 2 3 7 |
        ```

## Features that will be added
1. Computation of triangular matrix from matrix
    - upper triangular matrix can be done by the following:
        1. Transpose the original matrix
        2. Compute lower triangular matrix of transposed matrix
        3. Revert the tranpose
2. Vectors, Matrices of complex numbers
    - Computation of complex numbers can done with vectors
        - (1 + j) * (2 - j) &rarr; 2 + (-j + 2j) - (-1 * 1)
        - [T; 2] &larr; array with length 2 of data type T

## Tests todo
1. ~~Test triangular matrix computation~~ (2023/03/18)
    1. ~~upper triangular~~
    2. ~~lower triangular~~
2. ~~Implement and test determinant of matrix~~ (2023/03/18)
    1. ~~implementation~~
    2. ~~test~~

## Changes
1. Use single Vector to represent datas for Matrix
    - Use input data (x, y reference slice) to get size \
    (assume data size doesn't change)
    - When converting to a vector with `to_vec`, convert it to 2D vector
    - Better for performance, and easy to check for falty matrix by doing size \
    checking. (check if data size is x * y)
