# Derustify

Getting inspiration from [Writing a Linux Debugger](https://blog.tartanllama.xyz/writing-a-linux-debugger-setup/), 
which was implemented in C++, I've started to implement a Linux debugger
to get some more practice with writing Rust, using crates and reading 
documentations.

Features I plan to implement include 
- Interacting with DWARF using the Gimli crate to implement
  source-level stepping/breakpoints
- Stack, register, and memory inspection
- Proper error handling with Anyhow
- GUI in terminal?
