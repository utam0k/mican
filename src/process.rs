use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, ForkResult, Pid};

use std::process::exit;
use token::CommandData;

pub struct Process {
    pub pid: Pid,
    pub f: fn(CommandData) -> Result<(), String>,
    fork_result: ForkResult,
}

impl Process {
    pub fn new(f: fn(CommandData) -> Result<(), String>) -> Process {
        let result = fork().unwrap();
        let pid = match result {
            ForkResult::Parent { child } => child,
            ForkResult::Child => getpid(),
        };
        Process {
            pid: pid,
            f: f,
            fork_result: result,
        }
    }

    pub fn in_child(&self) -> bool {
        self.fork_result.is_child()
    }

    pub fn run(&self, cmd: CommandData) -> Result<(), String> {
        let result = (self.f)(cmd);
        self.exit();
        result
    }

    pub fn wait(&self) {
        waitpid(self.pid, None).unwrap();
    }

    fn exit(&self) {
        exit(self.pid.into())
    }
}
