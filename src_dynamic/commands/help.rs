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

use crate::{commands::COMMANDS, console, shell_println};

pub fn command(args: &[&str]) {
    if args.is_empty() {
        shell_println!("BlueOS kernel shell commands:");
        for (name, cmdinfo) in COMMANDS.entries() {
            let mut nbuf = [0u8; 32];
            let mut dbuf = [0u8; 128];
            let (Some(cname), Some(cdesc)) = (
                console::nul_into(&mut nbuf, name),
                console::nul_into(&mut dbuf, cmdinfo.description),
            ) else {
                continue;
            };
            shell_println!("  %-10s - %s", cname, cdesc);
        }
    } else {
        let cmdname = args[0];
        let mut nbuf = [0u8; 32];
        let Some(cname) = console::nul_into(&mut nbuf, cmdname) else {
            return;
        };
        match COMMANDS.get(cmdname) {
            Some(cmdinfo) => {
                let mut dbuf = [0u8; 128];
                if let Some(cdesc) = console::nul_into(&mut dbuf, cmdinfo.description) {
                    shell_println!("%s - %s", cname, cdesc);
                }
            }
            None => shell_println!("Unknown command: %s", cname),
        }
    }
}
