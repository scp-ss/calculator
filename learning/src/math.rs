pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

//pub fn square(a: i32) -> i32 {
//    a * a
//}

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
//pub fn exponent(a: i32, b: i32) -> i32 {
//  a.pow(b)
//}

//nneds fixin mf, stay AHRD!
