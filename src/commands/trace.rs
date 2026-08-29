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

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
};

const TRACE_CONTROL: &str = "/proc/trace/control";
const TRACE_STATS: &str = "/proc/trace/stats";
const TRACE_DUMP: &str = "/proc/trace/dump";
const TRACE_RAW: &str = "/proc/trace/raw";

pub fn command(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        return Err(usage());
    }

    match args[0] {
        "start" | "stop" | "reset" => {
            if is_tracing_disabled()? {
                println!("tracing is disabled in this build");
                return Ok(());
            }
            write_control(args[0])?;
            println!("trace {}", args[0]);
            Ok(())
        }
        "stats" => {
            print_file(TRACE_STATS)?;
            Ok(())
        }
        "dump" => {
            if is_tracing_disabled()? {
                println!("tracing is disabled in this build");
                return Ok(());
            }
            if args.len() == 1 {
                print_file(TRACE_DUMP)?;
                return Ok(());
            }
            if args.len() == 3 && args[1] == "--raw" {
                let mut out = File::create(args[2])
                    .map_err(|e| format!("failed to create raw output '{}': {}", args[2], e))?;
                let copied = copy_file(TRACE_RAW, &mut out)?;
                println!("saved {} bytes to {}", copied, args[2]);
                return Ok(());
            }
            Err(usage())
        }
        _ => Err(usage()),
    }
}

fn print_file(path: &str) -> Result<(), String> {
    let mut stdout = io::stdout();
    copy_file(path, &mut stdout).map(|_| ())
}

fn copy_file(path: &str, out: &mut dyn Write) -> Result<usize, String> {
    let mut file = File::open(path).map_err(|e| format!("failed to open '{}': {}", path, e))?;
    let mut buf = [0u8; 256];
    let mut total = 0usize;
    loop {
        let n = file
            .read(&mut buf)
            .map_err(|e| format!("failed to read '{}': {}", path, e))?;
        if n == 0 {
            break;
        }
        out.write_all(&buf[..n])
            .map_err(|e| format!("failed to write output for '{}': {}", path, e))?;
        total += n;
    }
    Ok(total)
}

fn write_control(cmd: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(TRACE_CONTROL)
        .map_err(|e| format!("failed to open '{}': {}", TRACE_CONTROL, e))?;
    file.write_all(cmd.as_bytes())
        .map_err(|e| format!("failed to write '{}': {}", TRACE_CONTROL, e))?;
    Ok(())
}

fn is_tracing_disabled() -> Result<bool, String> {
    let mut file =
        File::open(TRACE_STATS).map_err(|e| format!("failed to open '{}': {}", TRACE_STATS, e))?;
    let mut buf = [0u8; 256];
    let n = file
        .read(&mut buf)
        .map_err(|e| format!("failed to read '{}': {}", TRACE_STATS, e))?;
    Ok(contains_bytes(&buf[..n], b"compiled_in=0")
        || contains_bytes(&buf[..n], b"tracing is disabled in this build"))
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

fn usage() -> String {
    "Usage: trace <start|stop|reset|stats|dump [--raw <path>]>".to_string()
}
