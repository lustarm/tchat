use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use log::{debug, info, error};

use crate::command::Commands;
use crate::rand::gen_str;

/* == DEFINES == */
const INLINE: &str = "-> ";

/* == CLIENT == */
pub struct Client {
    stream: TcpStream,
    sid: String,

    // Buffer for commands
    buf: String,
    cmds: Commands,
}

fn bruh(c: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    c.stream.write(c.sid.as_bytes())?;
    c.stream.write(b"\r\n")?;
    Ok(())
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

        Client {
            stream: s,
            sid: s_,
            buf: String::new(),
            cmds: Commands::new(),
        }
    }

    pub fn cmd(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Insert the command at call time
        self.cmds.insert(String::from("test"), Arc::new(bruh));

        // Temporarily store the command function
        let command_fn = self.cmds.get(&self.buf).unwrap_or_else(|| {
            error!("Invalid function called");
            self.stream.write(b"Invalid command\r\n");
            &(Arc::new(|_| Ok(())) as Arc<dyn Fn(&mut Client) -> Result<(), Box<dyn std::error::Error>>>)
        }).clone();

        // Execute the command
        command_fn(self)?;

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
                    if pos > 0 {
                        pos -= 1;
                    }
                    continue;
                }

                b'\x7F' | b'\x08' => {
                    if pos > 0 {
                        self.stream.write(&[buf[pos]])?;
                        pos -= 1;
                    }
                    continue;
                }

                b'\r' | b'\t' => {
                    if pos > 0 {
                        pos -= 1;
                    }
                    continue;
                }

                b'\n' | b'\x00' => {
                    self.stream.write(b"\r\n")?;
                    pos = 0;
                }

                _ => {
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
            debug!("{}", self.buf.trim());
            self.cmd()?;
        }
    }
}

