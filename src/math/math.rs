pub fn factorial(x: i32) -> i32 {
    if x == 1{
        return 1;
    } else {
        return x * factorial(x-1);
    }
}

pub fn squared(x : i128) -> i128 {
    return x*x;
}