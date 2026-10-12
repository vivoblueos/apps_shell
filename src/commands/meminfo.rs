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

use allocator_crate::MemoryInfo;
use blueos_header::syscalls::NR::MemInfo;
use blueos_scal::bk_syscall;

pub fn command(_args: &[&str]) -> Result<(), String> {
    let mut info = MemoryInfo::default();
    // 出参：把用户态结构体指针传给内核，由内核填充。
    // direct 模式下 bk_syscall! 展开为带类型检查的直接调用，必须使用内核真实类型。
    let ret = bk_syscall!(MemInfo, &mut info as *mut MemoryInfo);
    if ret != 0 {
        return Err(format!("MemInfo syscall failed, ret = {}", ret));
    }
    println!("MemInfo syscall (kernel allocator stats):");
    println!("  total    = {} kB", info.total / 1024);
    println!("  used     = {} kB", info.used / 1024);
    println!("  max_used = {} kB", info.max_used / 1024);
    Ok(())
}
