use std::io;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

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

static LAST_SIGNAL: AtomicUsize = ATOMIC_USIZE_INIT;

extern "C" fn handle_sigint(sig: i32) {
    set_raw_signal(sig as usize);
}

fn set_raw_signal(sig: usize) {
    LAST_SIGNAL.store(sig, Ordering::Relaxed);
}

pub fn take_signal() -> Option<Signal> {
    take_last_signal()
}

fn take_last_signal() -> Option<Signal> {
    conv_signal(LAST_SIGNAL.swap(!0, Ordering::Relaxed))
}

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
