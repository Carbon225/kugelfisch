#[cfg(test)]
mod tests;

use kugelfisch::{ProgramIntent, ProgramConfig, ProgramError};

pub fn parse_args(args: Vec<&str>) -> Result<ProgramIntent, ProgramError> {
    let err = |msg: &str| Err(ProgramError::InvalidArgs(msg.to_owned()));

    enum Mode {
        Encrypt,
        Decrypt,
    }

    let mut mode = None;
    let mut pass = None;

    let mut from = None;
    let mut to = None;

    let mut i = 1;
    while i < args.len() {
        match args[i] {
            "-e" => {
                if mode.is_some() {
                    return err("Conflicting arguments: -d -e");
                }
                mode = Some(Mode::Encrypt);
                i += 1;
            }
            "-d" => {
                if mode.is_some() {
                    return err("Conflicting arguments: -e -d");
                }
                mode = Some(Mode::Decrypt);
                i += 1;
            }
            "-p" => {
                if i + 1 >= args.len() {
                    return err("-p used without passphrase");
                }
                pass = Some(args[i + 1]);
                i += 2;
            }
            other => {
                if from.is_none() {
                    from = Some(other);
                }
                else if to.is_none() {
                    to = Some(other);
                }
                else {
                    return err("Invalid arguments");
                }
                i += 1;
            }
        }
    }

    if from.is_none() || to.is_none() {
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
