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

use crate::{console, fsutil};

pub fn command(args: &[&str]) {
    if args.is_empty() {
        crate::shell_println!("Usage: touch <file1> <file2> ...");
        return;
    }
    for filename in args {
        let mut buf = [0u8; 256];
        let Some(cpath) = console::nul_into(&mut buf, filename) else {
            crate::shell_println!("touch: path too long");
            continue;
        };
        // Create the file if it does not exist; opening succeeds either way.
        let fd = unsafe { fsutil::open(cpath, libc::O_CREAT | libc::O_WRONLY, 0o644) };
        if fd >= 0 {
            unsafe { fsutil::close(fd) };
        }
    }
}
