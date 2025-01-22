/* == RAND == */
use rand::{distributions::Alphanumeric, Rng}; // 0.8

/* == HELPER == */
pub fn gen_str() -> Result<String, std::io::Error> {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    return Ok(s);
}
