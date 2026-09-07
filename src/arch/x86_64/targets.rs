use crate::vector_length::CpuArchitectureVectorLength;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Specifies target processor for compiling and optimizing to specifics of micro-architectures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TargetCpuArchitectureX64 {
    #[default]
    None,
    Generic,
    Native,

    // SSE4.2 Support CPUs
    X86_64_v2,
    Corei7,
    Nehalem,
    Westmere,
    Slm,
    Silvermont,
    Goldmont,
    Goldmont_plus,
    Tremont,

    // AVX Support CPUs
    Corei7_avx,
    Core_avx_i,
    Sandybridge,
    Ivybridge,
    Bdver1,
    Bdver2,
    Bdver3,
    Btver2,
    Xboxone,
    Ps4,

    // AVX2 Support CPUs
    X86_64_v3,
    Core_AVX2,
    Haswell,
    Broadwell,
    Skylake,
    Alderlake,
    Raptorlake,
    Meteorlake,
    Gracemont,
    Sierraforest,
    Grandridge,
    Arrowlake,
    Arrowlake_s,
    Lunarlake,
    Pantherlake,
    Wildcatlake,
    Clearwaterforest,
    Bdver4,
    Znver1,
    Znver2,
    Xboxxs,
    Ps5,
    Steamdeck,
    Znver3,

    // AVX512 Support CPUs
    X86_64_v4,
    Skx,
    Skylake_avx512,
    Cannonlake,
    Icelake_client,
    Icelake_server,
    Cascadelake,
    Cooperlake,
    Rocketlake,
    Tigerlake,
    Sapphirerapids,
    Emeraldrapids,
    Graniterapids,
    Graniterapids_d,
    Znver4,
    Steammachine,
    Znver5,
    Znver6,

	//AVX10.2 Support CPUs
    Diamondrapids,
    Novalake,
}

/// Minimum baseline CPU architecture for x86_64 code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MinimumCpuArchitectureX64 {
    #[default]
    None,
    AVX,
    AVX2,
    AVX512,
    AVX10_1,
    AVX10_2,
}

impl fmt::Display for MinimumCpuArchitectureX64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MinimumCpuArchitectureX64::None => write!(f, "sse4.2"),
            MinimumCpuArchitectureX64::AVX => write!(f, "avx"),
            MinimumCpuArchitectureX64::AVX2 => write!(f, "avx2"),
            MinimumCpuArchitectureX64::AVX512 => write!(f, "avx512"),
            MinimumCpuArchitectureX64::AVX10_1 => write!(f, "avx10.1"),
            MinimumCpuArchitectureX64::AVX10_2 => write!(f, "avx10.2"),
        }
    }
}

impl FromStr for MinimumCpuArchitectureX64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.trim().to_ascii_lowercase().replace('_', ".");
        match norm.as_str() {
            "none" | "sse4.2" | "sse42" | "default" => Ok(MinimumCpuArchitectureX64::None),
            "avx" => Ok(MinimumCpuArchitectureX64::AVX),
            "avx2" => Ok(MinimumCpuArchitectureX64::AVX2),
            "avx512" | "avx-512" => Ok(MinimumCpuArchitectureX64::AVX512),
            "avx10.1" | "avx10-1" => Ok(MinimumCpuArchitectureX64::AVX10_1),
            "avx10.2" | "avx10-2" => Ok(MinimumCpuArchitectureX64::AVX10_2),
            other => Err(format!("Unknown MinimumCpuArchitectureX64: '{}'", other)),
        }
    }
}

impl MinimumCpuArchitectureX64 {
    /// Evaluates optimal vector length (normal default)
    pub fn vector_length(&self) -> CpuArchitectureVectorLength {
        self.resolve_vector_length(None)
    }

