// Industry-Relevant Rust Function Examples

// 1. Web API Handler Function - User registration with validation
fn handle_user_registration(username: &str, email: &str, password: &str) -> Result<String, String> {
    // Input validation
    if username.is_empty() || email.is_empty() || password.len() < 8 {
        return Err("Invalid input: Username/email required, password min 8 chars".to_string());
    }
    
    // Email validation (simplified)
    if !email.contains('@') || !email.contains('.') {
        return Err("Invalid email format".to_string());
    }
    
    // Simulate database operation
    println!("Registering user: {} with email: {}", username, email);
    
    // Return success message
    Ok(format!("User {} registered successfully with ID: user_{}", username, 12345))
}

// 2. Database Connection Pool Manager - Connection pooling
fn get_database_connection(pool_size: usize) -> Result<Vec<String>, String> {
    if pool_size == 0 {
        return Err("Pool size must be greater than 0".to_string());
    }
    
    let mut connections = Vec::new();
    for i in 0..pool_size {
        let connection = format!("db_connection_{}", i);
        connections.push(connection);
    }
    
    println!("Created database connection pool with {} connections", pool_size);
    Ok(connections)
}

// 3. Financial Calculation Function - Loan payment calculations
fn calculate_loan_payment(principal: f64, annual_rate: f64, months: u32) -> Result<f64, String> {
    if principal <= 0.0 || annual_rate < 0.0 || months == 0 {
        return Err("Invalid loan parameters".to_string());
    }
    
    let monthly_rate = annual_rate / 12.0 / 100.0;
    let payment = principal * (monthly_rate * (1.0 + monthly_rate).powi(months as i32)) 
                 / ((1.0 + monthly_rate).powi(months as i32) - 1.0);
    
    Ok((payment * 100.0).round() / 100.0) // Round to 2 decimal places
}

// 4. Data Processing Pipeline Function - Sales data analysis
fn process_sales_data(sales: Vec<f64>) -> (f64, f64, f64, usize) {
    if sales.is_empty() {
        return (0.0, 0.0, 0.0, 0);
    }
    
    let total: f64 = sales.iter().sum();
    let average = total / sales.len() as f64;
    let maximum = sales.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let minimum = sales.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    
    // Use the minimum variable to avoid warning
    println!("Sales range: ${:.2} - ${:.2}", minimum, maximum);
    
    (total, average, maximum, sales.len())
}

// 5. Authentication Function - Permission checking
fn authenticate_user(token: &str, required_permissions: &[&str]) -> Result<bool, String> {
    // Simulate token validation
    if token.is_empty() {
        return Err("Authentication token required".to_string());
    }
    
    // Simulate permission checking
    let user_permissions = vec!["read", "write", "delete"];
    
    // Check if all required permissions are present
    let has_permission = required_permissions.iter().all(|&perm| {
        user_permissions.contains(&perm)
    });
    
    if has_permission {
        Ok(true)
    } else {
        Err("Insufficient permissions".to_string())
    }
}

// 6. Error Handling with Custom Error Types - Custom business errors
#[derive(Debug)]
enum BusinessError {
    ValidationError(()), // Changed to unit type to suppress warning
    DatabaseError(()),   // Changed to unit type to suppress warning
    NetworkError(()),    // Changed to unit type to suppress warning
}

fn process_payment(amount: f64, card_number: &str) -> Result<String, BusinessError> {
    // Validate amount
    if amount <= 0.0 {
        return Err(BusinessError::ValidationError(()));
    }
    
    // Validate card number (simplified)
    if card_number.len() != 16 || !card_number.chars().all(|c| c.is_digit(10)) {
        return Err(BusinessError::ValidationError(()));
    }
    
    // Simulate payment processing
    if amount > 10000.0 {
        // Use DatabaseError variant to avoid dead code warning
        return Err(BusinessError::DatabaseError(()));
    }
    
    // Simulate network error for demonstration
    if amount == 9999.99 {
        return Err(BusinessError::NetworkError(()));
    }
    
    Ok(format!("Payment of ${:.2} processed successfully", amount))
}

// 7. Configuration Management Function - Config validation
fn validate_config(config: &std::collections::HashMap<String, String>) -> Result<(), String> {
    let required_fields = ["database_url", "api_key", "port"];
    
    for field in &required_fields {
        if !config.contains_key(*field) {
            return Err(format!("Missing required configuration field: {}", field));
        }
    }
    
    // Validate port number
    if let Some(port_str) = config.get("port") {
        match port_str.parse::<u16>() {
            Ok(port) if port > 0 => {},
            _ => return Err("Invalid port number".to_string()),
        }
    }
    
    Ok(())
}

// 8. Caching Function - TTL-based caching
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn cache_data<T: Clone>(
    cache: &mut HashMap<String, (T, u64)>, 
    key: String, 
    value: T, 
    ttl_seconds: u64
) -> bool {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let expiry_time = current_time + ttl_seconds;
    cache.insert(key, (value, expiry_time));
    true
}

fn get_cached_data<T: Clone>(
    cache: &HashMap<String, (T, u64)>, 
    key: &str
) -> Option<T> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    if let Some((value, expiry)) = cache.get(key) {
        if *expiry > current_time {
            return Some(value.clone());
        }
    }
    None
}

// 9. Logging Function - Structured logging
fn log_event(level: &str, message: &str, metadata: Option<&std::collections::HashMap<String, String>>) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    print!("[{}] {} - {}", level, timestamp, message);
    
    if let Some(meta) = metadata {
        print!(" | Metadata: {:?}", meta);
    }
    println!();
}

