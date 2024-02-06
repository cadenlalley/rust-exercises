use error_chain::error_chain;
use std::io::Read;

// Use error chain crate to define foreign errors
// as errors that are defined. I.e. cast the
// std::io::Error and reqwest::Error into the
// error_chain defined errors.
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

// Main function that returns a Result
fn main() -> Result<()> {
    // Create a mutable variable called res that 
    // makes a get request to httpbin.org/get
    // and unwraps the result returned by the
    // reqwest::blocking::get function.
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;

    // Create a new string placeholder
    // for the body of the HTTP response.
    let mut body = String::new();

    // Read the response body into the body variable.
    // Also unwrap the result returned by the read_to_string.
    res.read_to_string(&mut body)?;

    // Print the HTTP response information.
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    // If the program has not already panicked,
    // return an Ok result.
    Ok(())
}
