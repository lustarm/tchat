/* == STD == */
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

/* == LOG == */
use log::{info, debug, error};

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
    fn new(mut s: TcpStream) -> Self{
        info!("Client connected: {}", s.peer_addr()
            .expect("Failed to parse client peer address"));

        let s_: String = gen_str().expect("Failed to generate session id string");
        debug!("Session ID generated: {}", s_);

        // Send telnet stuff
        s.write(b"\xFF\xFB\x01\xFF\xFB\x03\xFF\xFC\x22")
            .expect("Failed to write telnet codes");

        Client {
            stream: s,
            sid: s_,
        }
    }



    /* == socket read write == */

    // Mut because im writing to TcpStream
    fn srw(&mut self) -> Result<(), Box<dyn std::error::Error>>{

        let mut r: bool = true;
        let mut buf = [0; 128];

        // Dead read telnet stuff
        self.stream.read(&mut buf)?;

        // Write first prompt
        self.stream.write(INLINE.as_bytes())?;

        while r
        {
            // Clear buffer
            buf.fill(0);

            self.stream.read(&mut buf)?;

            match std::str::from_utf8(&buf) {
                Ok(cmd) => {
                    debug!("{:?}", cmd.trim_end_matches("\0"));
                    if cmd.trim_end_matches("\0") == "exit" {
                        debug!("Exit called by user: {} - {}", self.sid,
                            self.stream
                            .peer_addr()
                            .expect("Failed to parse peer address"));
                        self.stream.shutdown(Shutdown::Both)?;
                        // Shut down server also
                        r = false;
                    }
                }, // Print as UTF-8 string
                Err(e) => error!("Failed to convert buffer to UTF-8: {}", e), // Handle conversion error
            }
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
        thread::spawn(|| {
            let mut c: Client = Client::new(s);
            c.srw().expect("Failed SRW");
        });
    }

    Ok(())
}
