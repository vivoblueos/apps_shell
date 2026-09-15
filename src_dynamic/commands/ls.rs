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

use core::ffi::{c_char, c_void};

use crate::{console, fsutil, shell_println};

pub fn command(args: &[&str]) {
    let mut show_hidden = false;
    let mut long_format = false;
    let mut target = ".";

    for arg in args {
        match *arg {
            "-a" => show_hidden = true,
            "-l" => long_format = true,
            "-la" | "-al" => {
                show_hidden = true;
                long_format = true;
            }
            path if !path.starts_with('-') => target = path,
            _ => {
                let mut opt = [0u8; 32];
                let copt = console::nul_into(&mut opt, arg).unwrap_or(core::ptr::null());
                shell_println!("Unknown option: %s", copt);
                return;
            }
        }
    }

    let mut tbuf = [0u8; 256];
    let Some(ctarget) = console::nul_into(&mut tbuf, target) else {
        shell_println!("ls: path too long");
        return;
    };
    let dir = unsafe { fsutil::opendir(ctarget) };
    if dir.is_null() {
        shell_println!("Directory does not exist: %s", ctarget);
        return;
    }

    // The dynamic shell avoids allocation, so entries remain in readdir order.
    loop {
        let dent = unsafe { fsutil::readdir(dir) };
        if dent.is_null() {
            break;
        }
        let dent = unsafe { &*dent };
        let name = dent.d_name.as_ptr() as *const c_char;
        let mut name_buf = [0u8; 256];
        let name_bytes = fsutil::cstr_copy(&mut name_buf, name);
        let Ok(name_str) = core::str::from_utf8(name_bytes) else {
            continue;
        };
        if name_str == "." || name_str == ".." {
            continue;
        }
        if !show_hidden && name_str.starts_with('.') {
            continue;
        }

        if long_format {
            // Build "<dir>/<name>" for the stat call.
            let mut full = [0u8; 512];
            let tlen = target.len();
            if tlen + 1 + name_str.len() + 1 > full.len() {
                continue;
            }
            full[..tlen].copy_from_slice(target.as_bytes());
            full[tlen] = b'/';
            full[tlen + 1..tlen + 1 + name_str.len()].copy_from_slice(name_str.as_bytes());
            full[tlen + 1 + name_str.len()] = 0;
            let mut st = core::mem::MaybeUninit::<libc::stat>::zeroed();
            let mode =
                if unsafe { fsutil::stat(full.as_ptr() as *const c_char, st.as_mut_ptr()) } == 0 {
                    unsafe { st.assume_init() }.st_mode
                } else {
                    0
                };
            let is_dir = (mode & libc::S_IFMT) == libc::S_IFDIR;
            let perm = mode & 0o777;
            shell_println!(
                "%c%c%c%c%c%c%c%c%c %s",
                if is_dir { b'd' as isize } else { b'-' as isize },
                if perm & 0o400 != 0 {
                    b'r' as isize
                } else {
                    b'-' as isize
                },
                if perm & 0o200 != 0 {
                    b'w' as isize
                } else {
                    b'-' as isize
                },
                if perm & 0o100 != 0 {
                    b'x' as isize
                } else {
                    b'-' as isize
                },
                if perm & 0o40 != 0 {
                    b'r' as isize
                } else {
                    b'-' as isize
                },
                if perm & 0o20 != 0 {
                    b'w' as isize
                } else {
                    b'-' as isize
                },
                if perm & 0o10 != 0 {
                    b'x' as isize
                } else {
                    b'-' as isize
                },
                if perm & 0o4 != 0 {
                    b'r' as isize
                } else {
                    b'-' as isize
                },
                if perm & 0o2 != 0 {
                    b'w' as isize
                } else {
                    b'-' as isize
                },
                name_buf.as_ptr()
            );
        } else if dent.d_type == libc::DT_DIR {
            shell_println!("%s/", name_buf.as_ptr());
        } else {
            shell_println!("%s", name_buf.as_ptr());
        }
    }
    unsafe { fsutil::closedir(dir) };
}
