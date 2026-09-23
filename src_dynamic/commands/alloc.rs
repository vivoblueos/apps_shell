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

use crate::shell_println;

fn parse_usize(s: &str) -> Option<usize> {
    let mut v = 0usize;
    for b in s.bytes() {
        let d = (b as char).to_digit(10)?;
        v = v.checked_mul(10)?.checked_add(d as usize)?;
    }
    Some(v)
}

extern "C" {
    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> i32;
}

pub fn command(args: &[&str]) {
    if args.len() < 2 {
        shell_println!("Usage: alloc <size> <align>");
        return;
    }
    let (Some(size), Some(align)) = (parse_usize(args[0]), parse_usize(args[1])) else {
        shell_println!("Wrong format");
        return;
    };
    let mut result: *mut c_void = core::ptr::null_mut();
    let rc = unsafe { posix_memalign(&mut result, align, size) };
    if rc != 0 || result.is_null() {
        shell_println!("Unable to allocate memory");
    } else {
        shell_println!("0x%x", result as usize);
    }
}
