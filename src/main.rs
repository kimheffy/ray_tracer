mod tuple;

const EPSILON: f64 = 0.00001;

fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

fn main() {
    println!("Hello, world!");
}
