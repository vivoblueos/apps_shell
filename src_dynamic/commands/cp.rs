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
    if args.len() != 2 {
        shell_println!("Usage: cp <source file> <destination file/dir>");
        return;
    }
    let mut src = [0u8; 256];
    let mut dst = [0u8; 256];
    let Some(csrc) = console::nul_into(&mut src, args[0]) else {
        shell_println!("cp: source path too long");
        return;
    };
    // If the destination names a directory, append the source basename.
    let dst_bytes = if let Some(cdst) = console::nul_into(&mut dst, args[1]) {
        let mut st = core::mem::MaybeUninit::<libc::stat>::zeroed();
        let is_dir = unsafe { fsutil::stat(cdst, st.as_mut_ptr()) } == 0
            && (unsafe { st.assume_init() }.st_mode & libc::S_IFMT) == libc::S_IFDIR;
        if is_dir {
            let base = args[0].rsplit('/').next().unwrap_or(args[0]);
            let mut joined = [0u8; 256];
            let n = args[1].len();
            if n + 1 + base.len() + 1 > joined.len() {
                shell_println!("cp: destination path too long");
                return;
            }
            joined[..n].copy_from_slice(args[1].as_bytes());
            joined[n] = b'/';
            joined[n + 1..n + 1 + base.len()].copy_from_slice(base.as_bytes());
            joined[n + 1 + base.len()] = 0;
            joined.as_ptr() as *const core::ffi::c_char
        } else {
            cdst
        }
    } else {
        shell_println!("cp: destination path too long");
        return;
    };

    let fd_in = unsafe { fsutil::open(csrc, 0, 0) };
    if fd_in < 0 {
        shell_println!("Failed to open source file '%s'", csrc);
        return;
    }
    let fd_out = unsafe {
        fsutil::open(
            dst_bytes,
            libc::O_CREAT | libc::O_WRONLY | libc::O_TRUNC,
            0o644,
        )
    };
    if fd_out < 0 {
        shell_println!("Failed to create destination file");
        unsafe { fsutil::close(fd_in) };
        return;
    }
    let mut buf = [0u8; 512];
    loop {
        let n = unsafe { fsutil::read(fd_in, buf.as_mut_ptr() as *mut c_void, buf.len()) };
        if n <= 0 {
            break;
        }
        unsafe {
            fsutil::write(fd_out, buf.as_ptr() as *const c_void, n as usize);
        }
    }
    unsafe {
        fsutil::close(fd_in);
        fsutil::close(fd_out);
    }
}
