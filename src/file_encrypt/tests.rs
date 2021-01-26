use super::*;
use std::fs::File;
use std::io::{Read, Write};

const TEST_FILE_SIZE: usize = 203;
const TEST_FILE_SIZE_PADDED: usize = TEST_FILE_SIZE - (TEST_FILE_SIZE % 8) + 8;
const SAMPLE_DATA: [u8; TEST_FILE_SIZE] = [46, 140, 154, 241, 52, 106, 139, 241, 181, 93, 125, 162, 73, 15, 52, 120, 168, 153, 77, 29, 118, 162, 215, 50, 60, 95, 64, 197, 188, 66, 53, 148, 200, 112, 16, 77, 167, 192, 178, 230, 62, 214, 68, 23, 25, 19, 207, 92, 237, 139, 58, 183, 29, 208, 98, 179, 143, 147, 80, 37, 217, 205, 69, 41, 87, 163, 165, 211, 117, 45, 135, 215, 171, 241, 161, 52, 251, 44, 104, 252, 3, 104, 125, 106, 244, 200, 235, 72, 127, 104, 249, 75, 254, 175, 65, 22, 73, 131, 88, 84, 103, 216, 238, 184, 188, 83, 70, 62, 3, 127, 104, 30, 233, 100, 189, 59, 26, 232, 111, 76, 117, 93, 252, 30, 117, 127, 23, 165, 197, 216, 10, 205, 83, 210, 121, 159, 122, 100, 28, 14, 180, 201, 157, 251, 20, 108, 239, 77, 188, 139, 169, 204, 252, 223, 250, 118, 45, 200, 60, 92, 189, 254, 229, 100, 188, 246, 67, 116, 247, 157, 91, 1, 207, 230, 254, 0, 99, 56, 250, 169, 183, 233, 121, 250, 221, 11, 177, 169, 170, 188, 46, 57, 68, 235, 110, 45, 70, 78, 53, 146, 172, 13, 68];
const SAMPLE_DATA_PADDED: [u8; TEST_FILE_SIZE_PADDED] = [46, 140, 154, 241, 52, 106, 139, 241, 181, 93, 125, 162, 73, 15, 52, 120, 168, 153, 77, 29, 118, 162, 215, 50, 60, 95, 64, 197, 188, 66, 53, 148, 200, 112, 16, 77, 167, 192, 178, 230, 62, 214, 68, 23, 25, 19, 207, 92, 237, 139, 58, 183, 29, 208, 98, 179, 143, 147, 80, 37, 217, 205, 69, 41, 87, 163, 165, 211, 117, 45, 135, 215, 171, 241, 161, 52, 251, 44, 104, 252, 3, 104, 125, 106, 244, 200, 235, 72, 127, 104, 249, 75, 254, 175, 65, 22, 73, 131, 88, 84, 103, 216, 238, 184, 188, 83, 70, 62, 3, 127, 104, 30, 233, 100, 189, 59, 26, 232, 111, 76, 117, 93, 252, 30, 117, 127, 23, 165, 197, 216, 10, 205, 83, 210, 121, 159, 122, 100, 28, 14, 180, 201, 157, 251, 20, 108, 239, 77, 188, 139, 169, 204, 252, 223, 250, 118, 45, 200, 60, 92, 189, 254, 229, 100, 188, 246, 67, 116, 247, 157, 91, 1, 207, 230, 254, 0, 99, 56, 250, 169, 183, 233, 121, 250, 221, 11, 177, 169, 170, 188, 46, 57, 68, 235, 110, 45, 70, 78, 53, 146, 172, 13, 68, 0, 0, 0, 0, 0];
const EXPECTED_CIPHER: [u8; TEST_FILE_SIZE_PADDED + 8] = [98, 97, 98, 97, 98, 97, 98, 97, 179, 18, 7, 111, 169, 244, 22, 111, 249, 176, 133, 50, 31, 4, 221, 232, 174, 214, 55, 208, 150, 89, 245, 37, 109, 118, 136, 234, 213, 228, 63, 78, 90, 249, 103, 88, 141, 219, 114, 87, 155, 208, 220, 176, 107, 55, 66, 244, 137, 164, 25, 248, 137, 24, 223, 184, 249, 200, 182, 34, 175, 42, 101, 110, 81, 148, 236, 14, 37, 248, 29, 70, 5, 154, 178, 197, 33, 43, 138, 69, 249, 13, 48, 80, 42, 28, 158, 242, 121, 154, 54, 228, 43, 76, 32, 27, 207, 230, 145, 79, 179, 107, 49, 92, 140, 74, 40, 142, 79, 235, 166, 189, 154, 209, 106, 74, 170, 252, 54, 14, 16, 115, 105, 171, 32, 124, 222, 84, 42, 84, 156, 153, 140, 81, 88, 52, 175, 207, 127, 104, 199, 103, 58, 48, 68, 92, 111, 218, 132, 19, 108, 3, 71, 124, 106, 83, 86, 36, 175, 160, 5, 125, 112, 200, 21, 45, 19, 43, 13, 31, 212, 54, 37, 52, 18, 212, 145, 216, 209, 96, 109, 34, 148, 209, 179, 44, 159, 54, 56, 97, 69, 23, 8, 56, 14, 228, 129, 208, 143, 122, 91, 202, 181, 27, 126, 47, 112, 133];

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

#[test]
fn encrypts_sample() {
    write_to_file("encrypt.clear", &SAMPLE_DATA);

    let mut clear_file = open_temp_file("encrypt.clear", Read);
    let mut cipher_file = open_temp_file("encrypt.cipher", Write);
    encrypt_file(&mut clear_file, &mut cipher_file).unwrap();

    check_file("encrypt.cipher", &EXPECTED_CIPHER);
}

#[test]
fn decrypts_sample() {
    write_to_file("decrypt.cipher", &EXPECTED_CIPHER);

    let mut cipher_file = open_temp_file("decrypt.cipher", Read);
    let mut decrypted_file = open_temp_file("decrypt.decrypted", Write);
    decrypt_file(&mut cipher_file, &mut decrypted_file).unwrap();

    check_file("decrypt.decrypted", &SAMPLE_DATA_PADDED);
}

#[test]
fn encrypts_and_decrypts() {
    write_to_file("enc_dec.clear", &SAMPLE_DATA);

    let mut clear_file = open_temp_file("enc_dec.clear", Read);
    let mut cipher_file = open_temp_file("enc_dec.cipher", Write);
    encrypt_file(&mut clear_file, &mut cipher_file).unwrap();

    let mut cipher_file = open_temp_file("enc_dec.cipher", Read);
    let mut decrypted_file = open_temp_file("enc_dec.decrypted", Write);
    decrypt_file(&mut cipher_file, &mut decrypted_file).unwrap();

    check_file("enc_dec.decrypted", &SAMPLE_DATA_PADDED);
}
