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

use crate::{fsutil, shell_println};

pub fn command(_args: &[&str]) {
    let mut cwd = [0u8; 256];
    let p = unsafe { fsutil::getcwd(cwd.as_mut_ptr() as *mut c_char, cwd.len()) };
    if p.is_null() {
        shell_println!("Unable to get current directory");
        return;
    }
    // `getcwd` wrote the NUL-terminated path into `cwd`; print it directly.
    shell_println!("%s", cwd.as_ptr());
}
