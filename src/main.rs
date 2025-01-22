mod client;
mod rand;
mod command;

/* == STD == */
use std::net::{TcpListener, TcpStream};
use std::thread;

/* == LOG == */
use log::info;

use crate::client::Client;
use crate::command::Commands;

/* == Helper == */
fn logger_init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger_init();
    // Init commands
    let cmds = Commands::new();

    let listener: TcpListener = TcpListener::bind("0.0.0.0:8000")?;
    info!("Listening on port 8000");

    for stream in listener.incoming() {
        let s: TcpStream = stream?;
        // Is move here redundent?
        thread::spawn(|| {
            let mut c: Client = Client::new(s);
            let cmd = c.srw().expect("Failed SRW");

        });
    }

    Ok(())
}
