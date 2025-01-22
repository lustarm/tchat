/* == STD == */
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

/* == LOG == */
use log::{info, debug};

/* == RAND == */
use rand::{distributions::Alphanumeric, Rng}; // 0.8

/* == DEFINES == */
const INLINE: &str = "-> ";

/* == HELPER == */
fn gen_str() -> Result<String, std::io::Error> {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    return Ok(s);
}

/* == CLIENT == */
#[derive(Debug)]
struct Client {
    stream: TcpStream,
    sid: String,
}

impl Client {
    // Constructor
    fn new(s: TcpStream) -> Self{
        info!("Client connected: {}", s.peer_addr()
            .expect("Failed to parse client peer address"));

        let s_: String = gen_str().expect("Failed to generate session id string");
        debug!("Session ID generated: {}", s_);

        Client {
            stream: s,
            sid: s_,
        }
    }

    /* == socket read write == */

    // Mut because im writing to TcpStream
    fn srw(&mut self) -> Result<(), Box<dyn std::error::Error>>{

        let mut r: bool = true;

        while r
        {
            self.stream.write(b"hello world\n")?;
            r = false;
        }

        return Ok(());
    }
}

/* == Helper == */
fn logger_init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger_init();

    let listener: TcpListener = TcpListener::bind("0.0.0.0:8000")?;
    info!("Listening on port 8000");

    for stream in listener.incoming() {
        let s: TcpStream = stream?;
        // Is move here redundent?
        thread::spawn(move || {
            let mut c: Client = Client::new(s);
            c.srw().expect("Failed SRW");
        });
    }

    Ok(())
}
