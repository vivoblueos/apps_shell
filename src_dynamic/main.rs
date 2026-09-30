// Copyright (c) 2026 vivo Mobile Communication Co., Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The bootstrap shell as a dynamic application.
//!
//! The shell is a C-style `no_std` PIE: `blueos_scrt1::_start` is its ELF
//! entry, which resolves this crate's `main` and tail-calls
//! `__librs_start_main(main, info)` from the shared `libc.so.1`. It has no
//! Rust `alloc` — every path and line lives in a fixed buffer and crosses the
//! DSO boundary as a NUL-terminated C string (see `console`/`fsutil`).

#![no_std]
#![no_main]
#![feature(c_variadic)]

use core::ffi::{c_char, c_int, c_void};

mod commands;
mod console;
mod fsutil;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const MAX_INPUT: usize = 256;
const MAX_ARGS: usize = 16;

/// The dynamic entry: run the shell loop on the application main thread.
/// Returning exits the application, which the kernel reaps.
///
#[no_mangle]
pub extern "C" fn main(
    _argc: c_int,
    _argv: *const *const c_char,
    _envp: *const *const c_char,
) -> c_int {
    shell_loop();
    0
}

fn shell_loop() {
    shell_println!("Hello, shell!");
    loop {
        // `printf` is line-buffered, while the tty echoes input independently.
        // Write the prompt directly so it is visible before blocking in read.
        console::write_str("> ");

        // Read one committed line from the console (fd 0). The kernel tty
        // commits canonical input on CR, so a returned line ends with '\n'.
        let mut input = [0u8; MAX_INPUT];
        let mut len = 0usize;
        while len + 1 < MAX_INPUT {
            let n = unsafe { fsutil::read(0, input[len..].as_mut_ptr() as *mut c_void, 1) };
            if n <= 0 {
                // EOF or error: retire the shell application.
                return;
            }
            if input[len] == b'\n' {
                break;
            }
            len += 1;
        }
        input[len] = 0;

        let line = core::str::from_utf8(&input[..len]).unwrap_or("");
        let line = line.trim_matches(|c| c == ' ' || c == '\r' || c == '\t');
        if line.is_empty() {
            continue;
        }

        let mut argv: [&str; MAX_ARGS] = [""; MAX_ARGS];
        let mut argc = 0usize;
        for token in line.split_whitespace() {
            if argc == MAX_ARGS {
                break;
            }
            argv[argc] = token;
            argc += 1;
        }
        let args = &argv[..argc];

        if args[0] == "exit" {
            break;
        }

        match commands::COMMANDS.get(args[0]) {
            Some(info) => (info.handler)(&args[1..]),
            None => {
                let mut buf = [0u8; 64];
                if let Some(cmd) = console::nul_into(&mut buf, args[0]) {
                    shell_println!("Unknown command: %s", cmd);
                }
            }
        }
    }
}
