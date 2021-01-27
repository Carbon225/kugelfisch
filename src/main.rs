use kugelfisch::{ProgramIntent, ProgramConfig, ProgramError, run_program};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} e|d FILE [PASSPHRASE]", args[0]);
        return;
    }

    let passphrase = if args.len() < 4 {
        None
    } else {
        Some(args[3].as_str())
    };

    let config = ProgramConfig::new(&args[2], passphrase);
    match args[1].as_str() {
        "e" => run_program(&ProgramIntent::Encrypt(config)),
        "d" => run_program(&ProgramIntent::Decrypt(config)),
        _ => Err(ProgramError::InvalidMode(String::from(&args[1])))
    }.unwrap_or_else(|e| eprintln!("{}", e));
}
