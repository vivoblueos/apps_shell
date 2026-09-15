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

use crate::{console, shell_println};

fn parse_i64(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    let (neg, digits) = if let Some(rest) = s.strip_prefix('-') {
        (true, rest)
    } else {
        (false, s)
    };
    let mut v = 0u64;
    for b in digits.bytes() {
        let d = (b as char).to_digit(10)?;
        v = v.checked_mul(10)?.checked_add(d as u64)?;
    }
    if neg {
        Some(-(v as i64))
    } else {
        Some(v as i64)
    }
}

pub fn command(args: &[&str]) {
    if args.is_empty() {
        shell_println!("Usage: printf string<%s, %d, %f> [arg...]");
        return;
    }

    // The first arg is the format, the rest are values; expand into `out`.
    let fmt = args[0];
    let mut arg_i = 1usize;
    let mut out = [0u8; 256];
    let mut n = 0usize;

    let mut chars = fmt.bytes();
    while let Some(c) = chars.next() {
        if c == b'%' {
            let Some(spec) = chars.next() else { break };
            let value = if arg_i < args.len() { args[arg_i] } else { "" };
            arg_i += 1;
            match spec {
                b's' => {
                    let take = value.len().min(out.len() - n - 1);
                    out[n..n + take].copy_from_slice(&value.as_bytes()[..take]);
                    n += take;
                }
                b'd' | b'i' => match parse_i64(value) {
                    Some(v) => {
                        let text = i64_text(v);
                        let take = text.len().min(out.len() - n - 1);
                        out[n..n + take].copy_from_slice(&text[..take]);
                        n += take;
                    }
                    None => {
                        let mut vbuf = [0u8; 32];
                        let cval = console::nul_into(&mut vbuf, value).unwrap_or(core::ptr::null());
                        shell_println!("Expected integer, got '%s'", cval);
                        return;
                    }
                },
                b'%' => {
                    if n + 1 < out.len() {
                        out[n] = b'%';
                        n += 1;
                    }
                }
                _ => {
                    shell_println!("Unsupported format specifier: %%%c", spec as isize);
                    return;
                }
            }
        } else if n + 1 < out.len() {
            out[n] = c;
            n += 1;
        }
    }
    out[n] = 0;
    shell_println!("%s", out.as_ptr());
}

/// Decimal text of `v` (front-aligned in a fresh buffer).
fn i64_text(v: i64) -> [u8; 24] {
    let mut tmp = [0u8; 24];
    let neg = v < 0;
    let mut v = v.unsigned_abs();
    let mut i = tmp.len();
    if v == 0 {
        i -= 1;
        tmp[i] = b'0';
    }
    while v > 0 && i > 1 {
        i -= 1;
        tmp[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    if neg {
        i -= 1;
        tmp[i] = b'-';
    }
    let mut out = [0u8; 24];
    let text = &tmp[i..];
    let start = out.len() - text.len();
    out[start..].copy_from_slice(text);
    out
}
