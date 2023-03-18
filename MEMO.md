# Memos for linear algebra

## General algorithm for calculation of triangular matrix based on Doolittle's method
- A = LU
    - L: lower triangular matrix of matrix A
    - U: upper triangular matrix of matrix A
- L: size 3x3
    - L: [l1, l2, l3]^t
        - li1:
            - 1         if i == 1
            - ai1 / a11 if >= 2
        - li2:
            - 0                                                     if i == 1
            - 1                                                     if 1 == 2
            - (a11 * ai2 - a12 * ai1) / (a22 * a11 - a21 * a12)     if i > 2
        - li3:
            - 0                                                     if i < 3
            - 1                                                     if i == 3
            - (ai3 - li1*u13 - li2*u23) / u33                       if i > 3
    - l1: [1,       0,      0]
    - l2: [l21,     1,      0]
    - l3: [l31,     l32,    0]
- U: size 3x
    - U: [u1, u2, u3]^t
        - i_MAX: vertical size
        - j_MAX: horizontal size
        - uii:
            - aii - sum([li1*ui4 for i in range(i_MAX)])
        - uij:
            - 0                                                     if i >= (j + 1) && i > 1
            - aij - li1*uij - li2*u2j - ...                         else
    - u1: [u11, u12, u13]
        - u1 == a1
    - u2: [0,   u22, u23]
        - u2j:
            - 0                         if j == 1
            - a2j - (a21 * a1j) / a11   if j >= 2
    - u3: [0,   0,   u33]
        - u3j:
            - 0                                     if j < 3
            - a3j - (a31 * a1j) / a11 - l32 * u2j   if j >= 3