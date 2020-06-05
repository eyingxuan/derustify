use nix::unistd::{execvp, fork, ForkResult};
use std::{env::args, error::Error, ffi::CString};

// TODO: use failure crate?
fn main() -> Result<(), Box<dyn Error>> {
    // TODO: use structops to cleanup
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        println!("{}", "target program not specified");
        return Ok(());
    }
    let prog = CString::new(args[1].clone())?;

    match fork() {
        Ok(ForkResult::Parent { child }) => {}
        Ok(ForkResult::Child) => {
            execvp(prog.as_c_str(), &[prog.as_c_str()])?;
        }
        Err(_) => {}
    }

    Ok(())
}
