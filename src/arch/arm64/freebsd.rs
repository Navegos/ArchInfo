#[derive(Default, Debug, Clone)]
pub struct FreeBsdArm64Probe {
    pub hwcap: u64,
}

impl FreeBsdArm64Probe {
    pub fn query() -> Self {
        #[cfg(all(target_os = "freebsd", target_arch = "aarch64"))]
        unsafe {
            let mut hwcap: libc::c_ulong = 0;
            const AT_HWCAP: libc::c_int = 25;
            let ret = libc::elf_aux_info(
                AT_HWCAP,
                &mut hwcap as *mut _ as *mut libc::c_void,
                std::mem::size_of::<libc::c_ulong>() as libc::c_int,
            );
            if ret == 0 {
                Self { hwcap: hwcap as u64 }
            } else {
                Self::default()
            }
        }

        #[cfg(not(all(target_os = "freebsd", target_arch = "aarch64")))]
        {
            Self::default()
        }
    }
}

