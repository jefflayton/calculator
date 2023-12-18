use std::io;

fn main() {
    let operation: String = input();

    let answer: i32 = calculate_operation(operation);
    println!("{}", answer);
}

fn calculate_operation(operation: String) -> i32 {
    let mut result: i32 = 0;
    let mut last_operator: Option<char> = None;

    for character in operation.chars() {
        if character.is_numeric() {
            // Parse the number and evaluate.
            match character.to_digit(10) {
                Some(num) => {
                    let number: i32 = num as i32;
                    // If last_operator is defined or None
                    match last_operator {
                        Some(operator) => {
                            result = evaluate(operator, result, number);
                            last_operator = None;
                        }
                        None => {
                            result += number;
                        }
                    }
                }
                None => {}
            }
        } else if is_operator(character) {
            last_operator = Some(character);
        }
    }

    return result;
}

fn is_operator(operator: char) -> bool {
    let operators: &str = "+-*/%^";
    return operators.contains(operator);
}

fn evaluate(operator: char, first_number: i32, second_number: i32) -> i32 {
    return match operator {
        '+' => add(first_number, second_number),
        '-' => subtract(first_number, second_number),
        '*' => multiply(first_number, second_number),
        '/' => divide(first_number, second_number),
        '%' => modulo(first_number, second_number),
        '^' => power(first_number, second_number),
        _ => {
            println!("Operator not found.");
            return -1;
        }
    };
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
