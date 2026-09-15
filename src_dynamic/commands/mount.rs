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
    if args.len() != 2 {
        shell_println!("Usage: mount <path> <fstype>");
        return;
    }
    let mut tbuf = [0u8; 256];
    let mut fbuf = [0u8; 64];
    let (Some(target), Some(fstype)) = (
        console::nul_into(&mut tbuf, args[0]),
        console::nul_into(&mut fbuf, args[1]),
    ) else {
        shell_println!("mount: path too long");
        return;
    };
    let rc = unsafe { fsutil::mount(core::ptr::null(), target, fstype, 0, core::ptr::null()) };
    if rc != 0 {
        shell_println!("mount failed (error code: %d)", rc);
    }
}
