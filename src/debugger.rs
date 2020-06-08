use anyhow::{Error, Result};
use nix::{sys::ptrace, unistd::Pid};
use rustyline::{error::ReadlineError, Editor};

pub struct Debugger {
    child_pid: Pid,
    editor: Editor<()>,
}

impl Debugger {
    pub fn new(pid: Pid) -> Self {
        Debugger {
            child_pid: pid,
            editor: Editor::<()>::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.editor.readline("derustify> ") {
                Ok(cmd) => {
                    let cleaned_cmd = cmd.trim();
                    match cleaned_cmd {
                        "continue" => return self.handle_continue().map_err(Error::new),
                        _ => {
                            println!("{}", "invalid command");
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {}
                Err(ReadlineError::Eof) => {}
                Err(e) => return Err(Error::new(e)),
            }
        }
    }

    pub fn handle_continue(&mut self) -> Result<(), nix::Error> {
        println!("Continuing debugee execution");
        ptrace::cont(self.child_pid, None)
    }
}
