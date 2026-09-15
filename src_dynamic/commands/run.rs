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

use core::ffi::{c_char, c_long};

use crate::{console, fsutil, shell_println};

pub fn command(args: &[&str]) {
    if args.is_empty() {
        shell_println!("Usage: run <path> [arg...]");
        return;
    }

    // Build the POSIX argv shape (argv[0] = path, NUL-terminated) with fixed
    // buffers: up to 16 arguments, each at most 128 bytes.
    const MAX_ARGS: usize = 16;
    let mut storage: [[u8; 128]; MAX_ARGS] = [[0; 128]; MAX_ARGS];
    let mut ptrs: [*const c_char; MAX_ARGS + 1] = [core::ptr::null(); MAX_ARGS + 1];
    let count = args.len().min(MAX_ARGS);
    for i in 0..count {
        match console::nul_into(&mut storage[i], args[i]) {
            Some(p) => ptrs[i] = p,
            None => {
                shell_println!("run: argument too long");
                return;
            }
        }
    }
    ptrs[count] = core::ptr::null();
    let envp = [core::ptr::null::<c_char>()];

    let result: c_long = unsafe { fsutil::spawn(ptrs[0], ptrs.as_ptr(), envp.as_ptr()) };
    if result < 0 {
        shell_println!("spawn failed: errno %d", -result as isize);
    } else {
        shell_println!("launched handle %d", result as isize);
    }
}