// 10. Rate Limiting Function - API rate control
fn check_rate_limit(_user_id: &str, max_requests: usize, time_window: u64) -> Result<bool, String> {
    // In a real application, this would check against a database or cache
    // For demo purposes, we'll simulate with a simple check
    
    if max_requests == 0 || time_window == 0 {
        return Err("Invalid rate limit parameters".to_string());
    }
    
    // Simulate request count (in real app, this would be from storage)
    let request_count = 5; // Simulated current request count
    
    if request_count < max_requests {
        Ok(true) // Allow request
    } else {
        Err("Rate limit exceeded".to_string())
    }
}

fn main() {
    println!("=== Industry-Relevant Rust Functions ===\n");

    // 1. User Registration
    println!("1. User Registration:");
    match handle_user_registration("john_doe", "john@example.com", "password123") {
        Ok(msg) => println!("✓ Success: {}", msg),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();

    // 2. Database Connection
    println!("2. Database Connection Pool:");
    match get_database_connection(5) {
        Ok(connections) => println!("✓ Created {} connections: {:?}", connections.len(), connections),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();

    // 3. Financial Calculation
    println!("3. Loan Payment Calculation:");
    match calculate_loan_payment(100000.0, 5.5, 360) {
        Ok(payment) => println!("✓ Monthly payment: ${:.2}", payment),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();

    // 4. Data Processing
    println!("4. Sales Data Processing:");
    let sales_data = vec![1200.50, 980.75, 1500.00, 890.25, 2100.00];
    let (total, avg, max, count) = process_sales_data(sales_data);
    println!("✓ Total: ${:.2}, Average: ${:.2}, Max: ${:.2}, Count: {}", total, avg, max, count);
    println!();

    // 5. Authentication
    println!("5. User Authentication:");
    let permissions = vec!["read", "write"];
    match authenticate_user("valid_token_123", &permissions) {
        Ok(_) => println!("✓ User authenticated successfully"),
        Err(e) => println!("✗ Authentication failed: {}", e),
    }
    println!();

    // 6. Payment Processing
    println!("6. Payment Processing:");
    match process_payment(250.75, "1234567890123456") {
        Ok(msg) => println!("✓ {}", msg),
        Err(e) => println!("✗ Payment failed: {:?}", e),
    }
    
    // Test network error
    match process_payment(9999.99, "1234567890123456") {
        Ok(msg) => println!("✓ {}", msg),
        Err(e) => println!("✗ Network error: {:?}", e),
    }
    println!();

    // 7. Configuration Validation
    println!("7. Configuration Validation:");
    let mut config = std::collections::HashMap::new();
    config.insert("database_url".to_string(), "postgres://localhost:5432".to_string());
    config.insert("api_key".to_string(), "secret_key_123".to_string());
    config.insert("port".to_string(), "8080".to_string());
    
    match validate_config(&config) {
        Ok(()) => println!("✓ Configuration is valid"),
        Err(e) => println!("✗ Configuration error: {}", e),
    }
    println!();

    // 8. Caching
    println!("8. Caching System:");
    let mut cache: HashMap<String, (String, u64)> = HashMap::new();
    let cached = cache_data(&mut cache, "user_123".to_string(), "John Doe".to_string(), 300);
    println!("✓ Data cached: {}", cached);
    
    if let Some(data) = get_cached_data(&cache, "user_123") {
        println!("✓ Retrieved from cache: {}", data);
    }
    println!();

    // 9. Logging
    println!("9. Logging System:");
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("user_id".to_string(), "12345".to_string());
    metadata.insert("ip_address".to_string(), "192.168.1.1".to_string());
    
    log_event("INFO", "User login successful", Some(&metadata));
    log_event("ERROR", "Database connection failed", None);
    println!();

    // 10. Rate Limiting
    println!("10. Rate Limiting:");
    match check_rate_limit("user_123", 100, 3600) {
        Ok(allowed) => {
            if allowed {
                println!("✓ Request allowed");
            } else {
                println!("✗ Request blocked - rate limit exceeded");
            }
        },
        Err(e) => println!("✗ Rate limiting error: {}", e),
    }
}

// how to run code file 
// cargo run --bin SomeFunctionExample 

/*
Output: 

=== Industry-Relevant Rust Functions ===

1. User Registration:
Registering user: john_doe with email: john@example.com
✓ Success: User john_doe registered successfully with ID: user_12345

2. Database Connection Pool:
Created database connection pool with 5 connections
✓ Created 5 connections: ["db_connection_0", "db_connection_1", "db_connection_2", "db_connection_3", "db_connection_4"]

3. Loan Payment Calculation:
✓ Monthly payment: $567.79

4. Sales Data Processing:
Sales range: $890.25 - $2100.00
✓ Total: $6671.50, Average: $1334.30, Max: $2100.00, Count: 5

5. User Authentication:
✓ User authenticated successfully

6. Payment Processing:
✓ Payment of $250.75 processed successfully
✗ Network error: NetworkError(())

7. Configuration Validation:
✓ Configuration is valid

8. Caching System:
✓ Data cached: true
✓ Retrieved from cache: John Doe

9. Logging System:
[INFO] 1782740998 - User login successful | Metadata: {"ip_address": "192.168.1.1", "user_id": "12345"}
[ERROR] 1782740998 - Database connection failed

10. Rate Limiting:
✓ Request allowed

*/
