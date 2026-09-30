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

fn parse_usize(s: &str) -> Option<usize> {
    let mut v = 0usize;
    for b in s.bytes() {
        let d = (b as char).to_digit(10)?;
        v = v.checked_mul(10)?.checked_add(d as usize)?;
    }
    Some(v)
}

pub fn command(args: &[&str]) {
    if args.len() != 2 {
        shell_println!("Usage: truncate <file> <size>");
        return;
    }
    let mut buf = [0u8; 256];
    let Some(cpath) = console::nul_into(&mut buf, args[0]) else {
        shell_println!("truncate: path too long");
        return;
    };
    let Some(size) = parse_usize(args[1]) else {
        shell_println!("Invalid size value");
        return;
    };
    let fd = unsafe { fsutil::open(cpath, libc::O_WRONLY, 0) };
    if fd < 0 {
        shell_println!("Unable to open file '%s'", cpath);
        return;
    }
    unsafe {
        fsutil::ftruncate(fd, size as libc::off_t);
        fsutil::close(fd);
    }
}
