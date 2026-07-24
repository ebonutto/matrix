use matrix::Vector;

fn main() {
    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));
    // 0.0

    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));
    // 2.0

    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{}", u.dot(&v));
    // 9.0
}
