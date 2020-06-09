use anyhow::{Context, Result};
use nix::{sys::ptrace, unistd::Pid};
use rustyline::{error::ReadlineError, Editor};

use crate::debugger::{Breakpoint, CommandHandler, PtraceSender};

struct DebuggerState {
    breakpoints: Vec<Breakpoint>,
}

pub struct Debugger {
    pid: Pid,
    state: DebuggerState,
    editor: Editor<()>,
}

impl Debugger {
    pub fn new(pid: Pid) -> Self {
        Debugger {
            pid,
            editor: Editor::<()>::new(),
            state: DebuggerState {
                breakpoints: Vec::new(),
            },
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.editor.readline("derustify> ") {
                Ok(cmd) => {
                    let cleaned_cmd = cmd.trim();
                    match cleaned_cmd {
                        "continue" => return self.handle_continue(),
                        c => {
                            if c.starts_with("b set") {
                                let args: Vec<_> = c.split(" ").collect();
                                let addr = args[2].parse::<u64>()?;
                                self.handle_add_bp(addr)?;
                            } else {
                                println!("{}", "invalid command");
                            }
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {}
                Err(ReadlineError::Eof) => {}
                Err(e) => return Err(e).with_context(|| "could not read user input"),
            }
        }
    }

    fn handle_continue(&mut self) -> Result<()> {
        ptrace::cont(self.pid, None).with_context(|| "could not continue tracing")
    }

    fn handle_add_bp(&mut self, addr: u64) -> Result<()> {
        let bp = Breakpoint::new(addr, PtraceSender::new(self.pid))
            .with_context(|| format!("failed to add breakpoint for address {:X?}", addr))?;
        self.state.breakpoints.push(bp);
        Ok(())
    }
}
