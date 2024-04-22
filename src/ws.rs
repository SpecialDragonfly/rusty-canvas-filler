use std::{borrow::Borrow, str::FromStr};

use warp::ws::{Message, WebSocket};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::message::{Command, Info, InfoResult, Start, Stop, StrategyType};

pub async fn client_connection(ws: WebSocket) {
    println!("establishing client connection... {:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message): {}", e);
                break;
            }
        };

        println!("received message: {:?}", msg);

        let message = match msg.to_str() {
            Ok(v) => v,
            Err(_) => return,
        };

        let mut used_strategy: Option<StrategyType> = None;
        let command = Command::from_str(message).unwrap();
        match command {
            Command::Start{strategy} => {
                let strategy_ref = used_strategy.insert(strategy);
                strategy_ref.start(client_sender.borrow());
            },
            Command::Stop => {
                if used_strategy.is_some() {
                    let strategy_ref = used_strategy.unwrap();
                    strategy_ref.stop();
                }
            },
            Command::Info => {
                if used_strategy.is_some() {
                    let strategy_ref = used_strategy.unwrap();
                    strategy_ref.info(client_sender.borrow());
                } else {
                    let _ = client_sender.send(
                        Ok(
                            Message::text(serde_json::to_string(
                                &InfoResult{colours_used: 0, points_remaining: 0}
                            ).unwrap())
                        )
                    );
                }
            }
        }
    }

    println!("disconnected");
}