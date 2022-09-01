use hyper::Client;
use http::uri::Uri;
use std::sync::{Arc};
use tokio::sync::Semaphore;
use stopwatch::{Stopwatch};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    println!("start");
    let semaphore = Arc::new(Semaphore::new(50));
    let sw = Stopwatch::start_new();

    for i in 1 .. 100000000 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        tokio::spawn(async move {
            let client = Client::new();
            let resp = client.get(Uri::from_static("http://127.0.0.1:3030/properties")).await;
            match resp {
                Ok(r) => {
                    match hyper::body::to_bytes(r.into_body()).await {
                        Ok(x)=>{
                            println!("{i}");
                            // let y = x.to_ascii_lowercase();
                            // print!("{:?}", y[0]);
                        },
                        Err(err) => {
                            println!("err: {err}");
                        }
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

    println!("Time Taken {}ms", sw.elapsed_ms());
    let _ = io::stdin().lines();
    Ok(())
}
