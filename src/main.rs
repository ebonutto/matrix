use matrix::Matrix;

fn main() {
    let mut m1 = Matrix::from([[1, 2], [3, 4]]);
    let mut m2 = Matrix::from([[5, 6], [7, 8]]);

    println!("{}", m1);
    println!("{}", m2);

    m1.add(&m2);
    println!("{}", m1);
}
