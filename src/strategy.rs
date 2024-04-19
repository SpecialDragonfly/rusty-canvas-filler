use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use warp::filters::ws::Message;

pub struct Size {
    width: i32,
    height: i32
}

pub trait Strategy {
    fn run(&mut self, sender: &mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>);
    fn stop(&mut self);
    fn info(&self) -> InfoResult;
}

pub struct Blackhole {
    size: Size,
    running: bool
}


#[derive(Serialize, Deserialize)]
pub struct InfoResult {
    colours_used: i32,
    points_remaining: i32
}

impl Blackhole {
    pub fn new(width:i32, height:i32) -> Blackhole {
        Blackhole { size: Size { width, height }, running: false}
    }
}

impl Strategy for Blackhole {
    fn run(&mut self, sender: &mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>) {
        self.running = true;
        for column in 0..self.size.width {
            for row in 0..self.size.height {
                if self.running {
                    let _ = sender.send(Ok(Message::text(format!("filling in ({}, {})", column, row))));
                }
            }
        }
    }
    
    fn stop(&mut self) {
        self.running = false;
    }
    
    fn info(&self) -> InfoResult {
        return InfoResult{colours_used: 0, points_remaining: 0};
    }
}

pub struct Iterative {
    size: Size,
    running: bool
}
impl Iterative {
    pub fn new(width:i32, height:i32) -> Iterative {
        Iterative { size: Size { width, height }, running: false }
    }
}

impl Strategy for Iterative {

    fn run(&mut self, sender: &mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>) {
        self.running = true;
        for column in 0..self.size.width {
            for row in 0..self.size.height {
                if self.running {
                    let _ = sender.send(Ok(Message::text(format!("filling in ({}, {})", column, row))));
                }
            }
        }
    }
    
    fn stop(&mut self) {
        self.running = false;
    }
    
    fn info(&self) -> InfoResult {
        return InfoResult{colours_used: 0, points_remaining: 0};
    }
}

pub enum StrategyType {
    Blackhole(Blackhole),
    Iterative(Iterative)
}

impl Strategy for StrategyType {
    fn run(&mut self, sender: &mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>) {
        match self {
            StrategyType::Blackhole(f) => f.run(sender),
            StrategyType::Iterative(i) => i.run(sender)
        }
    }

    fn stop(&mut self) {
        match self {
            StrategyType::Blackhole(f) => f.stop(),
            StrategyType::Iterative(i) => i.stop()
        }
    }

    fn info(&self) -> InfoResult {
        match self {
            StrategyType::Blackhole(f) => f.info(),
            StrategyType::Iterative(i) => i.info()
        }
    }
}