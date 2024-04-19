use crate::{ws, Clients, Result};

use warp::reply::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}

// pub async fn canvas_filler_handler(body: String, clients: Clients) -> Result<impl Reply> {
//     println!("Message received: {}", body);

//     // payload has the format: <command>|<JSON arguments>
//     let parts = body.split("|").collect::<Vec<&str>>();

//     match parts[0] {
//         "start" => command::start_command(),
//         "stop" => command::stop_command(),
//         "info" => command::info_command(),
//         "clear" => command::clear_command(),
//         _ => 0
//     };

//     Ok(StatusCode::OK)
// }

// pub async fn heartbeat_handler() -> Result<impl Reply> {
//     Ok(StatusCode::OK)
// }

// pub async fn admin_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
//     let client = clients.read().await.get(&id).cloned();
//     match client {
//         Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
//         None => Err(warp::reject::not_found()),
//     }
// }

// pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
//     let user_id = body.user_id;
//     let uuid = Uuid::new_v4().as_simple().to_string();

//     register_client(uuid.clone(), user_id, clients).await; // Pass the entry topic
//     Ok(json(&RegisterResponse {
//         url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
//     }))
// }

// async fn register_client(id: String, user_id: String, clients: Clients) {
//     clients.write().await.insert(
//         id,
//         Client {
//             user_id,
//             sender: None,
//         },
//     );
// }

// pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
//     clients.write().await.remove(&id);
//     Ok(StatusCode::OK)
// }