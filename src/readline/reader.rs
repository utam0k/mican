use std::borrow::Cow;
use std::io;
use std::os::unix::io::AsRawFd;

use nix;
use nix::sys::termios::{tcgetattr, tcsetattr, SetArg, LocalFlags, InputFlags,
                        SpecialCharacterIndices};
use nix::sys::select::{select, FdSet};
use nix::unistd::read;

use readline::event::Kind as EventKind;
use readline::event::Event;
use readline::context::Context;
use readline::signal;

pub struct Reader {
    context: Context,
    /// Key bindings.
    bindings: Vec<(Cow<'static, [u8]>, EventKind)>,
}

impl Reader {
    pub fn new(con: Context) -> Self {
        settings_term();
        Self {
            bindings: bindings(),
            context: con,
        }
    }

    /// Interactively reads a line from `stdin`.
    /// When an interrupt intervened, return None.
    pub fn read_line(&mut self) -> Option<String> {
        self.context.editor.write_prompt();
        self.context.editor.display().unwrap();

        signal::prepare().unwrap();

        loop {
            // Received a something signal.
            if let Some(_sig) = signal::take() {
                let e = Event::from_event_kind(&Some(EventKind::Interrupt));
                if let Ok(Some(line)) = (e.handler)(&mut self.context, Vec::new()) {
                    self.context.editor.display().unwrap();
                    return Some(line);
                }
                self.context.editor.display().unwrap();
                return None;
            }

            if wait_input() {
                let mut ch: Vec<u8> = Vec::new();
                let _ = self.read_char(&mut ch).unwrap();
                let res = self.find_bind(&ch);
                let e = Event::from_event_kind(&res);
                if let Ok(Some(line)) = (e.handler)(&mut self.context, ch) {
                    self.context.editor.display().unwrap();
                    return Some(line);
                }
                self.context.editor.display().unwrap();
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
    let mut tio = tcgetattr(stdin_fileno).unwrap();

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
