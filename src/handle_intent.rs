use crate::file_encrypt::{encrypt_file, decrypt_file};
use crate::{ProgramError};

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use blake2::VarBlake2b;
use blake2::digest::{Update, VariableOutput};

pub fn handle_encrypt(path: &str, passphrase: Option<&str>) -> Result<(), ProgramError> {
    let out_path = String::from(path) + ".kugelfisch";

    let passphrase = get_passphrase(passphrase)
        .map_err(|e| ProgramError::OperationFailed(Box::from(e)))?;

    let key = derive_key(passphrase.as_str());

    println!("Encrypting {} to {}", path, out_path);
    process_file(path, out_path.as_str(),
                 |i, o|
                     encrypt_file(i, o, key.as_slice(), None))
}

pub fn handle_decrypt(path: &str, passphrase: Option<&str>) -> Result<(), ProgramError> {
    let out_path = String::from(path) + ".clear";

    let passphrase = get_passphrase(passphrase)
        .map_err(|e| ProgramError::OperationFailed(Box::from(e)))?;

    let key = derive_key(passphrase.as_str());

    println!("Decrypting {} to {}", path, out_path);
    process_file(path, out_path.as_str(),
                 |i, o|
                     decrypt_file(i, o, key.as_slice()))
}

fn process_file<F>(path: &str, out: &str,
                   op: F) -> Result<(), ProgramError>
    where F: Fn(&mut dyn Read, &mut dyn Write) -> Result<(), Box<dyn Error>> {
    let mut i_file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(ProgramError::ErrorOpening(String::from(path), Box::from(e)));
        }
    };

    let mut o_file = match File::create(out) {
        Ok(f) => f,
        Err(e) => {
            return Err(ProgramError::ErrorCreating(String::from(out), Box::from(e)));
        }
    };

    if let Err(e) = op(&mut i_file, &mut o_file) {
        return Err(ProgramError::OperationFailed(Box::from(e)));
    };

    Ok(())
}

fn get_passphrase(option: Option<&str>) -> Result<String, std::io::Error> {
    if let Some(pass) = option {
        println!("Using passphrase '{}'", pass);
        Ok(String::from(pass))
    } else {
        print!("Enter passphrase: ");
        std::io::stdout().flush()?;
        let mut pass = String::new();
        std::io::stdin().read_line(&mut pass)?;
        Ok(pass)
    }
}

fn derive_key(passphrase: &str) -> Vec<u32> {
    let mut hasher = VarBlake2b::new(56).unwrap();
    hasher.update(passphrase.as_bytes());
    let mut key = vec![0u32; 14];
    hasher.finalize_variable(|res| {
        for i in 0..14 {
            let mut x = [0u8; 4];
            x.copy_from_slice(&res[i * 4..i * 4 + 4]);
            key[i] = u32::from_le_bytes(x);
        }
    });
    return key;
}
