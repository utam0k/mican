use std::borrow::Cow;

use libc::STDIN_FILENO;

use nix;
use nix::sys::termios::{tcgetattr, tcsetattr, SetArg, LocalFlags, InputFlags,
                        SpecialCharacterIndices};
use nix::sys::select::{select, FdSet};
use nix::unistd::read;

use term::Term;

pub struct Reader {
    term: Term,
    bindings: Vec<(Cow<'static, [u8]>, Keybind)>,
}

impl Reader {
    pub fn new(prompt: String) -> Self {
        settings_term();
        Reader {
            term: Term::new(prompt),
            bindings: bindings(),
        }
    }

    pub fn read_line(&mut self) -> String {
        self.term.write_prompt().unwrap();
        loop {
            if wait_input() {
                let mut ch: Vec<u8> = Vec::new();
                let _ = self.read_char(&mut ch).unwrap();
                let res = self.find_bind(&ch);
                match res {
                    Some(Keybind::Enter) => {
                        let result = self.term.line.clone();
                        self.term.reset();
                        self.term.new_line().unwrap();
                        return result;
                    }
                    Some(Keybind::CtrlL) => {
                        self.term.clear_screen().unwrap();
                        self.term.write_line().unwrap();
                    }
                    Some(Keybind::Delete) => {
                        if self.term.pos > self.term.prompt.capacity() - 2 {
                            self.term.delete(1).unwrap();
                        }
                    } 
                    Some(Keybind::BackwardChar) => {
                        if self.term.pos > 0 {
                            self.term.move_left(1).unwrap();
                        }
                    }
                    Some(Keybind::ForwardChar) => {
                        self.term.move_right(1).unwrap();
                    }
                    None => {
                        self.term.put(String::from_utf8(ch).unwrap()).unwrap();
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

    fn find_bind(&self, ch: &Vec<u8>) -> Option<Keybind> {
        for &(ref bind, ref cmd) in &self.bindings {
            if bind == ch {
                return Some(cmd.clone());
            }
        }
        return None;
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

#[derive(Clone)]
enum Keybind {
    Enter,
    Delete,
    CtrlL,
    BackwardChar,
    ForwardChar,
}

fn bindings() -> Vec<(Cow<'static, [u8]>, Keybind)> {
    vec![
        (Cow::Borrowed(b"\r"), Keybind::Enter),
        (Cow::Borrowed(b"\x7f"), Keybind::Delete),
        (Cow::Borrowed(b"\x0c"), Keybind::CtrlL),
        (Cow::Borrowed(b"\x1b[C"), Keybind::ForwardChar),
        (Cow::Borrowed(b"\x1b[D"), Keybind::BackwardChar),
    ]
}
