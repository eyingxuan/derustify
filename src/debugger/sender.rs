use anyhow::{Context, Result};
use nix::{sys::ptrace, unistd::Pid};
use std::ffi::c_void;

#[derive(Copy, Clone)]
pub struct PtraceSender {
    pid: Pid,
}

impl PtraceSender {
    pub fn new(pid: Pid) -> Self {
        PtraceSender { pid }
    }

    pub fn write_addr(&self, addr: ptrace::AddressType, data: u64) -> Result<()> {
        let casted_data = data as *mut c_void;
        ptrace::write(self.pid, addr, casted_data)
            .with_context(|| format!("writing {:X?} to {:X?} failed", casted_data, addr))
    }

    pub fn read_addr(&self, addr: ptrace::AddressType) -> Result<i64> {
        ptrace::read(self.pid, addr).with_context(|| format!("reading from {:X?} failed", addr))
    }

    pub fn send_cont(&self) -> Result<()> {
        ptrace::cont(self.pid, None).with_context(|| "could not continue debugee")
    }
}
