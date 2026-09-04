#[derive(Default, Debug, Clone)]
pub struct LinuxArm64Probe {
    pub hwcap: u64,
    pub hwcap2: u64,
}

impl LinuxArm64Probe {
    pub fn query() -> Self {
        #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
        unsafe {
            const AT_HWCAP: libc::c_ulong = 16;
            const AT_HWCAP2: libc::c_ulong = 26;
            let hwcap = libc::getauxval(AT_HWCAP) as u64;
            let hwcap2 = libc::getauxval(AT_HWCAP2) as u64;
            Self { hwcap, hwcap2 }
        }

        #[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
        {
            Self::default()
        }
    }

    pub fn has_fp(&self) -> bool {
        (self.hwcap & (1 << 0)) != 0
    }

    pub fn has_asimd(&self) -> bool {
        (self.hwcap & (1 << 1)) != 0
    }

    pub fn has_aes(&self) -> bool {
        (self.hwcap & (1 << 3)) != 0
    }

    pub fn has_sha1(&self) -> bool {
        (self.hwcap & (1 << 5)) != 0
    }

    pub fn has_sha2(&self) -> bool {
        (self.hwcap & (1 << 6)) != 0
    }

    pub fn has_crc32(&self) -> bool {
        (self.hwcap & (1 << 7)) != 0
    }

    pub fn has_atomics(&self) -> bool {
        (self.hwcap & (1 << 8)) != 0
    }

    pub fn has_fp16(&self) -> bool {
        (self.hwcap & (1 << 9)) != 0
    }

    pub fn has_asimdrdm(&self) -> bool {
        (self.hwcap & (1 << 12)) != 0
    }

    pub fn has_jscvt(&self) -> bool {
        (self.hwcap & (1 << 13)) != 0
    }

    pub fn has_fcma(&self) -> bool {
        (self.hwcap & (1 << 14)) != 0
    }

    pub fn has_lrcpc(&self) -> bool {
        (self.hwcap & (1 << 15)) != 0
    }

    pub fn has_sha512(&self) -> bool {
        (self.hwcap & (1 << 21)) != 0
    }

    pub fn has_sha3(&self) -> bool {
        (self.hwcap & (1 << 22)) != 0
    }

    pub fn has_asimddp(&self) -> bool {
        (self.hwcap & (1 << 24)) != 0
    }

    pub fn has_sve(&self) -> bool {
        (self.hwcap & (1 << 22)) != 0
    }

    pub fn has_sve2(&self) -> bool {
        (self.hwcap2 & (1 << 1)) != 0
    }

    pub fn has_sveaes(&self) -> bool {
        (self.hwcap2 & (1 << 2)) != 0
    }

    pub fn has_svepmull(&self) -> bool {
        (self.hwcap2 & (1 << 3)) != 0
    }

    pub fn has_svebitperm(&self) -> bool {
        (self.hwcap2 & (1 << 4)) != 0
    }

    pub fn has_svesha3(&self) -> bool {
        (self.hwcap2 & (1 << 5)) != 0
    }

    pub fn has_svesm4(&self) -> bool {
        (self.hwcap2 & (1 << 6)) != 0
    }

    pub fn has_i8mm(&self) -> bool {
        (self.hwcap2 & (1 << 13)) != 0
    }

    pub fn has_bf16(&self) -> bool {
        (self.hwcap2 & (1 << 14)) != 0
    }

    pub fn has_bti(&self) -> bool {
        (self.hwcap2 & (1 << 17)) != 0
    }

    pub fn has_mops(&self) -> bool {
        (self.hwcap2 & (1 << 43)) != 0
    }
}

