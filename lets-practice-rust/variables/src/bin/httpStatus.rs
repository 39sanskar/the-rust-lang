// HTTP Status Codes with names and descriptions

fn main() {
    // Using tuples to store (code, name, description)
    let http_status_codes = [
        // 1xx Informational
        (100, "Continue", "The server has received the request headers and the client should proceed to send the request body"),
        (101, "Switching Protocols", "The requester has asked the server to switch protocols and the server has agreed to do so"),
        (102, "Processing", "The server has received and is processing the request, but no response is available yet"),
        
        // 2xx Success
        (200, "OK", "The request has succeeded"),
        (201, "Created", "The request has been fulfilled, resulting in the creation of a new resource"),
        (202, "Accepted", "The request has been accepted for processing, but the processing has not been completed"),
        (204, "No Content", "The server successfully processed the request and is not returning any content"),
        (206, "Partial Content", "The server is delivering only part of the resource due to a range header sent by the client"),
        
        // 3xx Redirection
        (300, "Multiple Choices", "There are multiple options for the resource from which the client may choose"),
        (301, "Moved Permanently", "The resource has permanently moved to a different URI"),
        (302, "Found", "The resource resides temporarily under a different URI"),
        (304, "Not Modified", "The resource has not been modified since the version specified by the request headers"),
        (307, "Temporary Redirect", "The request should be repeated with another URI, but future requests can still use the original URI"),
        (308, "Permanent Redirect", "The request and all future requests should be repeated using another URI"),
        
        // 4xx Client Error
        (400, "Bad Request", "The server cannot or will not process the request due to an apparent client error"),
        (401, "Unauthorized", "Authentication is required and has failed or has not yet been provided"),
        (403, "Forbidden", "The request contained valid data and was understood by the server, but the server is refusing action"),
        (404, "Not Found", "The requested resource could not be found but may be available in the future"),
        (405, "Method Not Allowed", "A request method is not supported for the requested resource"),
        (409, "Conflict", "The request could not be processed because of conflict in the request"),
        (410, "Gone", "The resource requested is no longer available and will not be available again"),
        (413, "Payload Too Large", "The request is larger than the server is willing or able to process"),
        (422, "Unprocessable Entity", "The request was well-formed but was unable to be followed due to semantic errors"),
        (429, "Too Many Requests", "The user has sent too many requests in a given amount of time"),
        
        // 5xx Server Error
        (500, "Internal Server Error", "A generic error message, given when an unexpected condition was encountered"),
        (501, "Not Implemented", "The server either does not recognize the request method or lacks the ability to fulfill the request"),
        (502, "Bad Gateway", "The server was acting as a gateway or proxy and received an invalid response from the upstream server"),
        (503, "Service Unavailable", "The server cannot handle the request due to a temporary overload or maintenance"),
        (504, "Gateway Timeout", "The server was acting as a gateway or proxy and did not receive a timely response from the upstream server"),
        (505, "HTTP Version Not Supported", "The server does not support the HTTP protocol version used in the request"),
    ];
    
    // Print all status codes
    println!("HTTP Status Codes:");
    println!("==================");
    for (code, name, description) in &http_status_codes {
        println!("{} - {}: {}", code, name, description);
    }
    
    // Access specific status code
    let not_found = http_status_codes[13]; // 404 index
    println!("\nSpecific example:");
    println!("Status Code: {}", not_found.0);
    println!("Name: {}", not_found.1);
    println!("Description: {}", not_found.2);
    
    // Example error codes with context
    let error_codes = [200, 404, 500];
    println!("\nCommon error codes in applications:");
    for &code in &error_codes {
        // Find the corresponding status code info
        if let Some(&(c, name, desc)) = http_status_codes.iter().find(|&&(c, _, _)| c == code) {
            println!("{} - {} - {}", c, name, desc);
        }
    }
}

// How to run code file 
// cargo run --bin httpStatus   
