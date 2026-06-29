// Comprehensive Rust Functions Examples

// 1. Basic function with parameters and return value
fn add_numbers(x: i32, y: i32) -> i32 {
    println!("Adding {} and {}", x, y);
    x + y  // Implicit return (no semicolon)
}

// 2. Function with multiple return values using tuples
fn calculate_stats(numbers: &[i32]) -> (i32, i32, f64) {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as i32;
    let average = sum as f64 / numbers.len() as f64;
    (sum, count, average)
}

// 3. Function with conditional logic
fn is_even_or_odd(num: i32) -> &'static str {
    if num % 2 == 0 {
        "even"
    } else {
        "odd"
    }
}

// 4. Function with pattern matching
fn get_grade(score: u8) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "Invalid score"
    }
}

// 5. Function with mutable parameters
fn increment_value(x: &mut i32) {
    *x += 1;
    println!("Value incremented to: {}", x);
}

// 6. Function with ownership transfer
fn take_ownership(s: String) -> String {
    println!("Took ownership of: {}", s);
    s  // Return ownership back
}

// 7. Function with borrowing (references)
fn calculate_rectangle_area(length: &f64, width: &f64) -> f64 {
    length * width
}

// 8. Generic function
fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 9. Function with default parameters (using Option)
fn greet_person(name: &str, title: Option<&str>) -> String {
    match title {
        Some(t) => format!("Hello, {} {}!", t, name),
        None => format!("Hello, {}!", name)
    }
}

// 10. Function that returns a closure
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

// 11. Recursive function
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// 12. Function with error handling
fn divide_numbers(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("=== Rust Functions Comprehensive Examples ===\n");

    // 1. Basic function call
    println!("1. Basic Function:");
    let sum = add_numbers(11, 22);
    println!("Sum: {}\n", sum);

    // 2. Multiple return values
    println!("2. Multiple Return Values:");
    let numbers = [10, 20, 30, 40, 50];
    let (total, count, avg) = calculate_stats(&numbers);
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}, Count: {}, Average: {:.2}\n", total, count, avg);

    // 3. Conditional logic
    println!("3. Conditional Logic:");
    let test_numbers = [4, 7, 12, 15];
    for &num in &test_numbers {
        println!("{} is {}", num, is_even_or_odd(num));
    }
    println!();

    // 4. Pattern matching
    println!("4. Pattern Matching:");
    let scores = [95, 82, 76, 43, 105];
    for &score in &scores {
        println!("Score {} = Grade {}", score, get_grade(score));
    }
    println!();

    // 5. Mutable parameters
    println!("5. Mutable Parameters:");
    let mut value = 10;
    println!("Original value: {}", value);
    increment_value(&mut value);
    println!("Final value: {}\n", value);

    // 6. Ownership transfer
    println!("6. Ownership Transfer:");
    let my_string = String::from("Hello, Rust!");
    let returned_string = take_ownership(my_string);
    println!("Got back: {}\n", returned_string);

    // 7. Borrowing
    println!("7. Borrowing:");
    let length = 5.5;
    let width = 3.2;
    let area = calculate_rectangle_area(&length, &width);
    println!("Rectangle {} x {} has area: {:.2}\n", length, width, area);

    // 8. Generic function
    println!("8. Generic Function:");
    let numbers_list = [34, 50, 25, 100, 65];
    let largest_number = find_largest(&numbers_list);
    println!("Largest number: {}", largest_number);
    
    let chars_list = ['y', 'm', 'a', 'q'];
    let largest_char = find_largest(&chars_list);
    println!("Largest character: {}\n", largest_char);

    // 9. Default parameters
    println!("9. Default Parameters:");
    println!("{}", greet_person("Alice", Some("Dr.")));
    println!("{}", greet_person("Bob", None));
    println!();

    // 10. Function returning closure
    println!("10. Function Returning Closure:");
    let doubler = create_multiplier(2);
    let tripler = create_multiplier(3);
    println!("5 doubled: {}", doubler(5));
    println!("5 tripled: {}", tripler(5));
    println!();

    // 11. Recursive function
    println!("11. Recursive Function:");
    for i in 0..6 {
        println!("{}! = {}", i, factorial(i));
    }
    println!();

    // 12. Error handling
    println!("12. Error Handling:");
    match divide_numbers(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e)
    }
    
    match divide_numbers(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e)
    }
}

// how to run this code file 
// cargo run --bin function 


/*
Output: 

=== Rust Functions Comprehensive Examples ===

1. Basic Function:
Adding 11 and 22
Sum: 33

2. Multiple Return Values:
Numbers: [10, 20, 30, 40, 50]
Sum: 150, Count: 5, Average: 30.00

3. Conditional Logic:
4 is even
7 is odd
12 is even
15 is odd

4. Pattern Matching:
Score 95 = Grade A
Score 82 = Grade B
Score 76 = Grade C
Score 43 = Grade F
Score 105 = Grade Invalid score

5. Mutable Parameters:
Original value: 10
Value incremented to: 11
Final value: 11

6. Ownership Transfer:
Took ownership of: Hello, Rust!
Got back: Hello, Rust!

7. Borrowing:
Rectangle 5.5 x 3.2 has area: 17.60

8. Generic Function:
Largest number: 100
Largest character: y

9. Default Parameters:
Hello, Dr. Alice!
Hello, Bob!

10. Function Returning Closure:
5 doubled: 10
5 tripled: 15

11. Recursive Function:
0! = 1
1! = 1
2! = 2
3! = 6
4! = 24
5! = 120

12. Error Handling:
10 / 2 = 5
Error: Cannot divide by zero!

*/
