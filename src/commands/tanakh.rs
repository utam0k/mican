use token::CommandData;

use std::io::Write;
use std::{thread, time};


// TODO
#[cfg_attr(feature = "cargo-clippy", allow(non_ascii_literal))]
pub fn run(cmd: CommandData) -> Result<(), String> {
    let mut out = cmd.out.unwrap();
    let tanakhs = vec![
        "  (´･_･`)´･_･`)  ",
        "   (´･_･`)_･`)   ",
        "    (´･_･`)`)    ",
        "    ((´･_･`)     ",
        "   (´･(´･_･`)    ",
        "   (´･_(´･_･`)   ",
        "  (´･_･`)´･_･`)  ",
        "   (´･_･`)_･`)   ",
        "    (´･_･`)`)    ",
        "    (´･_･`))     ",
        "     ((´･_･`)    ",
        "    (´･(´･_･`)   ",
        "   (´･_(´･_･`)   ",
    ];

    let mut n = 0;
    loop {
        let tanakh = tanakhs[n % tanakhs.len()].to_string();
        out.write_all(b"\x1b[2K\x1b[1G").unwrap();
        out.write_all(tanakh.as_bytes()).unwrap();

        let t = time::Duration::from_millis(200);
        thread::sleep(t);

        n += 1;
        if n > 100 {
            out.write_all(b"\n").unwrap();
            break;
        }
    }
    Ok(())
}
