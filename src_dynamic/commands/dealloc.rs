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

fn parse_hex(s: &str) -> Option<usize> {
    if s.is_empty() {
        return None;
    }
    let digits = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    let mut v = 0usize;
    for b in digits.bytes() {
        let d = (b as char).to_digit(16)?;
        v = v.checked_mul(16)?.checked_add(d as usize)?;
    }
    Some(v)
}

extern "C" {
    fn free(ptr: *mut c_void);
}

pub fn command(args: &[&str]) {
    if args.is_empty() {
        shell_println!("Usage: dealloc <ptr>");
        return;
    }
    let Some(ptr) = parse_hex(args[0]) else {
        shell_println!("Wrong format");
        return;
    };
    unsafe { free(ptr as *mut c_void) };
}
