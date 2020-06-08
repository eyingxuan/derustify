use anyhow::{Error, Result};
use anyhow::{Context, Result};
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
                        "continue" => return self.handle_continue(),
                        _ => {
                            println!("{}", "invalid command");
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {}
                Err(ReadlineError::Eof) => {}
                Err(e) => return Err(e).with_context(|| "could not read user input"),
            }
        }
    }

    pub fn handle_continue(&mut self) -> Result<()> {
        println!("Continuing debugee execution");
        ptrace::cont(self.pid, None).with_context(|| "could not continue tracing")
    }
}
