use std::borrow::Cow;
use std::io;
use std::os::unix::io::AsRawFd;

use nix;
use nix::sys::termios::{tcgetattr, tcsetattr, SetArg, LocalFlags, InputFlags,
                        SpecialCharacterIndices};
use nix::sys::select::{select, FdSet};
use nix::unistd::read;

use readline::editor::{Editor, Complete};
use readline::history::History;
use readline::event::Kind as EventKind;

pub struct Reader {
    ed: Editor,
    history: History,
    bindings: Vec<(Cow<'static, [u8]>, EventKind)>,
}

impl Reader {
    pub fn new(prompt: String) -> Self {
        settings_term();
        Self {
            ed: Editor::new(prompt),
            history: History::new(),
            bindings: bindings(),
        }
    }

    pub fn read_line(&mut self) -> String {
        self.ed.write_prompt();
        self.ed.display().unwrap();
        loop {
            if wait_input() {
                let mut ch: Vec<u8> = Vec::new();
                let _ = self.read_char(&mut ch).unwrap();
                let res = self.find_bind(&ch);
                if let Ok(Some(line)) = self.execute_sequence(&res, ch) {
                    self.ed.display().unwrap();
                    return line;
                }
                self.ed.display().unwrap();
            }
        }
    }

    fn execute_sequence(
        &mut self,
        res: &Option<EventKind>,
        ch: Vec<u8>,
    ) -> io::Result<Option<String>> {
        match res {
            Some(EventKind::Complete) => {
                if !self.ed.line.trim().len() == self.ed.line.len() {
                    // TODO
                    // self.ed.put("\t".into())?;
                    return Ok(None);
                } else {
                    self.ed.complete();
                    self.ed.completion_next();
                    self.ed.completion_disply();
                }
                Ok(None)
            }
            Some(EventKind::Enter) => {
                let result = self.ed.line.clone();
                self.ed.completion_clear();
                self.ed.reset();
                self.ed.new_line();
                self.history.push(result.clone());
                self.history.reset();
                Ok(Some(result))
            }
            Some(EventKind::CtrlL) => {
                self.ed.clear_screen();
                self.ed.write_line();
                Ok(None)
            }
            Some(EventKind::Delete) => {
                self.ed.completion_clear();
                self.ed.delete(1);
                Ok(None)
            }
            Some(EventKind::ForwardChar) => {
                self.ed.move_right(1);
                Ok(None)
            }
            Some(EventKind::BackwardChar) => {
                self.ed.move_left(1);
                Ok(None)
            }
            Some(EventKind::PreviousHistory) => {
                self.ed.completion_clear();
                if self.history.is_started() {
                    self.history.set_first(self.ed.line.clone());
                }
                let history = match self.history.prev() {
                    Some(h) => h,
                    None => return Ok(None),
                };
                self.ed.replace(history);
                self.ed.move_to_end();
                Ok(None)
            }
            Some(EventKind::NextHistory) => {
                self.ed.completion_clear();
                let history = match self.history.next() {
                    Some(h) => h,
                    None => return Ok(None),
                };
                self.ed.replace(history);
                self.ed.move_to_end();
                Ok(None)
            }
            Some(EventKind::BeginningOFLine) => {
                self.ed.move_to_first();
                Ok(None)
            }
            Some(EventKind::EndOfLine) => {
                self.ed.move_to_end();
                Ok(None)
            }
            Some(EventKind::Something) => Ok(None),
            None => {
                self.ed.completion_clear();

                self.ed.put(&String::from_utf8(ch).unwrap());
                self.history.reset_first();
                Ok(None)
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
            let result = retry(|| read(io::stdout().as_raw_fd(), &mut buf[len..]));

            buf.set_len(len);

            n = result?;
            buf.set_len(len + n);
        }

        Ok(n)
    }

    fn find_bind(&self, ch: &[u8]) -> Option<EventKind> {
        for (ref bind, ref cmd) in &self.bindings {
            if &Cow::Borrowed(ch) == bind {
                return Some(cmd.clone());
            }
        }
        None
    }
}

fn settings_term() {
    let stdin_fileno = io::stdout().as_raw_fd();
    let old_tio = tcgetattr(stdin_fileno).unwrap();
    let mut tio = old_tio;

    tio.input_flags.remove(
        InputFlags::INLCR | InputFlags::ICRNL,
    );
    tio.local_flags.remove(
        LocalFlags::ICANON | LocalFlags::ECHO,
    );
    tio.control_chars[SpecialCharacterIndices::VMIN as usize] = 0;
    tio.control_chars[SpecialCharacterIndices::VTIME as usize] = 0;

    tcsetattr(stdin_fileno, SetArg::TCSANOW, &tio).unwrap();
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

#[cfg_attr(feature = "cargo-clippy", allow(never_loop))]
fn wait_input() -> bool {
    let stdin_fileno = io::stdout().as_raw_fd();
    let mut r_fds = FdSet::new();
    r_fds.insert(stdin_fileno);

    let mut e_fds = FdSet::new();
    r_fds.insert(stdin_fileno);

    loop {
        match select(
            stdin_fileno + 1,
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

fn bindings() -> Vec<(Cow<'static, [u8]>, EventKind)> {
    vec![
        (Cow::Borrowed(b"\r"      ), EventKind::Enter),           // Enter
        (Cow::Borrowed(b"\x7f"    ), EventKind::Delete),          // BackSpace
        (Cow::Borrowed(b"\x1b[A"  ), EventKind::PreviousHistory), // Up
        (Cow::Borrowed(b"\x1b[B"  ), EventKind::NextHistory),     // Down
        (Cow::Borrowed(b"\x1b[C"  ), EventKind::ForwardChar),     // Left
        (Cow::Borrowed(b"\x1b[D"  ), EventKind::BackwardChar),    // Right

        (Cow::Borrowed(b"\t"      ), EventKind::Complete),        // Tab

        (Cow::Borrowed(b"\x01"    ), EventKind::BeginningOFLine), // Ctrl-A
        (Cow::Borrowed(b"\x02"    ), EventKind::BackwardChar),    // Ctrl-B
        (Cow::Borrowed(b"\x05"    ), EventKind::EndOfLine),       // Ctrl-E
        (Cow::Borrowed(b"\x06"    ), EventKind::ForwardChar),     // Ctrl-F
        (Cow::Borrowed(b"\x07"    ), EventKind::Something),       // Ctrl-G
        (Cow::Borrowed(b"\x0a"    ), EventKind::Enter),           // Ctrl-J
        (Cow::Borrowed(b"\x0b"    ), EventKind::Something),       // Ctrl-K
        (Cow::Borrowed(b"\x0c"    ), EventKind::CtrlL),           // Ctrl-L
        (Cow::Borrowed(b"\x0d"    ), EventKind::Enter),           // Ctrl-N
        (Cow::Borrowed(b"\x0e"    ), EventKind::NextHistory),     // Ctrl-N
        (Cow::Borrowed(b"\x10"    ), EventKind::PreviousHistory), // Ctrl-P
        (Cow::Borrowed(b"\x12"    ), EventKind::Something),       // Ctrl-R
        (Cow::Borrowed(b"\x14"    ), EventKind::Something),       // Ctrl-T
        (Cow::Borrowed(b"\x19"    ), EventKind::Something),       // Ctrl-Y
        (Cow::Borrowed(b"\x1d"    ), EventKind::Something),       // Ctrl-]
        (Cow::Borrowed(b"\x1b\x08"), EventKind::Something),       // Escape, Ctrl-H
        (Cow::Borrowed(b"\x1b\x1d"), EventKind::Something),       // Escape, Ctrl-]
        (Cow::Borrowed(b"\x1b\x7f"), EventKind::Something),       // Escape, Rubout
        (Cow::Borrowed(b"\x1bb"   ), EventKind::Something),       // Escape, b
        (Cow::Borrowed(b"\x1bd"   ), EventKind::Something),       // Escape, d
        (Cow::Borrowed(b"\x1bf"   ), EventKind::Something),       // Escape, f
        (Cow::Borrowed(b"\x1bt"   ), EventKind::Something),       // Escape, t
        (Cow::Borrowed(b"\x1by"   ), EventKind::Something),       // Escape, y
        (Cow::Borrowed(b"\x1b#"   ), EventKind::Something),       // Escape, #
        (Cow::Borrowed(b"\x1b<"   ), EventKind::Something),       // Escape, <
        (Cow::Borrowed(b"\x1b>"   ), EventKind::Something),       // Escape, >
    ]
}
