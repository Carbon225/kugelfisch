mod blowfish;
mod cbc;
mod file_encrypt;
mod handle_intent;
mod passphrase;

use crate::handle_intent::{handle_decrypt, handle_encrypt};

use std::error::Error;
use std::fmt::{Display, Debug, Formatter};

pub struct ProgramConfig<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
    pub passphrase: Option<&'a str>,
}

impl<'a> ProgramConfig<'a> {
    pub fn new(input_path: &'a str, output_path: &'a str, passphrase: Option<&'a str>) -> Self {
        ProgramConfig { input_path, output_path, passphrase }
    }
}

pub enum ProgramIntent<'a> {
    Encrypt(ProgramConfig<'a>),
    Decrypt(ProgramConfig<'a>),
}

#[derive(Debug)]
pub enum ProgramError {
    InvalidArgs(String),
    ErrorOpening(String, Box<dyn Error>),
    ErrorCreating(String, Box<dyn Error>),
    OperationFailed(Box<dyn Error>),
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            ProgramError::InvalidArgs(msg) => msg.clone(),
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
