use anyhow::Result;
use nix::{
    sys::{ptrace, wait::waitpid},
    unistd::{execvp, fork, ForkResult},
};
use std::{env::args, ffi::CString};

mod debugger;
use debugger::Debugger;

fn main() -> Result<()> {
    // TODO: use structops to cleanup
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        println!("{}", "target program not specified");
        return Ok(());
    }
    let prog = CString::new(args[1].clone())?;

    match fork()? {
        ForkResult::Parent { child } => {
            let mut debugger = Debugger::new(child);
            waitpid(child, None)?;
            debugger.run()?;
        }
        ForkResult::Child => {
            // TODO: allow users to pass in arguments
            ptrace::traceme()?;
            execvp(prog.as_c_str(), &[prog.as_c_str()])?;
        }
    }

    Ok(())
}
