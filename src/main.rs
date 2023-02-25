#[path = "./math/math.rs"]
mod math;
fn main() {
    println!("Hello world!");
    println!("{}", math::factorial(4));
    println!("{}", math::squared(12));
    let x:i32 = 4;
    print!("{}", x);
}

