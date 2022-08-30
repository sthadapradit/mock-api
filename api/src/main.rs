// use serde_derive::{Deserialize, Serialize};

use warp::Filter;

// #[derive(Deserialize, Serialize)]
// struct Employee {
//     name: String,
//     rate: u32,
// }

#[tokio::main]
async fn main() {
    

    let hi = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        });
        

    let bye = warp::path("bye")
        .map(|| {
            "bye"
        });

    let routes = warp::get().and(
            hi
            .or(bye)
        );
            
    // POST /employees/:rate  {"name":"Sean","rate":2}
    // let promote = warp::post()
    //     .and(warp::path("employees"))
    //     .and(warp::path::param::<u32>())
    //     // Only accept bodies smaller than 16kb...
    //     .and(warp::body::json())
    //     .map(|rate, mut employee: Employee| {
    //         employee.rate = rate;
    //         warp::reply::json(&employee)
    //     });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}