use std::net::TcpListener;

// Var lock for thread
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(":8000")?;

    for stream in listener.accept() {

    }

    Ok(())
}
