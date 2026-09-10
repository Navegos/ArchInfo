// SPDX-FileCopyrightText: Copyright (c) 2026 Navegos. @DevelVitorF. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// project: ArchInfo
// file: src/platform.rs
// created: 2026-09-05
// lastModified: 2026-09-09

use crate::vector_length::CpuArchitectureVectorLength;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported CPU Architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub enum Arch {
    #[default]
    Native,
    #[serde(rename = "x86_64")]
    X86_64,
    #[serde(rename = "aarch64")]
    Arm64,
    #[serde(rename = "riscv64")]
    Riscv64,
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arch::Native => write!(f, "native"),
            Arch::X86_64 => write!(f, "x86_64"),
            Arch::Arm64 => write!(f, "aarch64"),
            Arch::Riscv64 => write!(f, "riscv64"),
        }
    }
}

impl FromStr for Arch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().replace('-', "_").as_str() {
            "native" | "host" | "current" => Ok(Arch::Native),
            "x86_64" | "x64" | "amd64" | "x86-64" => Ok(Arch::X86_64),
            "arm64" | "aarch64" | "arm64ec" | "arm64e" => Ok(Arch::Arm64),
            "riscv64" | "riscv" | "rv64" => Ok(Arch::Riscv64),
            other => Err(format!("Unknown architecture: {}", other)),
        }
    }
}

impl Arch {
    /// Resolves Arch::Native to current host architecture
    pub fn resolve(self) -> Self {
        match self {
            Arch::Native => Arch::current(),
            other => other,
        }
    }

    /// Detects current host architecture at compile-time/runtime
    pub fn current() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Arch::X86_64
        }
        #[cfg(target_arch = "aarch64")]
        {
            Arch::Arm64
        }
        #[cfg(target_arch = "riscv64")]
        {
            Arch::Riscv64
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "riscv64")))]
        {
            Arch::X86_64
        }
    }

    /// Returns the list of all concrete supported architectures
    pub fn all() -> &'static [Arch] {
        &[Arch::X86_64, Arch::Arm64, Arch::Riscv64]
    }
}

/// Target Platforms supported by Unreal Engine and modern gaming / runtime environments
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    #[default]
    Native,
    Android,
    Windows,
    Linux,
    Freebsd,
    Macosx,
    Ios,
    Tvos,
    Xros,
    Xboxone,
    Xboxxs,
    Ps4,
    Ps5,
    #[serde(alias = "nx2")]
    Switch2,
    Steamdeck,
    Steammachine,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::Native => write!(f, "native"),
            Platform::Android => write!(f, "android"),
            Platform::Windows => write!(f, "windows"),
            Platform::Linux => write!(f, "linux"),
            Platform::Freebsd => write!(f, "freebsd"),
            Platform::Macosx => write!(f, "macosx"),
            Platform::Ios => write!(f, "ios"),
            Platform::Tvos => write!(f, "tvos"),
            Platform::Xros => write!(f, "xros"),
            Platform::Xboxone => write!(f, "xboxone"),
            Platform::Xboxxs => write!(f, "xboxxs"),
            Platform::Ps4 => write!(f, "ps4"),
            Platform::Ps5 => write!(f, "ps5"),
            Platform::Switch2 => write!(f, "switch2"),
            Platform::Steamdeck => write!(f, "steamdeck"),
            Platform::Steammachine => write!(f, "steammachine"),
        }
    }
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "native" | "host" | "current" => Ok(Platform::Native),
            "android" => Ok(Platform::Android),
            "windows" | "win64" | "win" => Ok(Platform::Windows),
            "linux" => Ok(Platform::Linux),
            "freebsd" => Ok(Platform::Freebsd),
            "macosx" | "macos" | "mac" | "darwin" => Ok(Platform::Macosx),
            "ios" => Ok(Platform::Ios),
            "tvos" => Ok(Platform::Tvos),
            "xros" | "visionos" => Ok(Platform::Xros),
            "xboxone" | "xb1" => Ok(Platform::Xboxone),
            "xboxxs" | "xboxseriesx" | "xboxseries" => Ok(Platform::Xboxxs),
            "ps4" | "playstation4" => Ok(Platform::Ps4),
            "ps5" | "playstation5" => Ok(Platform::Ps5),
            "switch2" | "nx2" => Ok(Platform::Switch2),
            "steamdeck" => Ok(Platform::Steamdeck),
            "steammachine" => Ok(Platform::Steammachine),
            other => Err(format!("Unknown platform: {}", other)),
        }
    }
}