    /// Evaluates vector length given an optional user requested vector length:
    /// - SSE4.2 and AVX: fixed vl128, ignores user input in vector_length.
    /// - AVX2: vl256 or vl128, user can select; if not specified or higher, defaults to vl256.
    /// - AVX512: vl512 or vl256 or vl128, user can select; if not specified, defaults to vl512.
    /// - AVX10.1 and AVX10.2: vl512 or vl256 or vl128, user can select; if not specified, defaults to vl256.
    pub fn resolve_vector_length(&self, requested: Option<CpuArchitectureVectorLength>) -> CpuArchitectureVectorLength {
        match self {
            MinimumCpuArchitectureX64::None | MinimumCpuArchitectureX64::AVX => {
                CpuArchitectureVectorLength::VL128
            }
            MinimumCpuArchitectureX64::AVX2 => {
                match requested {
                    Some(CpuArchitectureVectorLength::VL128) => CpuArchitectureVectorLength::VL128,
                    _ => CpuArchitectureVectorLength::VL256,
                }
            }
            MinimumCpuArchitectureX64::AVX512 => {
                match requested {
                    Some(CpuArchitectureVectorLength::VL128) => CpuArchitectureVectorLength::VL128,
                    Some(CpuArchitectureVectorLength::VL256) => CpuArchitectureVectorLength::VL256,
                    Some(CpuArchitectureVectorLength::VL512) => CpuArchitectureVectorLength::VL512,
                    _ => CpuArchitectureVectorLength::VL512,
                }
            }
            MinimumCpuArchitectureX64::AVX10_1 | MinimumCpuArchitectureX64::AVX10_2 => {
                match requested {
                    Some(CpuArchitectureVectorLength::VL128) => CpuArchitectureVectorLength::VL128,
                    Some(CpuArchitectureVectorLength::VL512) => CpuArchitectureVectorLength::VL512,
                    Some(CpuArchitectureVectorLength::VL256) => CpuArchitectureVectorLength::VL256,
                    _ => CpuArchitectureVectorLength::VL256,
                }
            }
        }
    }
}

/// Provides a mapping between the TargetCpuArchitectureX64 enum values and their corresponding string representations.
pub struct TargetCpuArchitectureX64Names;

