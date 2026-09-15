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
        shell_println!("Usage: umount <target>");
        return;
    }
    let mut buf = [0u8; 256];
    let Some(target) = console::nul_into(&mut buf, args[0]) else {
        shell_println!("umount: path too long");
        return;
    };
    let rc = unsafe { fsutil::umount(target) };
    if rc != 0 {
        shell_println!("umount failed (error code: %d)", rc);
    }
}
