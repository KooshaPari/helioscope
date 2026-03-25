# Mojo Matrix Module - High-performance matrix operations

from math import sqrt

# Matrix structure
struct Matrix:
    var data: List[List[Float64]]
    var rows: Int
    var cols: Int

    fn __init__(inout self, rows: Int, cols: Int, fill: Float64 = 0.0):
        self.rows = rows
        self.cols = cols
        self.data = List[List[Float64]]()
        for i in range(rows):
            row = List[Float64]()
            for j in range(cols):
                row.append(fill)
            self.data.append(row)

    fn __getitem__(self, i: Int, j: Int) -> Float64:
        return self.data[i][j]

    fn __setitem__(inout self, i: Int, j: Int, value: Float64):
        self.data[i][j] = value

# Matrix multiplication
fn matmul(a: Matrix, b: Matrix) -> Matrix:
    if a.cols != b.rows:
        raise Error("Matrix dimensions mismatch")
    
    result = Matrix(a.rows, b.cols, 0.0)
    for i in range(a.rows):
        for j in range(b.cols):
            var sum_val: Float64 = 0.0
            for k in range(a.cols):
                sum_val += a[i, k] * b[k, j]
            result[i, j] = sum_val
    return result

# Matrix addition
fn matadd(a: Matrix, b: Matrix) -> Matrix:
    if a.rows != b.rows or a.cols != b.cols:
        raise Error("Matrix dimensions mismatch")
    
    result = Matrix(a.rows, a.cols, 0.0)
    for i in range(a.rows):
        for j in range(a.cols):
            result[i, j] = a[i, j] + b[i, j]
    return result

# Matrix scalar multiplication
fn scale(matrix: Matrix, scalar: Float64) -> Matrix:
    result = Matrix(matrix.rows, matrix.cols, 0.0)
    for i in range(matrix.rows):
        for j in range(matrix.cols):
            result[i, j] = matrix[i, j] * scalar
    return result

# Transpose
fn transpose(matrix: Matrix) -> Matrix:
    result = Matrix(matrix.cols, matrix.rows, 0.0)
    for i in range(matrix.rows):
        for j in range(matrix.cols):
            result[j, i] = matrix[i, j]
    return result

# Identity matrix
fn identity(n: Int) -> Matrix:
    result = Matrix(n, n, 0.0)
    for i in range(n):
        result[i, i] = 1.0
    return result

# Determinant (2x2 only for simplicity)
fn determinant_2x2(matrix: Matrix) -> Float64:
    if matrix.rows != 2 or matrix.cols != 2:
        raise Error("Only 2x2 determinant supported")
    return matrix[0, 0] * matrix[1, 1] - matrix[0, 1] * matrix[1, 0]

# Frobenius norm
fn frobenius_norm(matrix: Matrix) -> Float64:
    var sum_sq: Float64 = 0.0
    for i in range(matrix.rows):
        for j in range(matrix.cols):
            sum_sq += matrix[i, j] ** 2
    return sqrt(sum_sq)

# Trace
fn trace(matrix: Matrix) -> Float64:
    if matrix.rows != matrix.cols:
        raise Error("Matrix must be square")
    
    var sum_val: Float64 = 0.0
    for i in range(matrix.rows):
        sum_val += matrix[i, i]
    return sum_val

# Zeros matrix
fn zeros(rows: Int, cols: Int) -> Matrix:
    return Matrix(rows, cols, 0.0)

# Ones matrix
fn ones(rows: Int, cols: Int) -> Matrix:
    return Matrix(rows, cols, 1.0)

fn main():
    # Test matrix operations
    a = Matrix(2, 2, 1.0)
    b = identity(2)
    c = matmul(a, b)
    print("Matrix operations ready")
