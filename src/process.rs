use libc::{pid_t, fork, waitpid, c_int};

use std::process::exit;
use token::CommandData;

#[derive(Debug)]
pub struct Process {
    pub pid: pid_t,
    pub f: fn(CommandData) -> Result<(), String>,
}

impl Process {
    pub fn new(f: fn(CommandData) -> Result<(), String>) -> Process {
        Process {
            pid: unsafe { fork() },
            f: f,
        }
    }

    pub fn in_child(&self) -> bool {
        self.pid == 0
    }

    pub fn run(&self, cmd: CommandData) -> Result<(), String> {
        let result = (self.f)(cmd);
        self.exit();
        result
    }

    pub fn wait(&self) {
        let mut status: i32 = 0;
        unsafe {
            waitpid(self.pid, &mut status as *mut c_int, 0);
        }
    }

    fn exit(&self) {
        exit(self.pid)
    }
}
