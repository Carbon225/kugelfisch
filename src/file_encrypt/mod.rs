#[cfg(test)]
mod tests;

use crate::cbc::{Encryptor, Decryptor, BlockProcessor};
use crate::blowfish;
use std::io::{Read, Write};
use std::error::Error;

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
                    output: &mut dyn Write,
                    key: &[u32],
                    iv: Option<u64>) -> Result<(), Box<dyn Error>> {
    let iv: u64 = iv.unwrap_or_else(|| rand::random());
    output.write_all(&iv.to_le_bytes())?;

    let (p, s) = blowfish::generate_keys(key);
    let algo = |x| blowfish::encrypt(x, &p, &s);

    let mut encryptor = Encryptor::new(&algo, iv);
    process_file(&mut encryptor, input, output)?;

    Ok(())
}

pub fn decrypt_file(input: &mut dyn Read,
                    output: &mut dyn Write,
                    key: &[u32]) -> Result<(), Box<dyn Error>> {
    let mut iv = [0; 8];
    input.read_exact(&mut iv)?;
    let iv = u64::from_le_bytes(iv);

    let (p, s) = blowfish::generate_keys(key);
    let algo = |x| blowfish::decrypt(x, &p, &s);

    let mut decryptor = Decryptor::new(&algo, iv);
    process_file(&mut decryptor, input, output)?;

    Ok(())
}
