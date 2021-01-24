use kugelfisch::file_encrypt::{encrypt_file, decrypt_file};
use std::fs::File;
use std::io::{Write};
use std::error::Error;
use std::fmt::{Display, Debug, Formatter};

#[derive(Debug)]
enum ProgramError {
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

fn process_file(path: &str, out: &str, op: fn(&File, &mut dyn Write) -> Result<(), Box<dyn Error>>) -> Result<(), Box<dyn Error>> {
    let ifile = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(Box::from(ProgramError::ErrorOpening(String::from(path), Box::from(e))));
        }
    };

    let mut ofile = match File::create(out) {
        Ok(f) => f,
        Err(e) => {
            return Err(Box::from(ProgramError::ErrorCreating(String::from(out), Box::from(e))));
        }
    };

    if let Err(e) = op(&ifile, &mut ofile) {
        return Err(Box::from(ProgramError::OperationFailed(Box::from(e))));
    };

    Ok(())
}

fn handle_encrypt(path: &str, passphrase: Option<&str>) -> Result<(), Box<dyn Error>> {
    let out_path = String::from(path) + ".kugelfisch";

    println!("Encrypting {} to {}", path, out_path);
    if let Some(pass) = passphrase {
        println!("Using passphrase {}", pass);
    }

    process_file(path, out_path.as_str(), encrypt_file)
}

fn handle_decrypt(path: &str, passphrase: Option<&str>) -> Result<(), Box<dyn Error>> {
    let out_path = String::from(path) + ".clear";

    println!("Decrypting {} to {}", path, out_path);
    if let Some(pass) = passphrase {
        println!("Using passphrase {}", pass);
    }

    process_file(path, out_path.as_str(), decrypt_file)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} e|d FILE [PASSPHRASE]", args[0]);
        return;
    }

    if let Err(e) = match args[1].as_str() {
        "e" => handle_encrypt(args[2].as_str(), args.get(3).and_then(|x| Some(x.as_str()))),
        "d" => handle_decrypt(args[2].as_str(), args.get(3).and_then(|x| Some(x.as_str()))),
        _ => Err(Box::from(ProgramError::InvalidMode(String::from(&args[1])))),
    } {
        eprintln!("{}", e);
    }
}
