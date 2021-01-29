use kugelfisch::{ProgramIntent, ProgramConfig, ProgramError, run_program};

fn print_usage(arg0: &str) {
    eprintln!("Usage: {} [-e] [-d] [-p PASSPHRASE] INPUT OUTPUT", arg0);
}

fn parse_args(args: &Vec<String>) -> Result<ProgramIntent, ProgramError> {
    let err = |msg: &str| Err(ProgramError::InvalidArgs(msg.to_owned()));
    let contains = |x: &str| args.iter().any(|a| a == x);

    if args.len() < 3 {
        return err("Not enough arguments");
    }

    if !contains("-d") && !contains("-e") {
        return err("Mode not given");
    }

    if contains("-d") && contains("-e") {
        return err("Conflicting arguments: -d -e");
    }

    let pass = args.windows(2)
        .find_map(|p| if p[0] == "-p" {
            Some(&p[1])
        } else {
            None
        });

    if args.iter().rev().take(2).any(|arg|
        arg == "-e" || arg == "-d" || (pass.is_some() && arg == pass.unwrap())) {
        return err("Missing input/output");
    }

    let config = ProgramConfig::new(
        &args[args.len() - 2],
        &args[args.len() - 1],
        pass.map(|p| p.as_str()));

    if contains("-d") {
        Ok(ProgramIntent::Decrypt(config))
    } else {
        Ok(ProgramIntent::Encrypt(config))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match parse_args(&args) {
        Ok(intent) => run_program(&intent),
        Err(e) => Err(e),
    }.unwrap_or_else(|e| {
        eprintln!("{}", e);
        print_usage(&args[0]);
    });
}