impl Platform {
    /// Alias for `Platform::Switch2`
    #[allow(non_upper_case_globals)]
    pub const Nx2: Platform = Platform::Switch2;

    /// Resolves Platform::Native to current host platform
    pub fn resolve(self) -> Self {
        match self {
            Platform::Native => Platform::current(),
            other => other,
        }
    }

    /// Detects current host platform
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        {
            Platform::Windows
        }
        #[cfg(target_os = "linux")]
        {
            Platform::Linux
        }
        #[cfg(target_os = "macos")]
        {
            Platform::Macosx
        }
        #[cfg(target_os = "freebsd")]
        {
            Platform::Freebsd
        }
        #[cfg(target_os = "android")]
        {
            Platform::Android
        }
        #[cfg(target_os = "ios")]
        {
            Platform::Ios
        }
        #[cfg(not(any(
            target_os = "windows",
            target_os = "linux",
            target_os = "macos",
            target_os = "freebsd",
            target_os = "android",
            target_os = "ios"
        )))]
        {
            Platform::Windows
        }
    }

    /// Returns whether the specified architecture is compatible with this platform
    pub fn is_arch_compatible(&self, arch: Arch) -> bool {
        let p = self.resolve();
        let a = arch.resolve();
        match p {
            Platform::Native => unreachable!(),
            Platform::Xboxone | Platform::Xboxxs | Platform::Ps4 | Platform::Ps5
            | Platform::Steamdeck | Platform::Steammachine => a == Arch::X86_64,
            Platform::Switch2 | Platform::Ios | Platform::Tvos | Platform::Xros => a == Arch::Arm64,
            Platform::Windows | Platform::Macosx => {
                matches!(a, Arch::X86_64 | Arch::Arm64)
            }

            Platform::Android | Platform::Linux | Platform::Freebsd => {
                matches!(a, Arch::X86_64 | Arch::Arm64 | Arch::Riscv64)
            }
        }
    }

    /// Returns the list of all supported platforms
    pub fn all() -> &'static [Platform] {
        &[
            Platform::Android,
            Platform::Windows,
            Platform::Linux,
            Platform::Freebsd,
            Platform::Macosx,
            Platform::Ios,
            Platform::Tvos,
            Platform::Xros,
            Platform::Xboxone,
            Platform::Xboxxs,
            Platform::Ps4,
            Platform::Ps5,
            Platform::Switch2,
            Platform::Steamdeck,
            Platform::Steammachine,
        ]
    }

    /// Returns default preferred vector length for platforms with dedicated hardware vector width characteristics.
    /// PS5 (AMD Zen 2 custom APU) defaults to 128-bit vector length (vl128) because FP3 was deleted and FP2 stripped.
    pub fn default_vector_length(&self) -> Option<CpuArchitectureVectorLength> {
        match self.resolve() {
            Platform::Macosx | Platform::Ios | Platform::Tvos | Platform::Xros | Platform::Xboxone | Platform::Ps4
            | Platform::Ps5 | Platform::Switch2 | Platform::Steamdeck => Some(CpuArchitectureVectorLength::VL128),
            Platform::Xboxxs | Platform::Steammachine => Some(CpuArchitectureVectorLength::VL256),
            _ => None,
        }
    }

    /// Returns whether this platform is a console platform
    pub fn is_console(&self) -> bool {
        matches!(
            self.resolve(),
            Platform::Xboxone
                | Platform::Xboxxs
                | Platform::Ps4
                | Platform::Ps5
                | Platform::Switch2
                | Platform::Steamdeck
                | Platform::Steammachine
        )
    }

    /// Returns the target CPU for console platforms, which is the same name as the console platform
    pub fn console_target_cpu(&self) -> Option<&'static str> {
        match self.resolve() {
            Platform::Xboxone => Some("xboxone"),
            Platform::Xboxxs => Some("xboxxs"),
            Platform::Ps4 => Some("ps4"),
            Platform::Ps5 => Some("ps5"),
            Platform::Switch2 => Some("switch2"),
            Platform::Steamdeck => Some("steamdeck"),
            Platform::Steammachine => Some("steammachine"),
            _ => None,
        }
    }
}
