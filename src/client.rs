use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{Read, Write};

use log::{info, debug, error};

use crate::rand::gen_str;
use crate::command::Commands;

/* == DEFINES == */
const INLINE: &str = "-> ";

/* == CLIENT == */
#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    sid: String,

    // Buffer for commands
    buf: String,
    cmds: HashMap<String, String>,
}

impl Client {
    // Constructor
    pub fn new(mut s: TcpStream) -> Self {
        info!(
            "Client connected: {}",
            s.peer_addr().expect("Failed to parse client peer address")
        );

        let s_: String = gen_str().expect("Failed to generate session id string");
        debug!("Session ID generated: {}", s_);

        s.write(b"\xFF\xFB\x01\xFF\xFB\x03\xFF\xFC\x22")
            .expect("Failed to write telnet codes");

        Client { stream: s, sid: s_, buf: String::new(), cmds: Commands::new() }
    }

    pub fn cmd(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = self.buf.to_ascii_lowercase();

        if let Some(c) = self.cmds.get(&cmd) {
            self.stream.write(c.as_bytes())?;
            self.stream.write(INLINE.as_bytes())?;
            debug!("value: {}", c);
        } else {
            self.stream.write(b"Invalid command\r\n")?;
            self.stream.write(INLINE.as_bytes())?;
        }

        Ok(())
    }

    /* == socket read write == */
    pub fn srw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Write first prompt
        self.stream.write(INLINE.as_bytes())?;

        let mut buf: [u8; 1024] = [0; 1024];
        let mut pos = 0;

        loop {
            self.stream.read(&mut buf[pos..pos + 1])?;

            match buf[pos] {
                0xFF => {
                    self.stream.read(&mut buf[pos..pos + 2])?;
                    if pos > 0 { pos -= 1; }
                    continue;
                },

                b'\x7F' | b'\x08' => {
                    if pos > 0 {
                        self.stream.write(&[buf[pos]])?;
                        pos -= 1;
                    }
                    continue;
                },

                b'\r' | b'\t' => {
                    if pos > 0 {
                        pos -= 1;
                    }
                    continue;
                },

                b'\n' | b'\x00' => {
                    self.stream.write(b"\r\n")?;
                    pos = 0;
                },

                _ => {
                    if let Some(character) = std::char::from_u32(buf[pos] as u32) {
                        debug!("buf[pos]: {}", character);
                        debug!("pos: {}", pos);
                    } else {
                        error!("Invalid UTF-8 character for value {}", buf[pos]);
                    }

                    if !buf[pos].is_ascii_alphanumeric() {
                        continue;
                    }
                    let b = buf[pos];
                    self.stream.write(&[b])?;
                    pos += 1;
                    continue;
                }
            }

            self.buf = String::from_utf8(buf.to_vec())?;
            self.cmd()?;
        }

    }
}
