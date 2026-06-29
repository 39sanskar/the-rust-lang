fn main() {
    // Scalar data types represent a single value. Types are: Integers, Floating-point numbers, Booleans, Characters

    // ===== INTEGERS =====
    let a: i32 = 98_222;  // Decimal integer with underscore for readability
    let b: i32 = 0xff;    // Hexadecimal (base 16) - equals 255 in decimal
    let c: i32 = 0o77;    // Octal (base 8) - equals 63 in decimal
    let d: i32 = 0b1111_0000;  // Binary (base 2) with underscore for readability - equals 240 in decimal
    let e: u8 = b'A';   // Byte literal - gets the ASCII value of 'A' which is 65

    
    println!("===== INTEGERS =====");
    println!("a = {}", a);  // 98222
    println!("b = {}", b);  // 255
    println!("c = {}", c);  // 63
    println!("d = {}", d);  // 240
    println!("e = {}", e);  // 65
    
    // Integer Overflow 
    let s: u8 = 255; // 8-bit unsigned integer which can hold a max value of 255. if i try to set this number greater than 255 in debug builds rust will panic and in release builds rust will perform 2's complement wrapping which means value is greater than the maximum will wrap around back to the minimum values so 256 would become 0 and 257 would become 1 also if you have a language server running and you try to change this to 256 you will get an error warning you of the overflow.  
    println!("s = {}", s); // 255 


    // ===== FLOATING-POINT NUMBERS =====
    let f: f32 = 3.14;           // 32-bit floating point
    let g: f64 = 3.14159265359;  // 64-bit floating point (default)
    let h: f64 = 2.0e3;          // Scientific notation: 2.0 * 10^3 = 2000.0

    println!("\n===== FLOATING-POINT NUMBERS =====");
    println!("f (f32) = {}", f);  // 3.14
    println!("g (f64) = {}", g);  // 3.14159265359
    println!("h (scientific) = {}", h);  // 2000


    // ===== BOOLEANS =====
    let is_rust_fun: bool = true;
    let is_finished: bool = false;
    let comparison_result = 5 > 3;  // This evaluates to true

    println!("\n===== BOOLEANS =====");
    println!("is_rust_fun = {}", is_rust_fun);        // true
    println!("is_finished = {}", is_finished);        // false
    println!("5 > 3 = {}", comparison_result);        // true


    // Boolean operations
    println!("NOT true = {}", !true);                 // false
    println!("true AND false = {}", true && false);   // false
    println!("true OR false = {}", true || false);    // true


    // ===== CHARACTERS =====
    let heart_eyed_cat = '😻';    // Unicode character
    let letter_z = 'z';           // ASCII character
    let number_char = '7';        // Character '7', not number 7
    let space = ' ';              // Space character
    let unicode_char = '\u{1F600}'; // Unicode character using escape sequence (😀)

    println!("\n===== CHARACTERS =====");
    println!("heart_eyed_cat = {}", heart_eyed_cat);  // 😻
    println!("letter_z = {}", letter_z);              // z
    println!("number_char = {}", number_char);        // 7
    println!("space = '{}'", space);                  // (space)
    println!("unicode_char = {}", unicode_char);      // 😀


    // Character operations
    let char_a = 'A';
    let char_b = ((char_a as u8) + 1) as char;  // Convert to u8, add 1, convert back
    println!("char_b (A+1) = {}", char_b);       // B

    
    // ===== INTEGER SIZES =====
    // Signed integers: i8, i16, i32, i64, i128, isize
    // Unsigned integers: u8, u16, u32, u64, u128, usize
    let small_num: i8 = 100;
    let big_num: i64 = 9_223_372_036_854_775_807;  // Maximum i64 value
    
    println!("\n===== INTEGER SIZES =====");
    println!("small_num (i8) = {}", small_num);
    println!("big_num (i64) = {}", big_num);
}



// How to run this file 
// cargo run --bin scalardatatypes 
