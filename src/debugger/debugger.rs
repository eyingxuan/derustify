use super::{breakpoint::Breakpoint, command_handler::CommandHandler, sender::PtraceSender};
use anyhow::{Context, Result};
use nix::{
    sys::wait::{waitpid, WaitStatus},
    unistd::Pid,
};
use rustyline::{error::ReadlineError, Editor};

pub struct DebuggerState {
    pub breakpoints: Vec<Breakpoint>,
}

pub struct Debugger {
    pid: Pid,
    state: DebuggerState,
    editor: Editor<()>,
    cmdhandler: CommandHandler,
}

impl Debugger {
    pub fn new(pid: Pid) -> Self {
        Debugger {
            pid,
            editor: Editor::<()>::new(),
            state: DebuggerState {
                breakpoints: Vec::new(),
            },
            cmdhandler: CommandHandler::new(PtraceSender::new(pid)),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.editor.readline("derustify> ") {
                Ok(cmd) => self.cmdhandler.run_command(cmd, &mut self.state)?,
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(e) => return Err(e).with_context(|| "could not read user input"),
            };
            match waitpid(self.pid, None)? {
                WaitStatus::Exited(_, exit_code) => {
                    println! {"child exited with exit code {}", exit_code};
                    break;
                }
                _ => {
                    continue;
                }
            }
        }

        Ok(())
    }
}
