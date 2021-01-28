#[cfg(test)]
mod tests;

use std::convert::TryInto;
use blake2::VarBlake2b;
use blake2::digest::{Update, VariableOutput};

pub fn derive_key(passphrase: &str) -> Vec<u32> {
    let mut hasher = VarBlake2b::new(56).unwrap();
    hasher.update(passphrase.as_bytes());
    let mut key = Vec::with_capacity(14);

    hasher.finalize_variable(|res| {
        for word in res.chunks_exact(4) {
            key.push(u32::from_le_bytes(word.try_into().unwrap()));
        }
    });

    return key;
}