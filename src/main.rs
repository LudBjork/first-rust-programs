#[path = "./math/math.rs"]
mod math;
fn main() {
    println!("Hello world!");
    println!("{}", math::factorial(4));
    println!("{}", math::squared(12));
    println!("{}", math::divide(4, 2));
    println!("{}", math::divide(1, 3));
    println!("{}", math::isPrime(5));
}

