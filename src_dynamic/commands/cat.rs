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

use core::ffi::c_void;

use crate::{console, fsutil, shell_println};

pub fn command(args: &[&str]) {
    if args.is_empty() {
        shell_println!("Usage: cat [<path> [<path> ...]]");
        return;
    }
    for filename in args {
        let mut path = [0u8; 256];
        let Some(cpath) = console::nul_into(&mut path, filename) else {
            shell_println!("cat: path too long");
            continue;
        };
        let fd = unsafe {
            fsutil::open(cpath, 0 /* O_RDONLY */, 0)
        };
        if fd < 0 {
            shell_println!("unable to open file '%s'", cpath);
            continue;
        }
        let mut buf = [0u8; 512];
        loop {
            let n = unsafe { fsutil::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) };
            if n <= 0 {
                break;
            }
            console::write_stdout(&buf[..n as usize]);
        }
        unsafe { fsutil::close(fd) };
    }
}
