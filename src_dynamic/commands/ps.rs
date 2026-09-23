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

use core::ffi::c_char;

use crate::{console, fsutil, shell_println};

/// Read the first line of /proc/<pid>/status into `out`.
fn read_status_line(pid: u32, out: &mut [u8]) -> isize {
    let mut path = [0u8; 64];
    let prefix = b"/proc/";
    path[..prefix.len()].copy_from_slice(prefix);
    let mut n = prefix.len();
    let mut digits = [0u8; 12];
    let digits = format_u32(pid, &mut digits);
    path[n..n + digits.len()].copy_from_slice(digits);
    n += digits.len();
    let suffix = b"/status";
    path[n..n + suffix.len()].copy_from_slice(suffix);
    n += suffix.len();
    path[n] = 0;
    let fd = unsafe { fsutil::open(path.as_ptr() as *const c_char, 0, 0) };
    if fd < 0 {
        return -1;
    }
    let n = fsutil::read_all(fd, out);
    unsafe { fsutil::close(fd) };
    n
}

/// Decimal digits of `v` into `buf` (front-aligned).
fn format_u32<'a>(v: u32, buf: &'a mut [u8]) -> &'a [u8] {
    let mut tmp = [0u8; 12];
    let mut i = tmp.len();
    if v == 0 {
        i -= 1;
        tmp[i] = b'0';
    }
    let mut v = v;
    while v > 0 && i > 0 {
        i -= 1;
        tmp[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    let digits = &tmp[i..];
    buf[..digits.len()].copy_from_slice(digits);
    &buf[..digits.len()]
}

pub fn command(_args: &[&str]) {
    let tid_c = b"TID\0".as_ptr() as isize;
    let status_c = b"STATUS\0".as_ptr() as isize;
    let prio_c = b"PRIORITY\0".as_ptr() as isize;
    shell_println!("%-10s %-10s %-9s KIND", tid_c, status_c, prio_c);
    let dir = unsafe { fsutil::opendir(b"/proc\0".as_ptr() as *const c_char) };
    if dir.is_null() {
        shell_println!("Failed to read /proc");
        return;
    }
    loop {
        let dent = unsafe { fsutil::readdir(dir) };
        if dent.is_null() {
            break;
        }
        let name = unsafe { &*dent }.d_name.as_ptr() as *const c_char;
        let mut name_buf = [0u8; 16];
        let name_bytes = fsutil::cstr_copy(&mut name_buf, name);
        let Ok(name_str) = core::str::from_utf8(name_bytes) else {
            continue;
        };
        let Ok(pid) = name_str.parse::<u32>() else {
            continue;
        };
        let mut status = [0u8; 128];
        let n = read_status_line(pid, &mut status);
        if n <= 0 {
            continue;
        }
        let first = status.split(|&b| b == b'\n').next().unwrap_or(&[]);
        let mut line = [0u8; 128];
        let take = first.len().min(line.len());
        line[..take].copy_from_slice(&first[..take]);
        let mut line_buf = [0u8; 128];
        let line_c = console::nul_into(
            &mut line_buf,
            core::str::from_utf8(&line[..take]).unwrap_or(""),
        )
        .unwrap_or(core::ptr::null());
        let mut pid_buf = [0u8; 16];
        let pid_c = console::nul_into(&mut pid_buf, name_str).unwrap_or(core::ptr::null());
        let dash = b"-\0".as_ptr() as isize;
        shell_println!("%-10s %-10s %-9s %s", pid_c, dash, dash, line_c);
    }
    unsafe { fsutil::closedir(dir) };
}
