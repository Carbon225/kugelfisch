#[cfg(test)]
mod tests;

use kugelfisch::{ProgramIntent, ProgramConfig, ProgramError};

pub fn parse_args(mut args: Vec<&str>) -> Result<ProgramIntent, ProgramError> {
    let err = |msg: &str| Err(ProgramError::InvalidArgs(msg.to_owned()));

    enum Mode {
        Encrypt,
        Decrypt,
    }

    let mut mode = None;
    let mut pass = None;

    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "-e" => {
                if mode.is_some() {
                    return err("Conflicting arguments: -d -e");
                }
                mode = Some(Mode::Encrypt);
                args.remove(i);
            }
            "-d" => {
                if mode.is_some() {
                    return err("Conflicting arguments: -e -d");
                }
                mode = Some(Mode::Decrypt);
                args.remove(i);
            }
            "-p" => {
                if i + 1 >= args.len() {
                    return err("-p used without passphrase");
                }
                pass = Some(args[i + 1]);
                args.remove(i);
                args.remove(i);
            }
            _ => i += 1
        }
    }

    let to = args.pop();
    let from = args.pop();

    if args.len() == 0 {
        return err("Not enough arguments");
    }

    let config = ProgramConfig::new(
        from.unwrap(),
        to.unwrap(),
        pass);

    match mode {
        Some(Mode::Encrypt) => Ok(ProgramIntent::Encrypt(config)),
        Some(Mode::Decrypt) => Ok(ProgramIntent::Decrypt(config)),
        _ => err("Mode not given")
    }
}
