pub fn blue(s: &str) -> String {
    format!("\x1B[44m{}\x1B[m", s)
}

pub fn gray(s: &str) -> String {
    format!("\x1B[48;5;240m{}\x1B[m", s)
}
