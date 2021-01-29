mod arg_parser;

use arg_parser::parse_args;
use kugelfisch::{run_program, ProgramError};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args_ref: Vec<&str> = args.iter().map(|arg| arg.as_str()).collect();

    if let Some(e) = parse_args(args_ref).and_then(|intent| run_program(&intent)).err() {
        eprintln!("{}", e);
        if let ProgramError::InvalidArgs(_) = e {
            eprintln!("Usage: {} [-e] [-d] [-p PASSPHRASE] INPUT OUTPUT", args[0]);
        }
    }
}
