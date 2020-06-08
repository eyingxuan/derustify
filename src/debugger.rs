use anyhow::{Context, Result};
use nix::{sys::ptrace, unistd::Pid};
use ptrace::AddressType;
use rustyline::{error::ReadlineError, Editor};
use std::ffi::c_void;

mod breakpoint;
use breakpoint::Breakpoint;

pub struct Debugger {
    pid: Pid,
    editor: Editor<()>,
    breakpoints: Vec<Breakpoint>,
}

impl Debugger {
    pub fn new(pid: Pid) -> Self {
        Debugger {
            pid,
            editor: Editor::<()>::new(),
            breakpoints: Vec::new(),
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

    pub fn write_addr(&mut self, addr: AddressType, data: u64) -> Result<()> {
        let casted_data = unsafe { data as *mut c_void };
        ptrace::write(self.pid, addr, casted_data)
            .with_context(|| format!("writing {:X?} to {:X?} failed", casted_data, addr))
    }

    pub fn read_addr(&self, addr: AddressType) -> Result<i64> {
        ptrace::read(self.pid, addr).with_context(|| format!("reading from {:X?} failed", addr))
    }

    pub fn handle_continue(&mut self) -> Result<()> {
        println!("Continuing debugee execution");
        ptrace::cont(self.pid, None).with_context(|| "could not continue tracing")
    }

    pub fn handle_add_bp(&mut self, addr: u64) -> Result<()> {
        let bp = Breakpoint::new(addr, self)?;
        self.breakpoints.push(bp);
        Ok(())
    }
}
