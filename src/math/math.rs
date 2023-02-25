pub fn factorial(x: i32) -> i32 {
    if x == 1{
        return 1;
    } else {
        return x * factorial(x-1);
    }
}

pub fn squared(x : i32) -> i32 {
    return x*x;
}

pub fn divide(x: i64, y: i64) -> f64{
    if y == 0 { // Not proper error handling, but it's ok for now 
        println!("ERROR: Division by 0");
        return 0.0;
    } else {
        return (x as f64/y as f64) as f64;
    }
}   
