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

use crate::{fsutil, shell_println};

pub fn command(_args: &[&str]) {
    let total_c = b"total\0".as_ptr() as isize;
    let used_c = b"used\0".as_ptr() as isize;
    let free_c = b"free\0".as_ptr() as isize;
    shell_println!("%-12s %-12s %-12s", total_c, used_c, free_c);
    let fd = unsafe {
        fsutil::open(
            b"/proc/meminfo\0".as_ptr() as *const core::ffi::c_char,
            0,
            0,
        )
    };
    if fd < 0 {
        shell_println!("Failed to read /proc/meminfo");
        return;
    }
    let mut buf = [0u8; 512];
    let n = fsutil::read_all(fd, &mut buf);
    unsafe { fsutil::close(fd) };
    let content = core::str::from_utf8(&buf[..n.max(0) as usize]).unwrap_or("");

    let mut total = "0";
    let mut used = "0";
    let mut free = "0";
    for line in content.split('\n') {
        if let Some((key, value)) = line.split_once(':') {
            let value = value.split_whitespace().next().unwrap_or("");
            match key.trim() {
                "MemTotal" => total = value,
                "MemUsed" => used = value,
                "MemAvailable" => free = value,
                _ => {}
            }
        }
    }
    // Print through the shared libc printf; values are plain digit strings.
    let mut tb = [0u8; 32];
    let mut ub = [0u8; 32];
    let mut fb = [0u8; 32];
    let (Some(ct), Some(cu), Some(cf)) = (
        crate::console::nul_into(&mut tb, total),
        crate::console::nul_into(&mut ub, used),
        crate::console::nul_into(&mut fb, free),
    ) else {
        return;
    };
    shell_println!("%-12s %-12s %-12s", ct, cu, cf);
}
