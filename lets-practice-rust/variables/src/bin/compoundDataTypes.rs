// Compound data types represent a group of value. 

fn main() {
    let tup = ("Sanskar Mishra", 100_000); // tuples are written by using a comma separated list inside of parentheses.  

    // We can get values out of tuples in two ways: destructuring and dot notation. 
    // To destructure a tuple will create a new set of variables inside parentheses here we have channel and sub count. 
    let (channel, sub_count) = tup;  // setting this equal to our tuple we will take first variable set that equal to the first value in our tuple then it'll take the second value and set that equal to second value in our tuple. 
    println!("Channel: {}, Sub count: {}", channel, sub_count); // Channel: Sanskar Mishra, Sub count: 100000 

    // get values of tuples using dot notation. 
    let sub_count = tup.1; 
    println!("Sub count (dot notation): {}", sub_count); // Sub count (dot notation): 100000

    // tuples as well as arrays both start at index 0 
    // to declare arrays in rust we use a comma separated list but instead of parentheses we use brackets
    // in rust arrays are fixed length - if you want something that can change size dynamically you would have to use a vector 

    let error_codes = [200, 404, 500, 404]; 
    let not_found = error_codes[1]; 
    println!("Not found error: {}", not_found); // 404

    let x = error_codes[3]; 
    println!("X value: {}", x); // 404 

    let byte = [0; 8];  // create an array of 8 values all set to be 0 
    println!("Byte array: {:?}", byte); // Byte array: [0, 0, 0, 0, 0, 0, 0, 0]

    // examples
    println!("First error code: {}", error_codes[0]); // 200 
    println!("Channel name (dot notation): {}", tup.0); // Sanskar Mishra 
}


// How to run this file 
// cargo run --bin compoundDataTypes 
