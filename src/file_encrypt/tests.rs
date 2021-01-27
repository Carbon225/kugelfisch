use super::*;
use std::fs::File;
use std::io::{Read, Write};

const TEST_FILE_SIZE: usize = 203;
const TEST_FILE_SIZE_PADDED: usize = TEST_FILE_SIZE - (TEST_FILE_SIZE % 8) + 8;
const SAMPLE_KEY: [u32; 4] = [1, 2, 3, 4];
const SAMPLE_DATA: [u8; TEST_FILE_SIZE] = [46, 140, 154, 241, 52, 106, 139, 241, 181, 93, 125, 162, 73, 15, 52, 120, 168, 153, 77, 29, 118, 162, 215, 50, 60, 95, 64, 197, 188, 66, 53, 148, 200, 112, 16, 77, 167, 192, 178, 230, 62, 214, 68, 23, 25, 19, 207, 92, 237, 139, 58, 183, 29, 208, 98, 179, 143, 147, 80, 37, 217, 205, 69, 41, 87, 163, 165, 211, 117, 45, 135, 215, 171, 241, 161, 52, 251, 44, 104, 252, 3, 104, 125, 106, 244, 200, 235, 72, 127, 104, 249, 75, 254, 175, 65, 22, 73, 131, 88, 84, 103, 216, 238, 184, 188, 83, 70, 62, 3, 127, 104, 30, 233, 100, 189, 59, 26, 232, 111, 76, 117, 93, 252, 30, 117, 127, 23, 165, 197, 216, 10, 205, 83, 210, 121, 159, 122, 100, 28, 14, 180, 201, 157, 251, 20, 108, 239, 77, 188, 139, 169, 204, 252, 223, 250, 118, 45, 200, 60, 92, 189, 254, 229, 100, 188, 246, 67, 116, 247, 157, 91, 1, 207, 230, 254, 0, 99, 56, 250, 169, 183, 233, 121, 250, 221, 11, 177, 169, 170, 188, 46, 57, 68, 235, 110, 45, 70, 78, 53, 146, 172, 13, 68];
const SAMPLE_DATA_PADDED: [u8; TEST_FILE_SIZE_PADDED] = [46, 140, 154, 241, 52, 106, 139, 241, 181, 93, 125, 162, 73, 15, 52, 120, 168, 153, 77, 29, 118, 162, 215, 50, 60, 95, 64, 197, 188, 66, 53, 148, 200, 112, 16, 77, 167, 192, 178, 230, 62, 214, 68, 23, 25, 19, 207, 92, 237, 139, 58, 183, 29, 208, 98, 179, 143, 147, 80, 37, 217, 205, 69, 41, 87, 163, 165, 211, 117, 45, 135, 215, 171, 241, 161, 52, 251, 44, 104, 252, 3, 104, 125, 106, 244, 200, 235, 72, 127, 104, 249, 75, 254, 175, 65, 22, 73, 131, 88, 84, 103, 216, 238, 184, 188, 83, 70, 62, 3, 127, 104, 30, 233, 100, 189, 59, 26, 232, 111, 76, 117, 93, 252, 30, 117, 127, 23, 165, 197, 216, 10, 205, 83, 210, 121, 159, 122, 100, 28, 14, 180, 201, 157, 251, 20, 108, 239, 77, 188, 139, 169, 204, 252, 223, 250, 118, 45, 200, 60, 92, 189, 254, 229, 100, 188, 246, 67, 116, 247, 157, 91, 1, 207, 230, 254, 0, 99, 56, 250, 169, 183, 233, 121, 250, 221, 11, 177, 169, 170, 188, 46, 57, 68, 235, 110, 45, 70, 78, 53, 146, 172, 13, 68, 0, 0, 0, 0, 0];
const EXPECTED_CIPHER: [u8; TEST_FILE_SIZE_PADDED + 8] = [98, 97, 98, 97, 98, 97, 98, 97, 90, 62, 228, 33, 191, 71, 4, 84, 97, 165, 64, 40, 28, 121, 180, 203, 234, 107, 231, 105, 252, 177, 245, 150, 61, 60, 78, 155, 171, 41, 214, 141, 69, 211, 159, 95, 252, 254, 1, 163, 187, 251, 164, 242, 91, 56, 102, 108, 225, 168, 94, 233, 105, 0, 52, 79, 30, 214, 77, 42, 222, 181, 233, 47, 249, 6, 208, 27, 224, 86, 117, 95, 40, 214, 169, 191, 202, 15, 43, 53, 174, 153, 77, 209, 214, 21, 158, 152, 249, 137, 234, 178, 198, 160, 180, 73, 187, 215, 9, 214, 4, 41, 58, 224, 177, 26, 214, 78, 184, 215, 118, 116, 208, 236, 211, 215, 37, 174, 4, 175, 47, 209, 198, 72, 33, 119, 252, 216, 236, 174, 76, 83, 44, 138, 184, 80, 102, 255, 251, 64, 72, 215, 196, 101, 185, 1, 51, 175, 88, 146, 82, 198, 27, 136, 91, 72, 219, 36, 239, 50, 37, 100, 167, 107, 34, 215, 48, 128, 46, 25, 177, 58, 157, 59, 204, 32, 68, 209, 58, 22, 181, 35, 67, 88, 71, 166, 88, 127, 133, 66, 219, 180, 27, 89, 235, 9, 223, 51, 204, 35, 241, 43, 107, 146, 175, 160, 209, 151];

