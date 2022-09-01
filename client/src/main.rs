use hyper::Client;
use http::uri::Uri;
use hyper::body::HttpBody;
use std::io;
use std::sync::{Arc};
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    println!("start");
    let semaphore = Arc::new(Semaphore::new(100));

    for i in 1 .. 1000000 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        tokio::spawn(async move {
            let client = Client::new();
            let resp = client.get(Uri::from_static("http://127.0.0.1:3030/properties")).await;
            match resp {
                Ok(r) => {
                    match hyper::body::to_bytes(r.into_body()).await {
                        Ok(x)=>{
                            str::from_utf8(x);
                            println!("{:?}", x.to_ascii_lowercase());
                        },
                        Err(_) => todo!(),
                    }
                    drop(permit);
                },
                Err(err) => {
                    println!("error --> {err}");
                    drop(permit);
                }
            }
        });
    }

    let _ = io::stdin().lines();
    println!("end");
    Ok(())
}
