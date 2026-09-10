// SPDX-FileCopyrightText: Copyright (c) 2026 Navegos. @DevelVitorF. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// project: ArchInfo
// file: src/arch/riscv64/linux.rs
// created: 2026-09-05
// lastModified: 2026-09-09

#[derive(Default, Debug, Clone)]
pub struct LinuxRiscv64Probe {
    pub hwcap: u64,
    pub hwprobe_ext: u64,
}

impl LinuxRiscv64Probe {
    pub fn query() -> Self {
        #[cfg(all(target_os = "linux", target_arch = "riscv64"))]
        unsafe {
            const AT_HWCAP: libc::c_ulong = 16;
            let hwcap = libc::getauxval(AT_HWCAP) as u64;

            // Linux riscv_hwprobe syscall (syscall 258)
            #[repr(C)]
            struct RiscvHwprobe {
                key: i64,
                value: u64,
            }

            const RISCV_HWPROBE_KEY_IMA_EXT_0: i64 = 4;
            let mut pair = RiscvHwprobe {
                key: RISCV_HWPROBE_KEY_IMA_EXT_0,
                value: 0,
            };

            let ret = libc::syscall(
                258, // __NR_riscv_hwprobe
                &mut pair as *mut _ as *mut libc::c_void,
                1 as libc::size_t,
                0 as libc::size_t,
                std::ptr::null::<libc::c_void>(),
                0 as libc::c_uint,
            );

            let hwprobe_ext = if ret == 0 { pair.value } else { 0 };

            Self { hwcap, hwprobe_ext }
        }

        #[cfg(not(all(target_os = "linux", target_arch = "riscv64")))]
        {
            Self::default()
        }
    }

    pub fn has_i(&self) -> bool {
        (self.hwcap & (1 << (b'I' - b'A'))) != 0
    }

    pub fn has_m(&self) -> bool {
        (self.hwcap & (1 << (b'M' - b'A'))) != 0
    }

    pub fn has_a(&self) -> bool {
        (self.hwcap & (1 << (b'A' - b'A'))) != 0
    }

    pub fn has_f(&self) -> bool {
        (self.hwcap & (1 << (b'F' - b'A'))) != 0
    }

    pub fn has_d(&self) -> bool {
        (self.hwcap & (1 << (b'D' - b'A'))) != 0
    }

    pub fn has_c(&self) -> bool {
        (self.hwcap & (1 << (b'C' - b'A'))) != 0
    }

    pub fn has_v(&self) -> bool {
        (self.hwcap & (1 << (b'V' - b'A'))) != 0
    }
}

