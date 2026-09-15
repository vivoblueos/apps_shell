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

pub fn command(args: &[&str]) {
    if let Some(pos) = args.iter().position(|&x| x == ">") {
        if pos + 1 >= args.len() {
            shell_println!("Missing filename after '>'");
            return;
        }
        let mut path = [0u8; 256];
        let Some(cpath) = console::nul_into(&mut path, args[pos + 1]) else {
            shell_println!("echo: path too long");
            return;
        };
        let fd =
            unsafe { fsutil::open(cpath, libc::O_CREAT | libc::O_WRONLY | libc::O_TRUNC, 0o644) };
        if fd < 0 {
            shell_println!("Failed to create file");
            return;
        }
        // Join the words with single spaces into a fixed buffer.
        let mut line = [0u8; 256];
        let mut n = 0usize;
        for (i, word) in args[..pos].iter().enumerate() {
            if i > 0 && n + 1 < line.len() {
                line[n] = b' ';
                n += 1;
            }
            let take = word.len().min(line.len() - n - 1);
            line[n..n + take].copy_from_slice(&word.as_bytes()[..take]);
            n += take;
        }
        line[n] = b'\n';
        unsafe {
            fsutil::write(fd, line.as_ptr() as *const core::ffi::c_void, n + 1);
            fsutil::close(fd);
        }
    } else {
        for (i, word) in args.iter().enumerate() {
            if i > 0 {
                console::write_str(" ");
            }
            console::write_str(word);
        }
        console::write_str("\n");
    }
}
