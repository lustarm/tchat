use std::collections::HashMap;

pub type Commands = HashMap<String, String>;

pub struct Command {
    name: String,
    description: String,
}
