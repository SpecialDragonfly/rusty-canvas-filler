use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use futures::{FutureExt, StreamExt};
use crate::strategy::{Blackhole, Iterative, Strategy, StrategyType};
use crate::{message, Client, Clients};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use message::StartCommand;
use message::StopCommand;
use message::InfoCommand;

pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("establishing client connection... {:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    let uuid = Uuid::new_v4().simple().to_string();

    let new_client = Client {
        user_id: uuid.clone(),
        sender: Some(client_sender),
    };

    clients.lock().await.insert(uuid.clone(), new_client);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        client_msg(&uuid, msg, &clients).await;
    }

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(client_id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    let parts = message.split("|").collect::<Vec<&str>>();

    let start_command = StartCommand::new().command;
    let stop_command = StopCommand::new().command;
    let info_command = InfoCommand::new().command;

    let locked = clients.lock().await;

    let mut strategy: Option<StrategyType> = None;

    match locked.get(client_id) {
        Some(v) => {
            if let Some(sender) = &v.sender {
                match parts[0].to_string() {
                    start_command => {
                        let strategy_name = parts[1];
                        let width = parts[2].parse().unwrap();
                        let height = parts[3].parse().unwrap();
                        match strategy_name {
                            "iterative" => {
                                let mut x = StrategyType::Iterative(Iterative::new(width, height));
                                x.run(sender);
                                strategy = Some(x);
                            },
                            "blackhole" => {
                                let mut x = StrategyType::Blackhole(Blackhole::new(width, height));
                                x.run(sender);
                                strategy = Some(x);
                            },
                            _ => {
                                return
                            }
                        }
                    },
                    stop_command => {
                        match strategy {
                            Some(mut strategy) => strategy.stop(),
                            _ => return
                        }
                    },
                    info_command => {
                        match strategy {
                            Some(strategy) => {
                                let x = strategy.info();
                                let _ = sender.send(
                                    Ok(
                                        Message::text(serde_json::to_string(&x).unwrap())
                                    )
                                );
                            },
                            _ => return
                        }
                    }
                }
            }
        },
        None => return,
    }
}