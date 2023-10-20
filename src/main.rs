use std::io;

fn main() {
    // Get the first number from user
    let first_number = input(); 
    // Convert the string to an integer
    let first_number: i32 = first_number.trim().parse().expect("Please type a number!");

    // Get the operator from user
    let operator = input();

    // Get the second number from user
    let second_number = input(); 
    // Convert the string to an integer
    let second_number: i32 = second_number.trim().parse().expect("Please type a number!");

    let mut answer: i32 = 0;

    match operator.trim() {
        "+" => {
            answer = add(first_number, second_number);
        }
        "-" => {
            answer = subtract(first_number, second_number);
        }
        "*" => {
            answer = multiply(first_number, second_number);
        }
        "/" => {
            answer = divide(first_number, second_number); 
        }
        "**" => {
            answer = power(first_number, second_number);
        }
        "%" => {
            answer = modulo(first_number, second_number);
        }
        _ => println!("Pick a valid operator.")
    }

    println!("Answer: {}", answer);
}

/// Get input from user
/// 
/// # Examples
/// ```
/// let mut first_number = input(); // 1
/// println!("{}", first_number);
/// // 1
/// ```
fn input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

/// Adds two integers together.
/// 
/// # Examples
/// ```
/// let a: i32 = 1;
/// let b: i32 = 2;
/// 
/// add(a, b); // 3
/// ```
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

/// Subtracts the first integer
/// from the second integer.
/// 
/// # Examples
/// ```
/// let a: i32 = 5;
/// let b: i32 = 4;
/// 
/// subtract(a, b); // 1
/// ```
fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}

/// Multiples two integers together.
/// 
/// # Examples
/// ```
/// let a: i32 = 4;
/// let b: i32 = 2;
/// 
/// multiply(a, b); // 8
/// ```
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

/// Divides the first integer
/// by the second integer.
/// 
/// # Examples
/// ```
/// let a: i32 = 4;
/// let b: i32 = 2;
/// 
/// divide(a, b); // 2
/// ```
fn divide(a: i32, b: i32) -> i32 {
    return a / b;
}

/// Returns the first integer to the
/// power of the second integer.
/// 
/// # Examples
/// ```
/// let a: i32 = 2;
/// let b: i32 = 3;
/// 
/// power(a, b); // 8
/// ```
fn power(a: i32, b: i32) -> i32 {
    return a.pow(b as u32);
}

/// Returns the mod of the first integer
/// divided by the second integer.
/// 
/// # Examples
/// ```
/// let a: i32 = 6;
/// let b: i32 = 3;
/// 
/// modulo(a, b); // 0
/// ```
fn modulo(a: i32, b: i32) -> i32 {
    return a % b;
}