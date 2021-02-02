use super::*;
use std::io::{Read, Write, Seek, SeekFrom};

fn algo(x: u8) -> u8 {
    !x
}

#[test]
fn encrypt_single_block() {
    let b1 = 0b10101010; // cipher
    let b2 = 0b00001111; // plain
    let b3 = cbc_encrypt(b1, b2, &algo);
    assert_eq!(b3, 0b01011010)
}

#[test]
fn decrypt_single_block() {
    let b1 = 0b10101010; // cipher
    let b2 = 0b01011010; // cipher
    let b3 = cbc_decrypt(b1, b2, &algo);
    assert_eq!(b3, 0b00001111)
}

#[test]
fn encrypt_data() {
    let mut data = vec![0xde, 0xad, 0xbe, 0xef];
    let expected = [0xde, 0x8c, 0xcd, 0xdd];
    encrypt_blocks(&mut data[..], &algo);
    assert_eq!(data, expected);
}

#[test]
fn decrypt_data() {
    let mut data = vec![0xde, 0x8c, 0xcd, 0xdd];
    let expected = [0xde, 0xad, 0xbe, 0xef];
    decrypt_blocks(&mut data[..], &algo);
    assert_eq!(data, expected);
}

#[test]
fn encrypt_stream() {
    let mut data = vec![0xde, 0xad, 0xbe, 0xef];
    let expected = [0xde, 0x8c, 0xcd, 0xdd];

    let mut encryptor = Encryptor::new(&algo, data[0]);

    for i in 1..data.len() {
        data[i] = encryptor.next(data[i]);
    }

    assert_eq!(data, expected)
}

#[test]
fn decrypt_stream() {
    let mut data = vec![0xde, 0x8c, 0xcd, 0xdd];
    let expected = [0xde, 0xad, 0xbe, 0xef];

    let mut decryptor = Decryptor::new(&algo, data[0]);

    for i in 1..data.len() {
        data[i] = decryptor.next(data[i]);
    }

    assert_eq!(data, expected)
}

#[test]
fn encrypt_iter() {
    let data: Vec<u8> = vec![0xde, 0xad, 0xbe, 0xef];
    let expected: [u8; 3] = [0x8c, 0xcd, 0xdd];

    let enc: Vec<u8> = data
        .into_iter()
        .encrypt(&algo)
        .collect();

    assert_eq!(enc, expected);
}

#[test]
fn decrypt_iter() {
    let data = vec![0xdeu8, 0x8c, 0xcd, 0xdd];
    let expected = [0xad, 0xbe, 0xef];

    let enc: Vec<u8> = data
        .into_iter()
        .decrypt(&algo)
        .collect();

    assert_eq!(enc, expected);
}

#[test]
fn encrypt_bytes() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = tempfile::tempfile()?;
    file.write_all(&[0xde, 0xad, 0xbe, 0xef])?;

    file.seek(SeekFrom::Start(0))?;

    let data: Vec<u8> =  file.bytes().map(|b| b.unwrap()).encrypt(&algo).collect();

    assert_eq!(data, [0x8c, 0xcd, 0xdd]);

    Ok(())
}
