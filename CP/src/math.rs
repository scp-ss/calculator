pub fn power(base: i32, exponent: u32) -> i32 {
    if exponent == 0 {
        return 1;
    }

    let mut result = base;
    for _ in 1..exponent {
        result *= base;
    }

    result
}

fn main() {
    let a = 2;
    let b = 10;
    let result = power(a, b);
    println!("{} raised to {} is {}", a, b, result);
}
