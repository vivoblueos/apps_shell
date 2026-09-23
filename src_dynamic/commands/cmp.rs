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
        shell_println!("Usage: cmp <path1> <path2>");
        return;
    }
    let mut p1 = [0u8; 256];
    let mut p2 = [0u8; 256];
    let (Some(c1), Some(c2)) = (
        console::nul_into(&mut p1, args[0]),
        console::nul_into(&mut p2, args[1]),
    ) else {
        shell_println!("cmp: path too long");
        return;
    };
    let fd1 = unsafe { fsutil::open(c1, 0, 0) };
    let fd2 = unsafe { fsutil::open(c2, 0, 0) };
    if fd1 < 0 {
        shell_println!("Failed to open '%s'", c1);
        return;
    }
    if fd2 < 0 {
        shell_println!("Failed to open '%s'", c2);
        return;
    }

    let mut diff_bytes = 0u64;
    let (mut total1, mut total2) = (0u64, 0u64);
    let (mut buf1, mut buf2) = ([0u8; 512], [0u8; 512]);
    loop {
        let n1 = unsafe { fsutil::read(fd1, buf1.as_mut_ptr() as *mut c_void, buf1.len()) };
        let n2 = unsafe { fsutil::read(fd2, buf2.as_mut_ptr() as *mut c_void, buf2.len()) };
        if n1 <= 0 && n2 <= 0 {
            break;
        }
        total1 += n1.max(0) as u64;
        total2 += n2.max(0) as u64;
        if n1 != n2 {
            break;
        }
        for i in 0..n1 as usize {
            if buf1[i] != buf2[i] {
                diff_bytes += 1;
            }
        }
    }

    if diff_bytes == 0 && total1 == total2 {
        shell_println!("Files are identical");
    } else if total1 == total2 {
        shell_println!("Found %u differing bytes", diff_bytes);
    } else {
        let len_diff = if total1 > total2 {
            total1 - total2
        } else {
            total2 - total1
        };
        shell_println!(
            "Files differ in length by %u bytes\nAdditionally found %u differing bytes",
            len_diff,
            diff_bytes
        );
    }
    unsafe {
        fsutil::close(fd1);
        fsutil::close(fd2);
    }
}
