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

use crate::{console, fsutil, shell_println};

fn parse_octal(s: &str) -> Option<u32> {
    if s.is_empty() {
        return None;
    }
    let mut v = 0u32;
    for b in s.bytes() {
        let d = (b as char).to_digit(8)?;
        v = v.checked_mul(8)?.checked_add(d)?;
    }
    Some(v)
}

pub fn command(args: &[&str]) {
    if args.is_empty() {
        shell_println!("Usage: mkdir [OPTION]... DIRECTORY...");
        return;
    }
    let mut parents = false;
    let mut verbose = false;
    let mut mode: Option<u32> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i] {
            "-p" | "--parents" => parents = true,
            "-v" | "--verbose" => verbose = true,
            "-m" | "--mode" => {
                i += 1;
                if i < args.len() {
                    match parse_octal(args[i]) {
                        Some(m) => mode = Some(m),
                        None => {
                            let mut mbuf = [0u8; 32];
                            let cmode =
                                console::nul_into(&mut mbuf, args[i]).unwrap_or(core::ptr::null());
                            shell_println!("invalid mode '%s'", cmode);
                            return;
                        }
                    }
                }
            }
            arg if arg.starts_with('-') => {
                let mut obuf = [0u8; 32];
                let copt = console::nul_into(&mut obuf, arg).unwrap_or(core::ptr::null());
                shell_println!("invalid option -- '%s'", copt);
                return;
            }
            dir => {
                let mut buf = [0u8; 256];
                let Some(cdir) = console::nul_into(&mut buf, dir) else {
                    shell_println!("mkdir: path too long");
                    continue;
                };
                let rc = unsafe { fsutil::mkdir(cdir, mode.unwrap_or(0o755) as core::ffi::c_int) };
                if rc != 0 {
                    if !parents {
                        shell_println!("cannot create directory '%s'", cdir);
                    }
                    continue;
                }
                if verbose {
                    shell_println!("created directory '%s'", cdir);
                }
            }
        }
        i += 1;
    }
}
