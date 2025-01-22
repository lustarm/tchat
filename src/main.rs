mod client;
mod rand;
mod command;
mod logger;

/* == STD == */
use std::net::{TcpListener, TcpStream};
use std::thread;

/* == LOG == */
use log::info;

use crate::client::Client;
use crate::logger::logger_init;

fn listen() -> Result<TcpListener, Box<dyn std::error::Error>> {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:8000")?;
    info!("Listening on port 8000");
    return Ok(listener)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger_init();

    let listener = listen()?;

    for stream in listener.incoming() {
        let s: TcpStream = stream?;
        thread::spawn(|| {
            Client::new(s)
                .srw().expect("Failed SRW");
        });
    }

    Ok(())
}
