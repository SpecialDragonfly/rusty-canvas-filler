use warp::{reply::Reply, Filter, Rejection};

mod ws;
mod message;

type Result<T> = std::result::Result<T, Rejection>;

pub async fn ws_handler(ws: warp::ws::Ws) -> Result<impl Reply> {
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket)))
}

#[tokio::main]
async fn main() {
    println!("Started!");

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and_then(ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