enum Mode {
    Read,
    Write,
}

use Mode::*;

fn open_temp_file(name: &str, mode: Mode) -> File {
    let path = std::env::temp_dir().join(String::from("kugelfisch.tests.") + name);
    match mode {
        Mode::Read => File::open(&path).unwrap(),
        Mode::Write => File::create(&path).unwrap()
    }
}

fn check_file(file: &str, expected: &[u8]) {
    let mut actual = Vec::new();
    let mut file = open_temp_file(file, Read);
    file.read_to_end(&mut actual).unwrap();

    assert_eq!(actual, expected);
}

fn write_to_file(file: &str, data: &[u8]) {
    let mut file = open_temp_file(file, Write);
    file.write_all(data).unwrap();
}

fn get_iv(data: &[u8]) -> u64 {
    let mut iv = [0u8; 8];
    iv.copy_from_slice(&data[..8]);
    u64::from_le_bytes(iv)
}

#[test]
fn encrypts_sample() {
    write_to_file("encrypt.clear", &SAMPLE_DATA);

    let mut clear_file = open_temp_file("encrypt.clear", Read);
    let mut cipher_file = open_temp_file("encrypt.cipher", Write);

    let iv = get_iv(&EXPECTED_CIPHER);
    encrypt_file(&mut clear_file, &mut cipher_file, &SAMPLE_KEY, Some(iv)).unwrap();

    check_file("encrypt.cipher", &EXPECTED_CIPHER);
}

#[test]
fn decrypts_sample() {
    write_to_file("decrypt.cipher", &EXPECTED_CIPHER);

    let mut cipher_file = open_temp_file("decrypt.cipher", Read);
    let mut decrypted_file = open_temp_file("decrypt.decrypted", Write);
    decrypt_file(&mut cipher_file, &mut decrypted_file, &SAMPLE_KEY).unwrap();

    check_file("decrypt.decrypted", &SAMPLE_DATA_PADDED);
}

#[test]
fn encrypts_and_decrypts() {
    write_to_file("enc_dec.clear", &SAMPLE_DATA);

    let mut clear_file = open_temp_file("enc_dec.clear", Read);
    let mut cipher_file = open_temp_file("enc_dec.cipher", Write);
    encrypt_file(&mut clear_file, &mut cipher_file, &SAMPLE_KEY, None).unwrap();

    let mut cipher_file = open_temp_file("enc_dec.cipher", Read);
    let mut decrypted_file = open_temp_file("enc_dec.decrypted", Write);
    decrypt_file(&mut cipher_file, &mut decrypted_file, &SAMPLE_KEY).unwrap();

    check_file("enc_dec.decrypted", &SAMPLE_DATA_PADDED);
}
