pub fn light_blue(s: &str) -> String {
    format!("\x1B[44m{}\x1B[m", s)
}

pub fn dark_blue(s: &str) -> String {
    format!("\x1B[48;5;24m{}\x1B[m", s)
}

pub fn gray(s: &str) -> String {
    format!("\x1B[48;5;240m{}\x1B[m", s)
}

pub fn white(s: &str) -> String {
    format!("\x1B[7m{}\x1B[m", s)
}
