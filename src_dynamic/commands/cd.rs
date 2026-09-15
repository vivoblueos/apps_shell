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
    if args.len() != 1 {
        shell_println!("Usage: cd <directory>");
        return;
    }

    let mut cwd = [0u8; 256];
    let p = unsafe { fsutil::getcwd(cwd.as_mut_ptr() as *mut core::ffi::c_char, cwd.len()) };
    if p.is_null() {
        shell_println!("Unable to get current directory");
        return;
    }
    // Build the target path by applying the components of `args[0]` to cwd.
    let mut target = [0u8; 256];
    let cwd_bytes = fsutil::cstr_copy(&mut target, p);
    let mut len = cwd_bytes.len();

    let arg = args[0];
    if arg != "." {
        for comp in arg.split('/') {
            match comp {
                "" | "." => {}
                ".." => {
                    while len > 1 && target[len - 1] != b'/' {
                        len -= 1;
                    }
                    if len > 1 {
                        len -= 1; // strip the '/'
                    }
                    if len == 0 {
                        len = 1; // stay at "/"
                    }
                }
                dir => {
                    if dir.len() + 1 + len + 1 > target.len() {
                        shell_println!("cd: path too long");
                        return;
                    }
                    if len != 1 || target[0] != b'/' {
                        target[len] = b'/';
                        len += 1;
                    }
                    target[len..len + dir.len()].copy_from_slice(dir.as_bytes());
                    len += dir.len();
                }
            }
        }
    }
    target[len] = 0;
    if unsafe { fsutil::chdir(target.as_ptr() as *const core::ffi::c_char) } != 0 {
        shell_println!("Unable to change directory to '%s'", target.as_ptr());
    }
}
