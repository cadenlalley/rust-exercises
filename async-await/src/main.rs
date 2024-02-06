use error_chain::error_chain;

// Use error chain for the same reasons 
// as the previous Request exercise
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


// Use tokio to make the main function async
#[tokio::main]
async fn main() -> Result<()> {
    // Await the result of the get request
    let res = reqwest::get("http://httpbin.org/get").await?;

    // Print the status & headers of the response
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    // Await the body of the response
    // then print it.
    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}
