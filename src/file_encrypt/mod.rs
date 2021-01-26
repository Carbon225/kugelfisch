#[cfg(test)]
mod tests;

use crate::cbc::{Encryptor, Decryptor, BlockProcessor};
use std::io::{Read, Write};
use std::error::Error;

fn algo(x: u64) -> u64 {
    !x
}

fn process_file(cbc: &mut dyn BlockProcessor<u64>,
                input: &mut dyn Read,
                output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    loop {
        let mut buf = [0; 8];
        if input.read(&mut buf)? <= 0 {
            break;
        }

        let block = u64::from_le_bytes(buf);
        let block = cbc.next(block);
        output.write(&block.to_le_bytes())?;
    }

    Ok(())
}

pub fn encrypt_file(input: &mut dyn Read,
                    output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    // TODO generate IV
    let iv: u64 = 0x6162616261626162;
    output.write_all(&iv.to_le_bytes())?;

    let mut encryptor = Encryptor::new(&algo, iv);
    process_file(&mut encryptor, input, output)?;

    Ok(())
}

pub fn decrypt_file(input: &mut dyn Read,
                    output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    let mut iv = [0; 8];
    input.read_exact(&mut iv)?;
    let iv = u64::from_le_bytes(iv);

    let mut decryptor = Decryptor::new(&algo, iv);
    process_file(&mut decryptor, input, output)?;

    Ok(())
}
