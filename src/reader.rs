use std::io;
use std::io::Write;
use std::borrow::Cow;

use libc::STDIN_FILENO;

use nix;
use nix::sys::termios::{tcgetattr, tcsetattr, SetArg, LocalFlags, InputFlags,
                        SpecialCharacterIndices};
use nix::sys::select::{select, FdSet};
use nix::unistd::read;

pub struct Reader {
    pos: i32,
    prompt: String,
    bindings: Vec<(Cow<'static, [u8]>, Keybind)>,
}

impl Reader {
    pub fn new(prompt: String) -> Self {
        settings_term();
        Reader {
            pos: 0,
            prompt: prompt,
            bindings: bindings(),
        }
    }

    pub fn read_line(&mut self) -> String {
        let mut line = Vec::new();

        print!("{}", self.prompt);
        io::stdout().flush().unwrap();
        loop {
            if wait_input() {
                let mut ch: Vec<u8> = Vec::new();
                let _a = self.read_char(&mut ch).unwrap();
                let mut res = None;
                for (ref bind, ref cmd) in &self.bindings {
                    if bind == &ch {
                        res = Some(cmd);
                    }
                }
                match res {
                    Some(Keybind::Enter) => {
                        println!("");
                        self.pos = 0;
                        return line.concat();
                    }
                    Some(Keybind::CtrlL) => {
                        print!("\x1b[2J\x1b[1;1H");
                        print!("{}{}", self.prompt, line.concat());
                        io::stdout().flush().unwrap();
                    }
                    Some(Keybind::Delete) => {
                        line.pop();
                        print!("\x1b[1D\x1b[J");
                        if self.pos > -1 {
                            self.pos -= 1;
                        }
                        io::stdout().flush().unwrap();
                    } 
                    None => {
                        let c = String::from_utf8(ch).unwrap();
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                        self.pos += 1;
                        line.push(c);
                    }
                }
            }
        }
    }

    fn read_char(&self, buf: &mut Vec<u8>) -> nix::Result<usize> {
        buf.reserve(32);

        let len = buf.len();
        let cap = buf.capacity();
        let n;

        unsafe {
            buf.set_len(cap);
            let result = retry(|| read(STDIN_FILENO, &mut buf[len..]));

            buf.set_len(len);

            n = result?;
            buf.set_len(len + n);
        }

        Ok(n)
    }
}

fn settings_term() {
    let old_tio = tcgetattr(STDIN_FILENO).unwrap();
    let mut tio = old_tio;

    tio.input_flags.remove(
        InputFlags::INLCR | InputFlags::ICRNL,
    );
    tio.local_flags.remove(
        LocalFlags::ICANON | LocalFlags::ECHO,
    );
    tio.control_chars[SpecialCharacterIndices::VMIN as usize] = 0;
    tio.control_chars[SpecialCharacterIndices::VTIME as usize] = 0;

    tcsetattr(STDIN_FILENO, SetArg::TCSANOW, &tio).unwrap();
}

fn retry<F, R>(mut f: F) -> nix::Result<R>
where
    F: FnMut() -> nix::Result<R>,
{
    loop {
        match f() {                                                                                                                                                                                             
            Err(_e) => (),                                                                                                                                         
            res => return res,                                                                                                                                                                                   
        }
    }
}

fn wait_input() -> bool {
    let mut r_fds = FdSet::new();
    r_fds.insert(STDIN_FILENO);

    let mut e_fds = FdSet::new();
    r_fds.insert(STDIN_FILENO);

    loop {
        match select(
            STDIN_FILENO + 1,
            Some(&mut r_fds),
            None,
            Some(&mut e_fds),
            None.as_mut(),
        ) {
            Ok(n) => return n == 1,
            Err(_e) => return false,
        }
    }
}

enum Keybind {
    Enter,
    Delete,
    CtrlL,
}

fn bindings() -> Vec<(Cow<'static, [u8]>, Keybind)> {
    vec![
        (Cow::Borrowed(b"\r"), Keybind::Enter),
        (Cow::Borrowed(b"\x7f"), Keybind::Delete),
        (Cow::Borrowed(b"\x0c"), Keybind::CtrlL),
    ]
}
