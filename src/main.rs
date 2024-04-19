use core::convert::Infallible;
use tokio::sync::{mpsc, Mutex};
use std::{collections::HashMap, sync::Arc};
use warp::{ws::Message, Filter, Rejection};

mod handler;
mod ws;
mod message;
mod strategy;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    println!("Started!");
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    // let echo = warp::path("echo")
    //     // The `ws()` filter will prepare the Websocket handshake.
    //     .and(warp::ws())
    //     .map(|ws: warp::ws::Ws| {
    //         // And then our closure will be called when it completes...
    //         ws.on_upgrade(|websocket| {
    //             // Just echo all messages back...
    //             let (tx, rx) = websocket.split();
    //             rx.forward(tx).map(|result| {
    //                 if let Err(e) = result {
    //                     eprintln!("websocket error: {:?}", e);
    //                 }
    //             })
    //         })
    //     });

    // let canvas_filler_route = warp::path("canvas-filler")
    //     .and(
    //       warp::body::bytes().and_then(|body: bytes::Bytes| async move {
    //           std::str::from_utf8(&body)
    //               .map(String::from)
    //               .map_err(|_e| warp::reject::custom(NotUtf8))
    //       })
    //     )
    //     .and(with_clients(clients.clone()))
    //     .and_then(handler::canvas_filler_handler);

    // let heartbeat_route = warp::path!("heartbeat").and_then(handler::heartbeat_handler);

    // let admin_route = warp::path("admin")
    //     .and(warp::ws())
    //     .and(warp::path::param())
    //     .and(with_clients(clients.clone()))
    //     .and_then(handler::admin_handler);

    // let register = warp::path("register");
    // let register_routes = register
    //     .and(warp::post())
    //     .and(warp::body::json())
    //     .and(with_clients(clients.clone()))
    //     .and_then(handler::register_handler)
    //     .or(register
    //         .and(warp::delete())
    //         .and(warp::path::param())
    //         .and(with_clients(clients.clone()))
    //         .and_then(handler::unregister_handler));

    // let routes = echo
    //     .or(canvas_filler_route)
    //     .or(heartbeat_route)
    //     .or(admin_route)
    //     .or(register_routes)
    //     .with(warp::cors().allow_any_origin());

    // warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// #[derive(Debug)]
// struct NotUtf8;
// impl warp::reject::Reject for NotUtf8 {}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
  warp::any().map(move || clients.clone())
}