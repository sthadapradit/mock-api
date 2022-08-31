use hyper::Client;
use hyper::Error;
use hyper::client::ResponseFuture;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("start");

    let client = Client::new();

    // for i in 1 .. 100 {
    //     let uri = "http://localhost:3030/properties".parse()?;
    //     let mut resp = client.get(uri).await?;

    //     println!("{i} Response: {}", resp.status());
    // }

    let handler = thread::spawn(async || {

        println!("shooting");
        let mut resp = client.get(uri);

        let mut resp = client.get(uri).await?;

        println!("hello")

        1
    });

    println!("done");

    Ok(())
}์