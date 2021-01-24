#[cfg(test)]
mod tests;

use crate::cbc::{Encryptor, Decryptor, BlockProcessor};
use std::io::{Read, Write};
use std::error::Error;

fn algo(x: u64) -> u64 {
    !x
}

fn process_file(cbc: &mut dyn BlockProcessor<u64>,
                ifile: &mut dyn Read,
                ofile: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    let mut block = 0;
    let mut pad = 0;
    for (i, b) in ifile.bytes().enumerate() {
        let b = b?;
        block <<= 8;
        block += b as u64;
        if i % 8 == 7 {
            let block = cbc.next(block);
            ofile.write(&block.to_be_bytes())?;
        }
        pad = 7 - (i % 8);
    }

    if pad > 0 {
        block <<= pad * 8;
        let block = cbc.next(block);
        ofile.write(&block.to_be_bytes())?;
    }

    Ok(())
}

pub fn encrypt_file(input: &mut dyn Read,
                    output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    // TODO generate IV
    let iv: u64 = 0x6162616261626162;
    output.write_all(&iv.to_be_bytes())?;

    let mut encryptor = Encryptor::new(&algo, iv);
    process_file(&mut encryptor, input, output)?;

    Ok(())
}

pub fn decrypt_file(input: &mut dyn Read,
                    output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    let mut iv: [u8; 8] = [0; 8];
    for (i, byte) in input.bytes().take(8).enumerate() {
        iv[i] = byte?;
    }
    let iv = u64::from_be_bytes(iv);

    let mut decryptor = Decryptor::new(&algo, iv);
    process_file(&mut decryptor, input, output)?;

    Ok(())
}