impl TargetCpuArchitectureX64Names {
    pub fn name(target: TargetCpuArchitectureX64) -> &'static str {
        match target {
            TargetCpuArchitectureX64::None | TargetCpuArchitectureX64::Generic => "x86-64-v2",
            TargetCpuArchitectureX64::Native => "native",
            TargetCpuArchitectureX64::X86_64_v2 => "x86-64-v2",
            TargetCpuArchitectureX64::Corei7 => "corei7",
            TargetCpuArchitectureX64::Nehalem => "nehalem",
            TargetCpuArchitectureX64::Westmere => "westmere",
            TargetCpuArchitectureX64::Slm => "slm",
            TargetCpuArchitectureX64::Silvermont => "silvermont",
            TargetCpuArchitectureX64::Goldmont => "goldmont",
            TargetCpuArchitectureX64::Goldmont_plus => "goldmont-plus",
            TargetCpuArchitectureX64::Tremont => "tremont",
            TargetCpuArchitectureX64::Corei7_avx => "corei7-avx",
            TargetCpuArchitectureX64::Core_avx_i => "core-avx-i",
            TargetCpuArchitectureX64::Sandybridge => "sandybridge",
            TargetCpuArchitectureX64::Ivybridge => "ivybridge",
            TargetCpuArchitectureX64::Bdver1 => "bdver1",
            TargetCpuArchitectureX64::Bdver2 => "bdver2",
            TargetCpuArchitectureX64::Bdver3 => "bdver3",
            TargetCpuArchitectureX64::Btver2 => "btver2",
            TargetCpuArchitectureX64::Xboxone => "xboxone",
            TargetCpuArchitectureX64::Ps4 => "ps4",
            TargetCpuArchitectureX64::X86_64_v3 => "x86-64-v3",
            TargetCpuArchitectureX64::Core_AVX2 => "core-avx2",
            TargetCpuArchitectureX64::Haswell => "haswell",
            TargetCpuArchitectureX64::Broadwell => "broadwell",
            TargetCpuArchitectureX64::Skylake => "skylake",
            TargetCpuArchitectureX64::Alderlake => "alderlake",
            TargetCpuArchitectureX64::Raptorlake => "raptorlake",
            TargetCpuArchitectureX64::Meteorlake => "meteorlake",
            TargetCpuArchitectureX64::Gracemont => "gracemont",
            TargetCpuArchitectureX64::Sierraforest => "sierraforest",
            TargetCpuArchitectureX64::Grandridge => "grandridge",
            TargetCpuArchitectureX64::Arrowlake => "arrowlake",
            TargetCpuArchitectureX64::Arrowlake_s => "arrowlake-s",
            TargetCpuArchitectureX64::Lunarlake => "lunarlake",
            TargetCpuArchitectureX64::Pantherlake => "pantherlake",
            TargetCpuArchitectureX64::Wildcatlake => "wildcatlake",
            TargetCpuArchitectureX64::Clearwaterforest => "clearwaterforest",
            TargetCpuArchitectureX64::Bdver4 => "bdver4",
            TargetCpuArchitectureX64::Znver1 => "znver1",
            TargetCpuArchitectureX64::Znver2 => "znver2",
            TargetCpuArchitectureX64::Xboxxs => "xboxxs",
            TargetCpuArchitectureX64::Ps5 => "ps5",
            TargetCpuArchitectureX64::Steamdeck => "steamdeck",
            TargetCpuArchitectureX64::Znver3 => "znver3",
            TargetCpuArchitectureX64::X86_64_v4 => "x86-64-v4",
            TargetCpuArchitectureX64::Skx => "skx",
            TargetCpuArchitectureX64::Skylake_avx512 => "skylake-avx512",
            TargetCpuArchitectureX64::Cannonlake => "cannonlake",
            TargetCpuArchitectureX64::Icelake_client => "icelake-client",
            TargetCpuArchitectureX64::Icelake_server => "icelake-server",
            TargetCpuArchitectureX64::Cascadelake => "cascadelake",
            TargetCpuArchitectureX64::Cooperlake => "cooperlake",
            TargetCpuArchitectureX64::Rocketlake => "rocketlake",
            TargetCpuArchitectureX64::Tigerlake => "tigerlake",
            TargetCpuArchitectureX64::Sapphirerapids => "sapphirerapids",
            TargetCpuArchitectureX64::Emeraldrapids => "emeraldrapids",
            TargetCpuArchitectureX64::Graniterapids => "graniterapids",
            TargetCpuArchitectureX64::Graniterapids_d => "graniterapids-d",
            TargetCpuArchitectureX64::Znver4 => "znver4",
            TargetCpuArchitectureX64::Steammachine => "steammachine",
            TargetCpuArchitectureX64::Znver5 => "znver5",
            TargetCpuArchitectureX64::Znver6 => "znver6",
            TargetCpuArchitectureX64::Diamondrapids => "diamondrapids",
            TargetCpuArchitectureX64::Novalake => "novalake",
        }
    }
}

