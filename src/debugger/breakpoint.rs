use super::Debugger;
use anyhow::Result;
use nix::sys::ptrace::AddressType;

pub struct Breakpoint {
    addr: AddressType,
    enabled: bool,
    instruct_byte: Option<u64>,
}

impl Breakpoint {
    pub fn new(addr: u64, debugger: &mut Debugger) -> Result<Self> {
        let mut bp = Breakpoint {
            addr: unsafe { addr as AddressType },
            enabled: false,
            instruct_byte: None,
        };
        bp.set_breakpoint(debugger)?;
        Ok(bp)
    }

    pub fn set_breakpoint(&mut self, debugger: &mut Debugger) -> Result<()> {
        let curr_inst = debugger.read_addr(self.addr)? as u64;
        self.instruct_byte = Some(curr_inst);
        // 0xCC is INT3 instruction
        debugger.write_addr(self.addr, curr_inst | 0xCC)?;
        Ok(())
    }

    pub fn rem_breakpoint(&mut self, debugger: &mut Debugger) -> Result<()> {
        let inst = self
            .instruct_byte
            .expect("instruction must be cached when breakpoint is set");
        debugger.write_addr(self.addr, inst)?;
        Ok(())
    }
}
