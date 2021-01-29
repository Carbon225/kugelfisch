use crate::file_encrypt::{encrypt_file, decrypt_file};
use crate::{ProgramError, ProgramConfig};
use crate::passphrase::derive_key;

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

pub fn handle_encrypt(config: &ProgramConfig) -> Result<(), ProgramError> {
    let passphrase = get_passphrase(config.passphrase)
        .map_err(|e| ProgramError::OperationFailed(Box::from(e)))?;

    let key = derive_key(passphrase.as_str());

    process_file(config.input_path, config.output_path,
                 |i, o|
                     encrypt_file(i, o, key.as_slice(), None))
}

pub fn handle_decrypt(config: &ProgramConfig) -> Result<(), ProgramError> {
    let passphrase = get_passphrase(config.passphrase)
        .map_err(|e| ProgramError::OperationFailed(Box::from(e)))?;

    let key = derive_key(passphrase.as_str());

    process_file(config.input_path, config.output_path,
                 |i, o|
                     decrypt_file(i, o, key.as_slice()))
}

fn process_file<F>(path: &str, out: &str,
                   op: F) -> Result<(), ProgramError>
    where F: Fn(&mut dyn Read, &mut dyn Write) -> Result<(), Box<dyn Error>> {
    let mut i_file = File::open(path)
        .map_err(|e| ProgramError::ErrorOpening(String::from(path), Box::from(e)))?;

    let mut o_file = File::create(out)
        .map_err(|e| ProgramError::ErrorCreating(String::from(out), Box::from(e)))?;

    op(&mut i_file, &mut o_file).map_err(|e| ProgramError::OperationFailed(e))
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
