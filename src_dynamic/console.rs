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

//! C-style console helpers for the dynamic shell.
//!
//! The shell is a `no_std` PIE without the Rust `alloc` crate (the shared
//! libc's own allocator is internal to the DSO), so all output goes through
//! the shared libc's `printf` and all dynamic strings are copied into caller
//! buffers and NUL-terminated.

use core::ffi::{c_char, c_int};

extern "C" {
    /// The shared libc's `printf`.
    pub fn printf(format: *const c_char, ...) -> c_int;
}

/// `printf`-style output for the shell.
///
/// ```ignore
/// shell_println!("launched handle %d", result);
/// ```
#[macro_export]
macro_rules! shell_print {
    ($fmt:expr $(, $arg:expr)*) => {{
        #[allow(unused_unsafe)]
        unsafe {
            $crate::console::printf(
                concat!($fmt, "\0").as_ptr() as *const core::ffi::c_char
                $(, $arg as isize)*
            );
        }
    }};
}

#[macro_export]
macro_rules! shell_println {
    () => {
        $crate::shell_print!("\n")
    };
    ($fmt:expr $(, $arg:expr)*) => {{
        $crate::shell_print!(concat!($fmt, "\n") $(, $arg)*);
    }};
}

/// Copy `s` into `buf` with a trailing NUL and return the C pointer.
/// Returns `None` when `buf` is too small.
pub fn nul_into<'a>(buf: &'a mut [u8], s: &str) -> Option<*const c_char> {
    let bytes = s.as_bytes();
    if bytes.len() + 1 > buf.len() {
        return None;
    }
    buf[..bytes.len()].copy_from_slice(bytes);
    buf[bytes.len()] = 0;
    Some(buf.as_ptr() as *const c_char)
}

/// Write raw bytes to fd 1 through the shared libc's `write`.
pub fn write_stdout(bytes: &[u8]) {
    unsafe {
        crate::fsutil::write(1, bytes.as_ptr() as *const core::ffi::c_void, bytes.len());
    }
    // Keep the external call from becoming a tail call through an even-address
    // Thumb PLT veneer. Normal calls use `bl ...@plt` and preserve Thumb state.
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

/// Write a NUL-terminated C string to fd 1.
pub fn write_cstr(s: *const c_char) {
    let mut len = 0usize;
    while unsafe { *s.add(len) } != 0 {
        len += 1;
    }
    unsafe {
        crate::fsutil::write(1, s as *const core::ffi::c_void, len);
    }
}

/// Write a `&str` (may contain interior NULs-free text only) to fd 1.
pub fn write_str(s: &str) {
    write_stdout(s.as_bytes());
}
