use super::*;
use std::io::{Read, Write};

const TEST_FILE_SIZE: usize = 256;
const SAMPLE_DATA: [u8; TEST_FILE_SIZE] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255];
const EXPECTED_CIPHER: [u8; TEST_FILE_SIZE + 8] = [97, 98, 97, 98, 97, 98, 97, 98, 158, 156, 156, 158, 154, 152, 152, 154, 105, 106, 105, 106, 105, 106, 105, 106, 134, 132, 132, 134, 130, 128, 128, 130, 97, 98, 97, 98, 97, 98, 97, 98, 190, 188, 188, 190, 186, 184, 184, 186, 105, 106, 105, 106, 105, 106, 105, 106, 166, 164, 164, 166, 162, 160, 160, 162, 97, 98, 97, 98, 97, 98, 97, 98, 222, 220, 220, 222, 218, 216, 216, 218, 105, 106, 105, 106, 105, 106, 105, 106, 198, 196, 196, 198, 194, 192, 192, 194, 97, 98, 97, 98, 97, 98, 97, 98, 254, 252, 252, 254, 250, 248, 248, 250, 105, 106, 105, 106, 105, 106, 105, 106, 230, 228, 228, 230, 226, 224, 224, 226, 97, 98, 97, 98, 97, 98, 97, 98, 30, 28, 28, 30, 26, 24, 24, 26, 105, 106, 105, 106, 105, 106, 105, 106, 6, 4, 4, 6, 2, 0, 0, 2, 97, 98, 97, 98, 97, 98, 97, 98, 62, 60, 60, 62, 58, 56, 56, 58, 105, 106, 105, 106, 105, 106, 105, 106, 38, 36, 36, 38, 34, 32, 32, 34, 97, 98, 97, 98, 97, 98, 97, 98, 94, 92, 92, 94, 90, 88, 88, 90, 105, 106, 105, 106, 105, 106, 105, 106, 70, 68, 68, 70, 66, 64, 64, 66, 97, 98, 97, 98, 97, 98, 97, 98, 126, 124, 124, 126, 122, 120, 120, 122, 105, 106, 105, 106, 105, 106, 105, 106, 102, 100, 100, 102, 98, 96, 96, 98, 97, 98, 97, 98, 97, 98, 97, 98];

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

#[test]
fn encrypt_random() {
    let mut clear_file = open_temp_file("encrypt.clear", Write);
    clear_file.write_all(&SAMPLE_DATA).unwrap();

    let clear_file = open_temp_file("encrypt.clear", Read);
    let mut cipher_file = open_temp_file("encrypt.cipher", Write);
    encrypt_file(&clear_file, &mut cipher_file).unwrap();

    let mut actual = Vec::new();
    let mut cipher_file = open_temp_file("encrypt.cipher", Read);
    cipher_file.read_to_end(&mut actual).unwrap();

    assert_eq!(actual[..], EXPECTED_CIPHER);
}

#[test]
fn decrypt_random() {
    let mut cipher_file = open_temp_file("decrypt.cipher", Write);
    cipher_file.write_all(&EXPECTED_CIPHER).unwrap();

    let cipher_file = open_temp_file("decrypt.cipher", Read);
    let mut decrypted_file = open_temp_file("decrypt.decrypted", Write);
    decrypt_file(&cipher_file, &mut decrypted_file).unwrap();

    let mut decrypted_file = open_temp_file("decrypt.decrypted", Read);
    let mut actual = Vec::new();
    decrypted_file.read_to_end(&mut actual).unwrap();

    assert_eq!(actual[..], SAMPLE_DATA)
}
