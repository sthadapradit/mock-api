use warp::Filter;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::BufReader;

mod city_search;
mod properties_search;

#[derive(Clone)]
#[derive(Deserialize, Serialize)]
struct Hotel {
    id: u32,
    name: String,
    city: String
}

impl Hotel {
    fn new(id: u32, name: String, city: String) -> Hotel {
        let hotel = Hotel {
            id, name, city
        };

        hotel
    }
}

#[tokio::main]
async fn main() {
    
    let file = File::open("properties.json").unwrap();
    let reader = BufReader::new(file);
    let result: properties_search::Root = serde_json::from_reader(reader).unwrap();

    let properties = warp::path("properties")
        .map(move|| {
            warp::reply::json(&result)
        });


    let hi = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        });

    
    let hotel = warp::path("hotels")
        .map(|| {
            let h = Hotel::new(1, "David Hotel".to_string(), "Bangkok".to_string());
            warp::reply::json(&h)
        });

    let routes = warp::get()
        .and(hi)
        .or(hotel)
        .or(properties);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
