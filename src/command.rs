use std::collections::HashMap;
use std::sync::Arc;

use crate::client::Client;

pub type Commands = HashMap<String, Arc<dyn Fn(&mut Client)
-> Result<(), Box<dyn std::error::Error>>>>;

