use super::{breakpoint::Breakpoint, debugger::DebuggerState, sender::PtraceSender};
use anyhow::{Context, Result};

pub struct CommandHandler {
    sender: PtraceSender,
}

impl CommandHandler {
    pub fn new(sender: PtraceSender) -> Self {
        CommandHandler { sender }
    }

    pub fn run_command(&self, cmd: String, state: &mut DebuggerState) -> Result<()> {
        if cmd == "" {
            return Ok(());
        }
        let args: Vec<_> = cmd.trim().split(" ").collect();
        match args[0] {
            "continue" => self.handle_continue()?,
            "b" => self.handle_break(&args[1..], state)?,
            _ => println!("invalid command"),
        };
        Ok(())
    }

    fn handle_continue(&self) -> Result<()> {
        self.sender.send_cont()
    }

    fn handle_break(&self, args: &[&str], state: &mut DebuggerState) -> Result<()> {
        if args.len() != 2 {
            println!("incorrect number of arguments passed");
        } else {
            if args[0] == "set" {
                let addr = args[1].parse::<u64>()?;
                let bp = Breakpoint::new(addr, self.sender).with_context(|| {
                    format!("failed to create breakpoint for address {:X?}", addr)
                })?;
                state.breakpoints.push(bp);
            } else {
                println!("invalid subcommand")
            }
        }

        Ok(())
    }
}
