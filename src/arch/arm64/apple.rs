#[derive(Default, Debug, Clone)]
pub struct AppleArm64Probe {
    pub feat_aes: bool,
    pub feat_pmull: bool,
    pub feat_advsimd: bool,
    pub feat_bf16: bool,
    pub feat_bti: bool,
    pub feat_crc32: bool,
    pub feat_csv2: bool,
    pub feat_dit: bool,
    pub feat_dpb: bool,
    pub feat_dpb2: bool,
    pub feat_dotprod: bool,
    pub feat_ecv: bool,
    pub feat_fcma: bool,
    pub feat_fgt: bool,
    pub feat_fhm: bool,
    pub feat_fp: bool,
    pub feat_fp16: bool,
    pub feat_fpac: bool,
    pub feat_frintts: bool,
    pub feat_flagm: bool,
    pub feat_flagm2: bool,
    pub feat_hcx: bool,
    pub feat_i8mm: bool,
    pub feat_jscvt: bool,
    pub feat_lor: bool,
    pub feat_lrcpc: bool,
    pub feat_lrcpc2: bool,
    pub feat_lse: bool,
    pub feat_lse2: bool,
    pub feat_pauth: bool,
    pub feat_pmuv3: bool,
    pub feat_ras: bool,
    pub feat_rdm: bool,
    pub feat_sb: bool,
    pub feat_sha1: bool,
    pub feat_sha256: bool,
    pub feat_sha3: bool,
    pub feat_sha512: bool,
    pub feat_sme: bool,
    pub feat_sme2: bool,
    pub feat_sme_f64f64: bool,
    pub feat_sme_i16i64: bool,
    pub feat_specres: bool,
    pub feat_wfxt: bool,
}

impl AppleArm64Probe {
    pub fn query() -> Self {
        #[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
        {
            fn check_sysctl(name: &str) -> bool {
                use std::ffi::CString;
                let c_name = CString::new(name).unwrap();
                let mut val: i32 = 0;
                let mut size = std::mem::size_of::<i32>();
                unsafe {
                    let ret = libc::sysctlbyname(
                        c_name.as_ptr(),
                        &mut val as *mut _ as *mut libc::c_void,
                        &mut size,
                        std::ptr::null_mut(),
                        0,
                    );
                    ret == 0 && val == 1
                }
            }

            Self {
                feat_aes: check_sysctl("hw.optional.arm.FEAT_AES"),
                feat_pmull: check_sysctl("hw.optional.arm.FEAT_PMULL"),
                feat_advsimd: check_sysctl("hw.optional.AdvSIMD"),
                feat_bf16: check_sysctl("hw.optional.arm.FEAT_BF16"),
                feat_bti: check_sysctl("hw.optional.arm.FEAT_BTI"),
                feat_crc32: check_sysctl("hw.optional.arm.FEAT_CRC32"),
                feat_csv2: check_sysctl("hw.optional.arm.FEAT_CSV2"),
                feat_dit: check_sysctl("hw.optional.arm.FEAT_DIT"),
                feat_dpb: check_sysctl("hw.optional.arm.FEAT_DPB"),
                feat_dpb2: check_sysctl("hw.optional.arm.FEAT_DPB2"),
                feat_dotprod: check_sysctl("hw.optional.arm.FEAT_DotProd"),
                feat_ecv: check_sysctl("hw.optional.arm.FEAT_ECV"),
                feat_fcma: check_sysctl("hw.optional.arm.FEAT_FCMA"),
                feat_fgt: check_sysctl("hw.optional.arm.FEAT_FGT"),
                feat_fhm: check_sysctl("hw.optional.arm.FEAT_FHM"),
                feat_fp: check_sysctl("hw.optional.floatingpoint"),
                feat_fp16: check_sysctl("hw.optional.arm.FEAT_FP16"),
                feat_fpac: check_sysctl("hw.optional.arm.FEAT_FPAC"),
                feat_frintts: check_sysctl("hw.optional.arm.FEAT_FRINTTS"),
                feat_flagm: check_sysctl("hw.optional.arm.FEAT_FlagM"),
                feat_flagm2: check_sysctl("hw.optional.arm.FEAT_FlagM2"),
                feat_hcx: check_sysctl("hw.optional.arm.FEAT_HCX"),
                feat_i8mm: check_sysctl("hw.optional.arm.FEAT_I8MM"),
                feat_jscvt: check_sysctl("hw.optional.arm.FEAT_JSCVT"),
                feat_lor: check_sysctl("hw.optional.arm.FEAT_LOR"),
                feat_lrcpc: check_sysctl("hw.optional.arm.FEAT_LRCPC"),
                feat_lrcpc2: check_sysctl("hw.optional.arm.FEAT_LRCPC2"),
                feat_lse: check_sysctl("hw.optional.arm.FEAT_LSE"),
                feat_lse2: check_sysctl("hw.optional.arm.FEAT_LSE2"),
                feat_pauth: check_sysctl("hw.optional.arm.FEAT_PAuth"),
                feat_pmuv3: check_sysctl("hw.optional.arm.FEAT_PMUv3"),
                feat_ras: check_sysctl("hw.optional.arm.FEAT_RAS"),
                feat_rdm: check_sysctl("hw.optional.arm.FEAT_RDM"),
                feat_sb: check_sysctl("hw.optional.arm.FEAT_SB"),
                feat_sha1: check_sysctl("hw.optional.arm.FEAT_SHA1"),
                feat_sha256: check_sysctl("hw.optional.arm.FEAT_SHA256"),
                feat_sha3: check_sysctl("hw.optional.arm.FEAT_SHA3"),
                feat_sha512: check_sysctl("hw.optional.arm.FEAT_SHA512"),
                feat_sme: check_sysctl("hw.optional.arm.FEAT_SME"),
                feat_sme2: check_sysctl("hw.optional.arm.FEAT_SME2"),
                feat_sme_f64f64: check_sysctl("hw.optional.arm.FEAT_SME_F64F64"),
                feat_sme_i16i64: check_sysctl("hw.optional.arm.FEAT_SME_I16I64"),
                feat_specres: check_sysctl("hw.optional.arm.FEAT_SPECRES"),
                feat_wfxt: check_sysctl("hw.optional.arm.FEAT_WFxT"),
            }
        }

        #[cfg(not(all(target_vendor = "apple", target_arch = "aarch64")))]
        {
            Self::default()
        }
    }
}

