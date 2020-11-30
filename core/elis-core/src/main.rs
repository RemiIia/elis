#[macro_use]
extern crate log;

use async_trait::async_trait;
use xactor::{Result as XResult, *};

struct Server {
    connections: Vec<Connection>,
}

struct Connection(usize);

impl Server {
    fn new() -> Self {
        let connections: Vec<Connection> = Vec::new();
        Server { connections }
    }
}

#[async_trait]
impl Actor for Server {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("Server was just created");
        Ok(())
    }
}

#[xactor::main]
async fn main() -> XResult<()> {
    // Logger settings
    flexi_logger::Logger::with_env_or_str("info")
        .format(flexi_logger::colored_detailed_format)
        .start()?;

    let addr = Server::new().start().await?;

    Ok(())
}

// use actix::dev::{MessageResponse, ResponseChannel};
// use actix::prelude::*;

// #[derive(Message)]
// #[rtype(result = "Responses")]
// enum Messages {
//     Ping,
//     Pong,
// }

// enum Responses {
//     GotPing,
//     GotPong,
// }

// impl<A, M> MessageResponse<A, M> for Responses
// where
//     A: Actor,
//     M: Message<Result = Responses>,
// {
//     fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
//         if let Some(tx) = tx {
//             tx.send(self);
//         }
//     }
// }

// // Define actor
// struct MyActor;

// // Provide Actor implementation for our actor
// impl Actor for MyActor {
//     type Context = Context<Self>;

//     fn started(&mut self, _ctx: &mut Context<Self>) {
//         println!("Actor is alive");
//     }

//     fn stopped(&mut self, _ctx: &mut Context<Self>) {
//         println!("Actor is stopped");
//     }
// }

// /// Define handler for `Messages` enum
// impl Handler<Messages> for MyActor {
//     type Result = Responses;

//     fn handle(&mut self, msg: Messages, _ctx: &mut Context<Self>) -> Self::Result {
//         match msg {
//             Messages::Ping => Responses::GotPing,
//             Messages::Pong => Responses::GotPong,
//         }
//     }
// }

// #[actix_rt::main]
// async fn main() {
//     // Start MyActor in current thread
//     let addr = MyActor.start();

//     // Send Ping message.
//     // send() message returns Future object, that resolves to message result
//     let ping_future = addr.send(Messages::Ping).await;
//     let pong_future = addr.send(Messages::Pong).await;

//     match pong_future {
//         Ok(res) => match res {
//             Responses::GotPing => println!("Ping received"),
//             Responses::GotPong => println!("Pong received"),
//         },
//         Err(e) => println!("Actor is probably dead: {}", e),
//     }

//     match ping_future {
//         Ok(res) => match res {
//             Responses::GotPing => println!("Ping received"),
//             Responses::GotPong => println!("Pong received"),
//         },
//         Err(e) => println!("Actor is probably dead: {}", e),
//     }
// }
