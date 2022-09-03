use http::Uri;
use hyper::Client;
use std::sync::{Arc};
use tokio::sync::Semaphore;
use stopwatch::{Stopwatch};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    println!("start");
    let env_url: String = env::var("url").expect("$url is not set");    
    let env_concurrent: String = env::var("concurrent").expect("$concurrent is not set");    
    println!("send req to {:?}", env_url);

    let url: Uri = env_url.parse()?;

    let semaphore = Arc::new(Semaphore::new(env_concurrent.parse::<usize>().unwrap()));

    let sw = Stopwatch::start_new();
    let client = Client::new();

    for i in 1 .. 10000 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let tmp_client = client.clone();
        let tmp_url = url.clone();
        
        tokio::spawn(async move {
            let resp = tmp_client.get(tmp_url).await;

            match resp {
                Ok(_r) => { 
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
    Ok(())
}
