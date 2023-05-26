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

pub fn is_prime(num: i64) -> bool {
    if num <= 1{
        return false;
    }
    let max_factor: i64 = f64::ceil(f64::sqrt(num as f64)) as i64;
    for i in 2..max_factor+1 {
        if num % i == 0{
            return false;
        }
    }
    return true;
}

