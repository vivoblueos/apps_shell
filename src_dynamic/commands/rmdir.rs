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
    if args.is_empty() {
        shell_println!("Usage: rmdir <path1> <path2> ...");
        return;
    }
    for dir in args {
        let mut buf = [0u8; 256];
        let Some(cdir) = console::nul_into(&mut buf, dir) else {
            shell_println!("rmdir: path too long");
            continue;
        };
        if unsafe { fsutil::rmdir(cdir) } != 0 {
            shell_println!("Failed to remove dir '%s'", cdir);
        }
    }
}
