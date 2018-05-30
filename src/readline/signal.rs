use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};

use nix::sys::signal;
use nix::libc::c_int;
use nix::sys::signal::Signal as NixSignal;

#[derive(Debug)]
pub enum Signal {
    Interrupt,
    Suspend,
    Continue,
    Quit,
}

static LAST_SIGNAL: AtomicUsize = AtomicUsize::new(0);

#[cfg_attr(feature = "cargo-clippy", allow(cast_sign_loss))]
extern "C" fn handle_sigint(sig: i32) {
    set_raw_signal(sig as usize);
}

fn set_raw_signal(sig: usize) {
    LAST_SIGNAL.store(sig, Ordering::Relaxed);
}

pub fn take() -> Option<Signal> {
    take_last_signal()
}

fn take_last_signal() -> Option<Signal> {
    conv_signal(LAST_SIGNAL.swap(!0, Ordering::Relaxed))
}

#[cfg_attr(feature = "cargo-clippy", allow(cast_possible_wrap, cast_possible_truncation))]
fn conv_signal(n: usize) -> Option<Signal> {
    if n == !0 {
        None
    } else {
        match NixSignal::from_c_int(n as c_int).ok() {
            Some(NixSignal::SIGINT) => Some(Signal::Interrupt),
            Some(NixSignal::SIGTSTP) => Some(Signal::Suspend),
            Some(NixSignal::SIGCONT) => Some(Signal::Continue),
            Some(NixSignal::SIGQUIT) => Some(Signal::Quit),
            _ => None,
        }
    }
}

pub fn prepare() -> io::Result<()> {
    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(handle_sigint),
        signal::SaFlags::empty(),
        signal::SigSet::empty(),
    );

    let _ = unsafe { signal::sigaction(NixSignal::SIGINT, &sig_action).unwrap() };
    let _ = unsafe { signal::sigaction(NixSignal::SIGTSTP, &sig_action).unwrap() };
    let _ = unsafe { signal::sigaction(NixSignal::SIGCONT, &sig_action).unwrap() };
    let _ = unsafe { signal::sigaction(NixSignal::SIGQUIT, &sig_action).unwrap() };

    Ok(())
}
