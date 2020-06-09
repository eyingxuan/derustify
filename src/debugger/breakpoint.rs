use super::sender::PtraceSender;
use anyhow::{Context, Result};
use nix::sys::ptrace::AddressType;

pub struct Breakpoint {
    addr: AddressType,
    enabled: bool,
    sender: PtraceSender,
    instruct_byte: Option<u64>,
}

impl Breakpoint {
    pub fn new(addr: u64, sender: PtraceSender) -> Result<Self> {
        let mut bp = Breakpoint {
            addr: addr as AddressType,
            enabled: false,
            instruct_byte: None,
            sender,
        };
        bp.enable_breakpoint()?;
        Ok(bp)
    }

    pub fn enable_breakpoint(&mut self) -> Result<()> {
        let curr_inst = self
            .sender
            .read_addr(self.addr)
            .with_context(|| format!("failed to enable breakpoint at {:X?}", self.addr))?
            as u64;
        self.instruct_byte = Some(curr_inst);
        // 0xCC is INT3 instruction
        self.sender
            .write_addr(self.addr, (curr_inst & 0xFF) | 0xCC)?;
        self.enabled = true;
        Ok(())
    }

    pub fn disable_breakpoint(&mut self) -> Result<()> {
        let inst = self
            .instruct_byte
            .expect("instruction must be cached when breakpoint is set");
        self.sender.write_addr(self.addr, inst)?;
        self.enabled = false;
        Ok(())
    }
}
