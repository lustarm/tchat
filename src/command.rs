use std::collections::HashMap;

use crate::client::Client;

pub type Commands = HashMap<String, Box<dyn Fn(Client) -> Result<(), Box<dyn std::error::Error>>>>;

