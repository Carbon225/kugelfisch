use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::{NamedTempFile, TempPath};
use std::io::{Write, Read};
use std::error::Error;

#[derive(Debug)]
struct PathError {}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path unwrapping error")
    }
}

impl Error for PathError {}

fn path_str(path: &std::path::Path) -> Result<&str, PathError> {
    path.to_str().ok_or(PathError {})
}

fn get_temp_path() -> Result<TempPath, Box<dyn Error>> {
    Ok(NamedTempFile::new()?.into_temp_path())
}

fn kf(args: &[&str]) -> Result<Command, Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("kugelfisch")?;
    cmd.args(args);
    Ok(cmd)
}

fn test_encrypt_decrypt(encrypt_args: &[&str],
                        encrypt_stdin: Option<&str>,
                        decrypt_args: &[&str],
                        decrypt_stdin: Option<&str>) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let mut plain_file = NamedTempFile::new()?;
    plain_file.write_all(TEST_DATA.as_bytes())?;

    let cipher_file = get_temp_path()?;

    let args: Vec<&str> = encrypt_args.into_iter().chain([
        path_str(plain_file.path())?,
        path_str(&cipher_file)?,
    ].iter()).map(|x| *x).collect();

    let mut cmd = kf(args.as_slice())?;
    if let Some(stdin) = encrypt_stdin {
        cmd.write_stdin(stdin);
    }
    cmd.assert()
        .success();

    let mut decrypt_file = NamedTempFile::new()?;

    let args: Vec<&str> = decrypt_args.into_iter().chain([
        path_str(&cipher_file)?,
        path_str(decrypt_file.path())?,
    ].iter()).map(|x| *x).collect();

    let mut cmd = kf(args.as_slice())?;
    if let Some(stdin) = decrypt_stdin {
        cmd.write_stdin(stdin);
    }
    cmd.assert()
        .success();

    let mut contents = Vec::new();
    decrypt_file.read_to_end(&mut contents)?;

    Ok((contents, TEST_DATA.as_bytes().to_owned()))
}

const TEST_DATA: &str = "Test data to encrypt\nwith blowfish\n";

// --- TESTS ---

#[test]
fn no_such_file() -> Result<(), Box<dyn std::error::Error>> {
    kf(&["-e", "-p", "passwd", "no/such/file", "to"])?
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
    Ok(())
}

#[test]
fn encrypt_pass_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut plain_file = NamedTempFile::new()?;
    plain_file.write_all(TEST_DATA.as_bytes())?;

    let cipher_file = get_temp_path()?;

    kf(&[
        "-e", "-p", "1234",
        path_str(plain_file.path())?,
        path_str(&cipher_file)?,
    ])?
        .assert()
        .success();

    Ok(())
}

#[test]
fn encrypt_decrypt() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e", "-p", "1234"], None,
        &["-d", "-p", "1234"], None,
    )?;
    assert_eq!(&left[..right.len()], right);
    Ok(())
}

#[test]
fn encrypt_decrypt_wrong_pass() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e", "-p", "1234"], None,
        &["-d", "-p", "12345"], None,
    )?;
    assert_ne!(&left[..right.len()], right);
    Ok(())
}

#[test]
fn encrypt_stdin_decrypt() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e"], Some("1234"),
        &["-d", "-p", "1234"], None,
    )?;
    assert_eq!(&left[..right.len()], right);
    Ok(())
}

#[test]
fn encrypt_decrypt_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e", "-p", "1234"], None,
        &["-d"], Some("1234"),
    )?;
    assert_eq!(&left[..right.len()], right);
    Ok(())
}

#[test]
fn encrypt_stdin_lfcr_decrypt() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e"], Some("1234\n\r"),
        &["-d", "-p", "1234"], None,
    )?;
    assert_eq!(&left[..right.len()], right);
    Ok(())
}

#[test]
fn encrypt_decrypt_stdin_lfcr() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e", "-p", "1234"], None,
        &["-d"], Some("1234\n\r"),
    )?;
    assert_eq!(&left[..right.len()], right);
    Ok(())
}

#[test]
fn encrypt_stdin_decrypt_wrong() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e"], Some("1234"),
        &["-d", "-p", "12345"], None,
    )?;
    assert_ne!(&left[..right.len()], right);
    Ok(())
}

#[test]
fn encrypt_decrypt_stdin_wrong() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e", "-p", "1234"], None,
        &["-d"], Some("12345"),
    )?;
    assert_ne!(&left[..right.len()], right);
    Ok(())
}

#[test]
fn encrypt_stdin_lfcr_decrypt_wrong() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e"], Some("1234\n\r"),
        &["-d", "-p", "12345"], None,
    )?;
    assert_ne!(&left[..right.len()], right);
    Ok(())
}

#[test]
fn encrypt_decrypt_stdin_lfcr_wrong() -> Result<(), Box<dyn std::error::Error>> {
    let (left, right) = test_encrypt_decrypt(
        &["-e", "-p", "1234"], None,
        &["-d"], Some("12345\n\r"),
    )?;
    assert_ne!(&left[..right.len()], right);
    Ok(())
}
