use token::CommandData;

use std::io::Write;
use std::{thread, time};

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
        let tanakh = format!("{}", tanakhs[n % tanakhs.len()]);
        out.write_all("\x1b[2K\x1b[1G".as_bytes()).unwrap();
        out.write_all(tanakh.as_bytes()).unwrap();

        let t = time::Duration::from_millis(200);
        thread::sleep(t);

        n += 1;
        if n > 100 {
            out.write_all("\n".as_bytes()).unwrap();
            break;
        }
    }
    return Ok(());
}
