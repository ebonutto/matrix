mod matrix;

use matrix::Matrix;

fn main() {
    let matrix = Matrix::zeros(3, 1);
    matrix.print();
}
