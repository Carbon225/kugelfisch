mod blowfish;
mod cbc;
mod file_encrypt;

use file_encrypt::{encrypt_file, decrypt_file};
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;
use std::fmt::{Display, Debug, Formatter};

pub struct ProgramConfig<'a> {
    input_path: &'a str,
    passphrase: Option<&'a str>,
}

impl<'a> ProgramConfig<'a> {
    pub fn new(input_path: &'a str, passphrase: Option<&'a str>) -> Self {
        ProgramConfig { input_path, passphrase }
    }
}

pub enum ProgramIntent<'a> {
    Encrypt(ProgramConfig<'a>),
    Decrypt(ProgramConfig<'a>),
}

#[derive(Debug)]
pub enum ProgramError {
    InvalidMode(String),
    ErrorOpening(String, Box<dyn Error>),
    ErrorCreating(String, Box<dyn Error>),
    OperationFailed(Box<dyn Error>),
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            ProgramError::InvalidMode(mode) => format!("Invalid mode '{}'", mode),
            ProgramError::ErrorOpening(file, cause) => format!("Error opening '{}': {}", file, cause),
            ProgramError::ErrorCreating(file, cause) => format!("Error creating '{}': {}", file, cause),
            ProgramError::OperationFailed(cause) => format!("Operation failed: {}", cause),
        })
    }
}

impl Error for ProgramError {}

pub fn run_program(intent: &ProgramIntent) -> Result<(), ProgramError> {
    match intent {
        ProgramIntent::Encrypt(config) => handle_encrypt(config.input_path, config.passphrase),
        ProgramIntent::Decrypt(config) => handle_decrypt(config.input_path, config.passphrase),
    }
}

fn get_passphrase(option: Option<&str>) -> Result<String, std::io::Error> {
    if let Some(pass) = option {
        println!("Using passphrase {}", pass);
        Ok(String::from(pass))
    } else {
        print!("Enter passphrase: ");
        let mut pass = String::new();
        std::io::stdin().read_line(&mut pass)?;
        Ok(pass)
    }
}

fn handle_encrypt(path: &str, passphrase: Option<&str>) -> Result<(), ProgramError> {
    let out_path = String::from(path) + ".kugelfisch";

    let passphrase = get_passphrase(passphrase)
        .map_err(|e| ProgramError::OperationFailed(Box::from(e)))?;

    println!("Encrypting {} to {}", path, out_path);
    process_file(path, out_path.as_str(), encrypt_file)
}

fn handle_decrypt(path: &str, passphrase: Option<&str>) -> Result<(), ProgramError> {
    let out_path = String::from(path) + ".clear";

    let passphrase = get_passphrase(passphrase)
        .map_err(|e| ProgramError::OperationFailed(Box::from(e)))?;

    println!("Decrypting {} to {}", path, out_path);
    process_file(path, out_path.as_str(), decrypt_file)
}

fn process_file(path: &str, out: &str,
                op: fn(&mut dyn Read,&mut dyn Write) -> Result<(), Box<dyn Error>>) -> Result<(), ProgramError> {
    let mut i_file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(ProgramError::ErrorOpening(String::from(path), Box::from(e)));
        },
    };

    let mut o_file = match File::create(out) {
        Ok(f) => f,
        Err(e) => {
            return Err(ProgramError::ErrorCreating(String::from(out), Box::from(e)));
        },
    };

    if let Err(e) = op(&mut i_file, &mut o_file) {
        return Err(ProgramError::OperationFailed(Box::from(e)));
    };

    Ok(())
}
