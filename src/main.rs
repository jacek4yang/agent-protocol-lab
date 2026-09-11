mod transport;

use serde::Serialize;
use transport::TransportClient;

#[derive(Debug, Serialize)]
struct ExampleResult<'a> {
    message: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TransportClient::new()?;

    println!("=== GET ===");

    let response = client.get("https://httpbin.org/get").await?;

    println!("status: {}", response.status);
    println!("headers: {:#?}", response.headers);
    println!("body:\n{}", String::from_utf8_lossy(&response.body));

    println!();
    println!("=== POST JSON ===");

    let request = ExampleResult {
        message: "hello from agent-protocol-lab",
    };

    let response = client
        .post_json("https://httpbin.org/post", &request)
        .await?;

    println!("status: {}", response.status);
    println!("headers: {:#?}", response.headers);
    println!("body:\n{}", String::from_utf8_lossy(&response.body));

    Ok(())
}
