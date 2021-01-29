use super::parse_args;
use kugelfisch::ProgramIntent;

fn assert_encrypt(intent: &ProgramIntent,
                  input: &str,
                  output: &str,
                  passwd: &Option<&str>) {
    if let ProgramIntent::Encrypt(config) = intent {
        assert_eq!(config.input_path, input);
        assert_eq!(config.output_path, output);
        assert_eq!(config.passphrase, *passwd);
    } else {
        panic!("Invalid mode");
    }
}

fn get_args(cmd: &str) -> Vec<&str> {
    cmd.split(" ").collect()
}

#[test]
fn parse_encrypt_passwd_1() {
    let cmd = get_args("bin -e -p passwd from to");
    let intent = parse_args(cmd).unwrap();
    assert_encrypt(&intent, "from", "to", &Some("passwd"));
}

#[test]
fn parse_encrypt_passwd_2() {
    let cmd = get_args("bin -p passwd -e from to");
    let intent = parse_args(cmd).unwrap();
    assert_encrypt(&intent, "from", "to", &Some("passwd"));
}

#[test]
fn parse_encrypt_passwd_3() {
    let cmd = get_args("bin -p passwd from to -e");
    let intent = parse_args(cmd).unwrap();
    assert_encrypt(&intent, "from", "to", &Some("passwd"));
}

#[test]
#[should_panic]
fn parse_incorrect_mode_after_passwd() {
    let cmd = get_args("bin -p -e passwd from to");
    parse_args(cmd).unwrap();
}

#[test]
#[should_panic]
fn parse_incorrect_empty_passwd_1() {
    let cmd = get_args("bin -p -e from to");
    parse_args(cmd).unwrap();
}

#[test]
#[should_panic]
fn parse_incorrect_empty_passwd_2() {
    let cmd = get_args("bin -e -p from to");
    parse_args(cmd).unwrap();
}

#[test]
#[should_panic]
fn parse_incorrect_no_mode() {
    let cmd = get_args("bin from to");
    parse_args(cmd).unwrap();
}

#[test]
fn parse_encrypt_no_passwd() {
    let cmd = get_args("bin -e from to");
    let intent = parse_args(cmd).unwrap();
    assert_encrypt(&intent, "from", "to", &None);
}
