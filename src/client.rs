use std::net::TcpStream;
use std::io::{Read, Write};

use log::{info, debug, error};

use crate::rand::gen_str;

/* == DEFINES == */
const INLINE: &str = "-> ";

/* == CLIENT == */
#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    sid: String,
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

        Client { stream: s, sid: s_ }
    }

    /* == socket read write == */

    // Mut because im writing to TcpStream
    pub fn srw(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let r: bool = true;

        // Write first prompt
        self.stream.write(INLINE.as_bytes())?;

        let mut buf: [u8; 1024] = [0; 1024];
        let mut pos = 0;

        while r {
            self.stream.read(&mut buf[pos..pos + 1])?;

            match buf[pos] {
                0xFF => {
                    self.stream.read(&mut buf[pos..pos + 2])?;
                    if pos > 0 { pos -= 1; }
                },

                b'\x7F' | b'\x08' => {
                    if pos > 0 {
                        self.stream.write(&[buf[pos]])?;
                        pos -= 1;
                    }
                },

                b'\r' | b'\t' => {
                    if pos > 0 {
                        pos -= 1;
                    }
                },

                b'\n' | b'\x00' => {
                    self.stream.write(b"\r\n")?;
                    self.stream.write(INLINE.as_bytes())?;
                    pos = 0;
                },

                _ => {
                    if let Some(character) = std::char::from_u32(buf[pos] as u32) {
                        debug!("buf[pos]: {}", character);
                    } else {
                        error!("Invalid UTF-8 character for value {}", buf[pos]);
                    }

                    if buf[pos] == b'\x1B' {
                        buf[pos] = b'^';
                        let mut b = buf[pos];
                        self.stream.write(&[b])?;
                        pos += 1;
                        buf[pos] = b'[';
                        b = buf[pos];
                        self.stream.write(&[b])?;
                    } else {
                        let b = buf[pos];
                        self.stream.write(&[b])?;
                        pos += 1;
                    }
                }
            }
            debug!("pos: {}", pos);
        }

        return Ok(String::from_utf8(buf.to_vec())?);
    }
}