impl FromStr for TargetCpuArchitectureX64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.to_ascii_lowercase().replace(['_', '.'], "-");
        match norm.as_str() {
            "x86-64" | "generic" | "none" | "default" => Ok(TargetCpuArchitectureX64::Generic),
            "native" => Ok(TargetCpuArchitectureX64::Native),
            "x86-64-v2" => Ok(TargetCpuArchitectureX64::X86_64_v2),
            "corei7" => Ok(TargetCpuArchitectureX64::Corei7),
            "nehalem" => Ok(TargetCpuArchitectureX64::Nehalem),
            "westmere" => Ok(TargetCpuArchitectureX64::Westmere),
            "slm" => Ok(TargetCpuArchitectureX64::Slm),
            "silvermont" => Ok(TargetCpuArchitectureX64::Silvermont),
            "goldmont" => Ok(TargetCpuArchitectureX64::Goldmont),
            "goldmont-plus" => Ok(TargetCpuArchitectureX64::Goldmont_plus),
            "tremont" => Ok(TargetCpuArchitectureX64::Tremont),
            "corei7-avx" => Ok(TargetCpuArchitectureX64::Corei7_avx),
            "core-avx-i" => Ok(TargetCpuArchitectureX64::Core_avx_i),
            "sandybridge" => Ok(TargetCpuArchitectureX64::Sandybridge),
            "ivybridge" => Ok(TargetCpuArchitectureX64::Ivybridge),
            "bdver1" => Ok(TargetCpuArchitectureX64::Bdver1),
            "bdver2" => Ok(TargetCpuArchitectureX64::Bdver2),
            "bdver3" => Ok(TargetCpuArchitectureX64::Bdver3),
            "btver2" => Ok(TargetCpuArchitectureX64::Btver2),
            "xboxone" => Ok(TargetCpuArchitectureX64::Xboxone),
            "ps4" => Ok(TargetCpuArchitectureX64::Ps4),
            "x86-64-v3" => Ok(TargetCpuArchitectureX64::X86_64_v3),
            "core-avx2" => Ok(TargetCpuArchitectureX64::Core_AVX2),
            "haswell" => Ok(TargetCpuArchitectureX64::Haswell),
            "broadwell" => Ok(TargetCpuArchitectureX64::Broadwell),
            "skylake" => Ok(TargetCpuArchitectureX64::Skylake),
            "alderlake" => Ok(TargetCpuArchitectureX64::Alderlake),
            "raptorlake" => Ok(TargetCpuArchitectureX64::Raptorlake),
            "meteorlake" => Ok(TargetCpuArchitectureX64::Meteorlake),
            "gracemont" => Ok(TargetCpuArchitectureX64::Gracemont),
            "sierraforest" => Ok(TargetCpuArchitectureX64::Sierraforest),
            "grandridge" => Ok(TargetCpuArchitectureX64::Grandridge),
            "arrowlake" => Ok(TargetCpuArchitectureX64::Arrowlake),
            "arrowlake-s" => Ok(TargetCpuArchitectureX64::Arrowlake_s),
            "lunarlake" => Ok(TargetCpuArchitectureX64::Lunarlake),
            "pantherlake" => Ok(TargetCpuArchitectureX64::Pantherlake),
            "wildcatlake" => Ok(TargetCpuArchitectureX64::Wildcatlake),
            "clearwaterforest" => Ok(TargetCpuArchitectureX64::Clearwaterforest),
            "bdver4" => Ok(TargetCpuArchitectureX64::Bdver4),
            "znver1" => Ok(TargetCpuArchitectureX64::Znver1),
            "znver2" => Ok(TargetCpuArchitectureX64::Znver2),
            "xboxxs" => Ok(TargetCpuArchitectureX64::Xboxxs),
            "ps5" => Ok(TargetCpuArchitectureX64::Ps5),
            "steamdeck" => Ok(TargetCpuArchitectureX64::Steamdeck),
            "znver3" => Ok(TargetCpuArchitectureX64::Znver3),
            "x86-64-v4" => Ok(TargetCpuArchitectureX64::X86_64_v4),
            "skx" => Ok(TargetCpuArchitectureX64::Skx),
            "skylake-avx512" => Ok(TargetCpuArchitectureX64::Skylake_avx512),
            "cannonlake" => Ok(TargetCpuArchitectureX64::Cannonlake),
            "icelake-client" => Ok(TargetCpuArchitectureX64::Icelake_client),
            "icelake-server" => Ok(TargetCpuArchitectureX64::Icelake_server),
            "cascadelake" => Ok(TargetCpuArchitectureX64::Cascadelake),
            "cooperlake" => Ok(TargetCpuArchitectureX64::Cooperlake),
            "rocketlake" => Ok(TargetCpuArchitectureX64::Rocketlake),
            "tigerlake" => Ok(TargetCpuArchitectureX64::Tigerlake),
            "sapphirerapids" => Ok(TargetCpuArchitectureX64::Sapphirerapids),
            "emeraldrapids" => Ok(TargetCpuArchitectureX64::Emeraldrapids),
            "graniterapids" => Ok(TargetCpuArchitectureX64::Graniterapids),
            "graniterapids-d" => Ok(TargetCpuArchitectureX64::Graniterapids_d),
            "znver4" => Ok(TargetCpuArchitectureX64::Znver4),
            "steammachine" => Ok(TargetCpuArchitectureX64::Steammachine),
            "znver5" => Ok(TargetCpuArchitectureX64::Znver5),
            "znver6" => Ok(TargetCpuArchitectureX64::Znver6),
            "diamondrapids" => Ok(TargetCpuArchitectureX64::Diamondrapids),
            "novalake" => Ok(TargetCpuArchitectureX64::Novalake),
            other => Err(format!("Unknown x64 target CPU: {}", other)),
        }
    }
}

