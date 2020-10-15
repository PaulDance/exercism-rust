use rand::prelude::*;

const ALPHA_START: u8 = 97;
const ALPHA_LENGTH: u8 = 26;
const KEY_MIN_LEN: usize = 100;

fn invalid_key(key: &str) -> bool {
    key.is_empty() || key.chars().any(|chr| !chr.is_ascii_lowercase())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if invalid_key(key) {
        None
    } else {
        Some(
            s.bytes()
                .zip(key.bytes().cycle())
                .map(|(sc, kc)| sc + kc - 2 * ALPHA_START)
                .map(|pos| (ALPHA_START + pos % ALPHA_LENGTH) as char)
                .collect(),
        )
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if invalid_key(key) {
        None
    } else {
        Some(
            s.bytes()
                .zip(key.bytes().cycle())
                .map(|(sc, kc)| if sc < kc { ALPHA_LENGTH } else { 0 } + sc - kc)
                .map(|pos| (ALPHA_START + pos % ALPHA_LENGTH) as char)
                .collect(),
        )
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key = (0..s.len().max(KEY_MIN_LEN))
        .map(|_| ('a'..='z').choose(&mut rng).unwrap())
        .collect::<String>();
    let ciphertext = encode(&key, s).unwrap();
    (key, ciphertext)
}
