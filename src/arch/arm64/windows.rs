// Copyright Epic Games, Inc. All Rights Reserved.

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
use windows_sys::Win32::System::Threading::IsProcessorFeaturePresent;

#[derive(Default, Debug, Clone)]
pub struct WindowsArm64Probe {
    pub v8_available: bool,
    pub crypto_available: bool,
    pub crc32_available: bool,
    pub v81_atomic_available: bool,
    pub v82_dp_available: bool,
    pub v83_jscvt_available: bool,
    pub v83_lrcpc_available: bool,
    pub sve_available: bool,
    pub sve2_available: bool,
    pub sve2_1_available: bool,
    pub sve_aes_available: bool,
    pub sve_pmull128_available: bool,
    pub sve_bitperm_available: bool,
    pub sve_sha3_available: bool,
    pub sve_sm4_available: bool,
    pub sme_available: bool,
    pub sme2_available: bool,
    pub sme2_1_available: bool,
    pub sme_aes_available: bool,
    pub sme_b16b16_available: bool,
    pub sme_f16f16_available: bool,
    pub sme_f32f32_available: bool,
    pub sme_f64f64_available: bool,
    pub sme_i16i64_available: bool,
    pub sme_fa64_available: bool,
}

impl WindowsArm64Probe {
    pub fn query() -> Self {
        #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
        unsafe {
            Self {
                v8_available: IsProcessorFeaturePresent(29) != 0,
                crypto_available: IsProcessorFeaturePresent(30) != 0,
                crc32_available: IsProcessorFeaturePresent(31) != 0,
                v81_atomic_available: IsProcessorFeaturePresent(34) != 0,
                v82_dp_available: IsProcessorFeaturePresent(43) != 0,
                v83_jscvt_available: IsProcessorFeaturePresent(44) != 0,
                v83_lrcpc_available: IsProcessorFeaturePresent(45) != 0,
                sve_available: IsProcessorFeaturePresent(46) != 0,
                sve2_available: IsProcessorFeaturePresent(47) != 0,
                sve2_1_available: IsProcessorFeaturePresent(48) != 0,
                sve_aes_available: IsProcessorFeaturePresent(49) != 0,
                sve_pmull128_available: IsProcessorFeaturePresent(50) != 0,
                sve_bitperm_available: IsProcessorFeaturePresent(51) != 0,
                sve_sha3_available: IsProcessorFeaturePresent(52) != 0,
                sve_sm4_available: IsProcessorFeaturePresent(53) != 0,
                sme_available: IsProcessorFeaturePresent(54) != 0,
                sme2_available: IsProcessorFeaturePresent(55) != 0,
                sme2_1_available: IsProcessorFeaturePresent(56) != 0,
                sme_aes_available: IsProcessorFeaturePresent(57) != 0,
                sme_b16b16_available: IsProcessorFeaturePresent(59) != 0,
                sme_f16f16_available: IsProcessorFeaturePresent(60) != 0,
                sme_f32f32_available: IsProcessorFeaturePresent(61) != 0,
                sme_f64f64_available: IsProcessorFeaturePresent(62) != 0,
                sme_i16i64_available: IsProcessorFeaturePresent(63) != 0,
                sme_fa64_available: IsProcessorFeaturePresent(64) != 0,
            }
        }

        #[cfg(not(all(target_os = "windows", target_arch = "aarch64")))]
        {
            Self::default()
        }
    }
}

