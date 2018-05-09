use token::CommandData;

use std::io::Write;

pub fn run(cmd: CommandData) -> Result<(), String> {
    let mut out = cmd.out.unwrap();
    out.write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap();
    return Ok(());
}

#[test]
fn test_bye_run() {
    use std::fs::File;
    use token::Output;

    let cmd = CommandData {
        program: "clear".to_string(),
        options: vec![],
        out: Some(Output::from(File::create("/dev/null").unwrap())),
        input: None,
    };

    assert!(run(cmd).is_ok());
}
