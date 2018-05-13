use std::io;

trait Cursor {
    fn move_up(dst: impl io::Write, n: usize) -> io::Result<()>;

    fn move_down(dst: impl io::Write, n: usize) -> io::Result<()>;

    fn move_left(dst: impl io::Write, n: usize) -> io::Result<()>;

    fn move_right(dst: impl io::Write, n: usize) -> io::Result<()>;
}

pub mod unix_cursor {
    use std::io;
    use std::io::Write;
    use std::mem::zeroed;
    use nix::libc::{c_int, c_ushort, ioctl, TIOCGWINSZ};

    pub fn move_to(n: usize) -> io::Result<()> {
        write(&format!("\x1b[{}G", n))
    }

    pub fn move_up(n: usize) -> io::Result<()> {
        write(&format!("\x1b[{}A", n))
    }

    pub fn move_down(n: usize) -> io::Result<()> {
        write(&format!("\x1b[{}B", n))
    }

    pub fn move_right(n: usize) -> io::Result<()> {
        write(&format!("\x1b[{}C", n))
    }

    pub fn move_left(n: usize) -> io::Result<()> {
        write(&format!("\x1b[{}D", n))
    }

    pub fn move_under_line_first(n: usize) -> io::Result<()> {
        write(&format!("\x1b[{}E", n))
    }

    pub fn clear_to_screen_end() -> io::Result<()> {
        write("\x1b[0J")
    }

    fn write(s: &str) -> io::Result<()> {
        let stdout = io::stdout();
        let mut lock = stdout.lock();

        lock.write_all(s.as_bytes())?;
        lock.flush()
    }

    pub fn get_winsize(fd: c_int) -> io::Result<Winsize> {
        let mut winsz: Winsize = unsafe { zeroed() };

        let res = unsafe { ioctl(fd, TIOCGWINSZ.into(), &mut winsz) };

        if res == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(winsz)
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct Winsize {
        pub ws_row: c_ushort,
        pub ws_col: c_ushort,
        pub ws_xpixel: c_ushort,
        pub ws_ypixel: c_ushort,
    }
}
