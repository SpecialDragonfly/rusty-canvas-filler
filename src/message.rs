use std::str::FromStr;

use serde::Serialize;
use tokio::sync::mpsc;
use warp::filters::ws::Message;

pub enum Command {
    Start{strategy: StrategyType},
    Stop,
    Info,
}

pub enum StrategyType {
    Blackhole{size: Size, remaining: u32},
    Iterative{size: Size, remaining: u32}
}

pub struct Size {
    pub width: u32,
    pub height: u32
}

impl FromStr for StrategyType {
    type Err = ParseCommandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("|").collect::<Vec<&str>>();
        match parts[..] {
            ["iterative", width_str, height_str] => Ok(StrategyType::Iterative{
                size: Size{
                    width: width_str.parse().map_err(|_| ParseCommandError)?, 
                    height: height_str.parse().map_err(|_| ParseCommandError)?
                },
                remaining: width_str.parse::<u32>().unwrap() * height_str.parse::<u32>().unwrap(),
            }),
            ["blackhole", width_str, height_str] => Ok(StrategyType::Blackhole{
                size: Size{
                    width: width_str.parse().map_err(|_| ParseCommandError)?, 
                    height: height_str.parse().map_err(|_| ParseCommandError)?
                },
                remaining: width_str.parse::<u32>().unwrap() * height_str.parse::<u32>().unwrap(),
            }),
            _ => Err(ParseCommandError)
        }
    }
}

pub trait Start{ fn start(&self, sender: &mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>); }
impl Start for StrategyType {
    fn start(&self, sender: &mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>) {
        match self {
            Self::Iterative { size , remaining: _} => {
                for column in 0..size.width {
                    for row in 0..size.height {
                        let _ = sender.send(
                            Ok(
                                Message::text([column.to_string(), row.to_string(), "#00ff00".to_string()].join("|"))
                            )
                        );
                    }
                }
            },
            Self::Blackhole { size , remaining: _} => {
                for column in 0..size.width {
                    for row in 0..size.height {
                        let _ = sender.send(
                            Ok(
                                Message::text([column.to_string(), row.to_string(), "#00ff00".to_string()].join("|"))
                            )
                        );
                    }
                }
            }
        }
    }
}
pub trait Stop{ fn stop(&self); }
impl Stop for StrategyType {
    fn stop(&self) {
        
    }
}

pub trait Info{ fn info(&self, sender: &mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>); }
impl Info for StrategyType {
    fn info(&self, sender: &mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>) {
        let mut _remaining:u32 = 0;
        match self {
            Self::Iterative { size: _, remaining } => _remaining = remaining.to_owned(),
            Self::Blackhole { size: _, remaining } => _remaining = remaining.to_owned()
        }
        let _ = sender.send(
            Ok(
                Message::text(serde_json::to_string(
                    &InfoResult{colours_used: 0, points_remaining: _remaining}
                ).unwrap())
            )
        );
    }
}

#[derive(Serialize)]
pub struct InfoResult {
    pub colours_used: u32,
    pub points_remaining: u32
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCommandError;

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If the | doesn't exist in the string, add it in so that it doesn't then panic.
        let mut actual_string = s.to_string();
        if s.find("|").is_none() {
            actual_string = format!("{}|", s);
        }
        if let Some((command, strategy_str)) = actual_string.split_once("|") {
            match command {
                "start" => Ok(Command::Start { strategy: strategy_str.parse()? }),
                "stop" => Ok(Command::Stop),
                "info" => Ok(Command::Info),
                _ => Err(ParseCommandError)
            }
        } else {
            Err(ParseCommandError)
        }
    }
}