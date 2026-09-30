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

//! C-style filesystem and path helpers for the dynamic shell.
//!
//! The shell has no Rust `alloc`, so every path lives in a caller-provided
//! byte buffer and crosses the DSO boundary as a NUL-terminated C string.

use core::ffi::{c_char, c_int, c_long, c_void};

use libc::{dirent, off_t, size_t, ssize_t, stat as stat_t, DIR};

extern "C" {
    pub fn open(path: *const c_char, oflag: c_int, mode: c_int) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn mkdir(path: *const c_char, mode: c_int) -> c_int;
    pub fn rmdir(path: *const c_char) -> c_int;
    pub fn unlink(path: *const c_char) -> c_int;
    pub fn chdir(path: *const c_char) -> c_int;
    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    pub fn stat(path: *const c_char, buf: *mut stat_t) -> c_int;
    pub fn fstat(fd: c_int, buf: *mut stat_t) -> c_int;
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;
    pub fn mount(
        source: *const c_char,
        target: *const c_char,
        fstype: *const c_char,
        flags: u64,
        data: *const c_void,
    ) -> c_int;
    pub fn umount(target: *const c_char) -> c_int;
    pub fn opendir(path: *const c_char) -> *mut DIR;
    pub fn readdir(dir: *mut DIR) -> *mut dirent;
    pub fn closedir(dir: *mut DIR) -> c_int;
    pub fn spawn(
        path: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_long;
}

/// Read a whole fd into `out` (up to its capacity). Returns the filled length.
pub fn read_all(fd: c_int, out: &mut [u8]) -> isize {
    let mut filled = 0usize;
    while filled < out.len() {
        let n = unsafe {
            read(
                fd,
                out[filled..].as_mut_ptr() as *mut c_void,
                out.len() - filled,
            )
        };
        if n <= 0 {
            break;
        }
        filled += n as usize;
    }
    filled as isize
}

/// Copy `path` into `buf` NUL-terminated, failing when it does not fit.
pub fn path_into<'a>(buf: &'a mut [u8], path: &str) -> Option<*const c_char> {
    crate::console::nul_into(buf, path)
}

/// The length of a NUL-terminated string.
pub fn cstr_len(s: *const c_char) -> usize {
    let mut len = 0usize;
    while unsafe { *s.add(len) } != 0 {
        len += 1;
    }
    len
}

/// Copy the bytes of a NUL-terminated string into `out` and return the slice.
pub fn cstr_copy<'a>(out: &'a mut [u8], s: *const c_char) -> &'a [u8] {
    let len = cstr_len(s);
    let len = len.min(out.len());
    for (i, b) in out[..len].iter_mut().enumerate() {
        *b = unsafe { *s.add(i) } as u8;
    }
    &out[..len]
}
