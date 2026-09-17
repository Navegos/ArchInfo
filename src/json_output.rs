// SPDX-FileCopyrightText: Copyright (c) 2026 Navegos. @DevelVitorF. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// project: ArchInfo
// file: src/json_output.rs
// created: 2026-09-05
// lastModified: 2026-09-17

use crate::arch::arm64::{self, Arm64CPUFeatures, Arm64ISA, MinimumCpuArchitectureArm64, MinimumCpuArchitectureArm64ClangNames, TargetCpuArchitectureArm64, TargetCpuArchitectureArm64Names};
use crate::arch::riscv64::{self, Riscv64CPUFeatures, Riscv64ISA, TargetCpuArchitectureRiscv64, TargetCpuArchitectureRiscv64Names};
use crate::arch::x86_64::{self, MinimumCpuArchitectureX64, TargetCpuArchitectureX64, TargetCpuArchitectureX64Names, X64CPUFeatures, X64ISA};
use crate::arch::CPUFeatures;
use crate::platform::{Arch, Platform};
use crate::profiles::TargetProfile;
use crate::vector_length::CpuArchitectureVectorLength;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Returns the default output folder based on the operating system:
/// - Windows: `<USER>/AppData/Roaming/Unreal Engine/ArchInfo` or `<USER>/Documents/Unreal Engine/ArchInfo`
/// - macOS: `/Users/<USER>/.config/Unreal Engine/ArchInfo`
/// - Linux/FreeBSD: `/home/<USER>/.config/Unreal Engine/ArchInfo` or `/home/<USER>/Documents/Unreal Engine/ArchInfo`
pub fn get_default_output_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            if !appdata.trim().is_empty() {
                return PathBuf::from(appdata).join("Unreal Engine").join("ArchInfo");
            }
        }
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            let docs = PathBuf::from(&userprofile).join("Documents").join("Unreal Engine").join("ArchInfo");
            let docs_parent = PathBuf::from(&userprofile).join("Documents");
            if docs_parent.exists() {
                return docs;
            }
            return PathBuf::from(userprofile).join("AppData").join("Roaming").join("Unreal Engine").join("ArchInfo");
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            if !home.trim().is_empty() {
                return PathBuf::from(home).join(".config").join("Unreal Engine").join("ArchInfo");
            }
        }
        if let Ok(user) = std::env::var("USER") {
            if !user.trim().is_empty() {
                return PathBuf::from("/Users").join(user).join(".config").join("Unreal Engine").join("ArchInfo");
            }
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            if !xdg.trim().is_empty() {
                return PathBuf::from(xdg).join("Unreal Engine").join("ArchInfo");
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            if !home.trim().is_empty() {
                let config_dir = PathBuf::from(&home).join(".config").join("Unreal Engine").join("ArchInfo");
                return config_dir;
            }
        }
        if let Ok(user) = std::env::var("USER") {
            if !user.trim().is_empty() {
                return PathBuf::from("/home").join(user).join(".config").join("Unreal Engine").join("ArchInfo");
            }
        }
    }

    PathBuf::from(".").join("Unreal Engine").join("ArchInfo")
}

fn parse_extension_tokens(s: &str) -> Vec<&str> {
    s.split(|c| c == '+' || c == ',' || c == ' ')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .collect()
}

fn get_target_arm64_disabled_isas(target: TargetCpuArchitectureArm64) -> Vec<Arm64ISA> {
    parse_extension_tokens(arm64::ClangTargetCpuArchitectureArm64NOISANames::name(target))
        .into_iter()
        .filter_map(|t| t.parse::<Arm64ISA>().ok())
        .collect()
}

fn parse_android_api_level(s: &str) -> Result<(u32, String), String> {
    let raw = s.trim().trim_start_matches("android-").trim_start_matches("ndk-").trim();
    let parts: Vec<&str> = raw.split('.').collect();
    let major: u32 = parts[0]
        .parse()
        .map_err(|_| format!("Invalid Android NDK API level: '{}'", s))?;
    if !(24..=30).contains(&major) {
        return Err(format!(
            "Android NDK API level '{}' is out of supported range (24 to 30)",
            s
        ));
    }
    Ok((major, s.trim().to_string()))
}

fn parse_linux_glibc_version(s: &str) -> Result<String, String> {
    let raw = s.trim().trim_start_matches("glibc-").trim_start_matches("glibc").trim_start_matches('v').trim();
    if raw.contains('.') {
        let parts: Vec<&str> = raw.split('.').collect();
        if parts.len() < 2 || parts[0] != "2" {
            return Err(format!(
                "Linux glibc version must start with 2. (e.g. 2.17 to 2.44), got: '{}'",
                s
            ));
        }
        let minor: u32 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid Linux glibc minor version: '{}'", parts[1]))?;
        if !(17..=44).contains(&minor) {
            return Err(format!(
                "Linux glibc version 2.{} is out of supported range (2.17 to 2.44)",
                minor
            ));
        }
        Ok(format!("2.{}", minor))
    } else {
        let val: u32 = raw
            .parse()
            .map_err(|_| format!("Invalid Linux glibc version: '{}'", s))?;
        if (217..=244).contains(&val) {
            Ok(format!("2.{}", val - 200))
        } else if (17..=44).contains(&val) {
            Ok(format!("2.{}", val))
        } else {
            Err(format!(
                "Linux glibc version '{}' is out of supported range (2.17 to 2.44)",
                s
            ))
        }
    }
}

fn parse_freebsd_os_version(s: &str) -> Result<String, String> {
    let raw = s.trim().trim_start_matches("freebsd-").trim_start_matches("freebsd").trim_start_matches('v').trim();
    if raw.contains('.') {
        let parts: Vec<&str> = raw.split('.').collect();
        let major: u32 = parts[0]
            .parse()
            .map_err(|_| format!("Invalid FreeBSD major version: '{}'", s))?;
        let minor: u32 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid FreeBSD minor version: '{}'", parts[1]))?;
        if major < 13 || major > 15 || (major == 15 && minor > 3) {
            return Err(format!(
                "FreeBSD OS version {}.{} is out of supported range (13.0 to 15.3)",
                major, minor
            ));
        }
        Ok(format!("{}.{}", major, minor))
    } else {
        let major: u32 = raw
            .parse()
            .map_err(|_| format!("Invalid FreeBSD version: '{}'", s))?;
        if (13..=15).contains(&major) {
            Ok(format!("{}.0", major))
        } else {
            Err(format!(
                "FreeBSD OS version '{}' is out of supported range (13.0 to 15.3)",
                s
            ))
        }
    }
}

fn parse_msvc_runtime_version(raw: &str) -> Result<String, String> {
    let s = raw.trim().trim_start_matches("msvc-").trim_start_matches("msvc").trim_start_matches('v').trim();
    if s.contains('.') {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() < 2 || parts[0] != "19" {
            return Err(format!(
                "MSVC version must start with 19 (e.g. 19.30 to 19.52), got: '{}'",
                raw
            ));
        }
        let minor: u32 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid MSVC minor version: '{}'", parts[1]))?;
        if !(30..=52).contains(&minor) {
            return Err(format!(
                "MSVC version 19.{} is out of supported range (19.30 to 19.52)",
                minor
            ));
        }
        if parts.len() == 2 {
            Ok(format!("19.{}", parts[1]))
        } else {
            Ok(format!("19.{}.{}", parts[1], parts[2..].join(".")))
        }
    } else {
        // Pure digits: e.g. 1930, 1952, 193030000, 195299999, 195136257
        if s.len() == 4 {
            if !s.starts_with("19") {
                return Err(format!("MSVC version must start with 19, got: '{}'", raw));
            }
            let minor: u32 = s[2..4]
                .parse()
                .map_err(|_| format!("Invalid MSVC version: '{}'", raw))?;
            if !(30..=52).contains(&minor) {
                return Err(format!(
                    "MSVC version 19{} is out of supported range (1930 to 1952)",
                    minor
                ));
            }
            Ok(format!("19.{}", &s[2..4]))
        } else if s.len() >= 9 {
            if !s.starts_with("19") {
                return Err(format!("MSVC version must start with 19, got: '{}'", raw));
            }
            let minor: u32 = s[2..4]
                .parse()
                .map_err(|_| format!("Invalid MSVC version: '{}'", raw))?;
            if !(30..=52).contains(&minor) {
                return Err(format!(
                    "MSVC version 19{} is out of supported range (1930 to 1952)",
                    minor
                ));
            }
            Ok(format!("19.{}.{}", &s[2..4], &s[4..]))
        } else {
            Err(format!("Invalid MSVC version format: '{}'", raw))
        }
    }
}

fn parse_macos_level(s: &str) -> Result<String, String> {
    let lower = s.trim().to_ascii_lowercase();
    let stripped = lower
        .trim_start_matches("macos-")
        .trim_start_matches("macosx-")
        .trim_start_matches("macos")
        .trim_start_matches("macosx")
        .trim();
    match stripped {
        "15" | "15.0" | "sequoia" => Ok("15.0".to_string()),
        "26" | "26.0" | "tahoe" => Ok("26.0".to_string()),
        "27" | "27.0" | "golden gate" | "goldengate" | "golden_gate" => Ok("27.0".to_string()),
        other => Err(format!(
            "macOS level must be Sequoia (15 or 15.0), Tahoe (26 or 26.0), or Golden Gate (27 or 27.0), got: '{}'",
            other
        )),
    }
}

fn parse_apple_mobile_level(s: &str, os_name: &str) -> Result<String, String> {
    let lower = s.trim().to_ascii_lowercase();
    let stripped = lower
        .trim_start_matches("ios-")
        .trim_start_matches("ios")
        .trim_start_matches("tvos-")
        .trim_start_matches("tvos")
        .trim_start_matches("xros-")
        .trim_start_matches("xros")
        .trim_start_matches("visionos-")
        .trim_start_matches("visionos")
        .trim();
    match stripped {
        "26" | "26.0" => Ok("26.0".to_string()),
        "27" | "27.0" => Ok("27.0".to_string()),
        other => Err(format!(
            "{} level must be 26 (26.0) or 27 (27.0), got: '{}'",
            os_name, other
        )),
    }
}

/// Computes the clang target triple flag, target OS level, and target runtime level.
pub fn compute_target_clang_triple(
    platform: Platform,
    arch: Arch,
    target_os_level: Option<&str>,
    target_runtime_level: Option<&str>,
    target_is_simulator: bool,
) -> Result<(String, String, String), String> {
    let p = platform.resolve();
    let a = arch.resolve();

    if target_is_simulator {
        if !matches!(p, Platform::Ios | Platform::Tvos | Platform::Xros) {
            return Err("-simulator is only accepted for IOS, TVOS, or XrOS".to_string());
        }
        if a == Arch::Arm64E {
            return Err("The simulator is not supported with arm64e".to_string());
        }
    }

    match p {
        Platform::Android => {
            let (api_int, os_lvl) = if let Some(os_str) = target_os_level {
                parse_android_api_level(os_str)?
            } else {
                let detected = std::env::var("ANDROID_PLATFORM")
                    .or_else(|_| std::env::var("ANDROID_NDK_API_LEVEL"))
                    .or_else(|_| std::env::var("ANDROID_API_LEVEL"))
                    .ok();
                if let Some(ref d) = detected {
                    if let Ok(parsed) = parse_android_api_level(d) {
                        parsed
                    } else {
                        (24, "24".to_string())
                    }
                } else {
                    (24, "24".to_string())
                }
            };
            let arch_str = match a {
                Arch::Arm64 => "aarch64",
                Arch::X86_64 => "x86_64",
                Arch::Riscv64 => "riscv64",
                _ => "aarch64",
            };
            let triple = format!("--target='{}-none-linux-android{}'", arch_str, api_int);
            Ok((triple, os_lvl, "".to_string()))
        }

        Platform::Linux | Platform::Steamdeck | Platform::Steammachine => {
            let rt_lvl = if let Some(rt_str) = target_runtime_level {
                parse_linux_glibc_version(rt_str)?
            } else {
                let detected = std::env::var("GLIBC_VERSION").ok();
                if let Some(ref d) = detected {
                    if let Ok(parsed) = parse_linux_glibc_version(d) {
                        parsed
                    } else {
                        "2.17".to_string()
                    }
                } else {
                    "2.17".to_string()
                }
            };
            let arch_str = match a {
                Arch::Arm64 => "aarch64",
                Arch::X86_64 => "x86_64",
                Arch::Riscv64 => "riscv64",
                _ => "x86_64",
            };
            let triple = format!("--target='{}-unknown-linux-gnu{}'", arch_str, rt_lvl);
            Ok((triple, "".to_string(), rt_lvl))
        }

        Platform::Freebsd => {
            let os_lvl = if let Some(os_str) = target_os_level {
                parse_freebsd_os_version(os_str)?
            } else {
                let detected = std::env::var("FREEBSD_VERSION").ok();
                if let Some(ref d) = detected {
                    if let Ok(parsed) = parse_freebsd_os_version(d) {
                        parsed
                    } else {
                        "13.0".to_string()
                    }
                } else {
                    "13.0".to_string()
                }
            };
            let arch_str = match a {
                Arch::Arm64 => "aarch64",
                Arch::X86_64 => "x86_64",
                Arch::Riscv64 => "riscv64",
                _ => "x86_64",
            };
            let triple = format!("--target='{}-unknown-linux-gnu{}'", arch_str, os_lvl);
            Ok((triple, os_lvl, "".to_string()))
        }

        Platform::Windows | Platform::Xboxone | Platform::Xboxxs => {
            let rt_lvl = if let Some(rt_str) = target_runtime_level {
                parse_msvc_runtime_version(rt_str)?
            } else {
                let detected = std::env::var("_MSC_FULL_VER")
                    .or_else(|_| std::env::var("MSVC_VERSION"))
                    .ok();
                if let Some(ref d) = detected {
                    if let Ok(parsed) = parse_msvc_runtime_version(d) {
                        parsed
                    } else {
                        "19.51.36231".to_string()
                    }
                } else {
                    "19.51.36231".to_string()
                }
            };
            let arch_str = match a {
                Arch::Arm64EC => "arm64ec",
                Arch::Arm64 => "aarch64",
                Arch::X86_64 => "x86_64",
                _ => "x86_64",
            };
            let triple = format!("--target='{}-pc-windows-msvc{}'", arch_str, rt_lvl);
            Ok((triple, "".to_string(), rt_lvl))
        }

        Platform::Macosx => {
            let os_lvl = if let Some(os_str) = target_os_level {
                parse_macos_level(os_str)?
            } else {
                let detected = std::env::var("MACOSX_DEPLOYMENT_TARGET").ok();
                if let Some(ref d) = detected {
                    if let Ok(parsed) = parse_macos_level(d) {
                        parsed
                    } else {
                        "15.0".to_string()
                    }
                } else {
                    "15.0".to_string()
                }
            };
            let major: u32 = os_lvl.split('.').next().and_then(|s| s.parse().ok()).unwrap_or(0);
            if a == Arch::X86_64 && major >= 27 {
                return Err("macOS 27 (Golden Gate) and beyond completely drops Intel support".to_string());
            }
            let arch_str = match a {
                Arch::Arm64E => "arm64e",
                Arch::Arm64 => "aarch64",
                Arch::X86_64 => "x86_64",
                _ => "aarch64",
            };
            let triple = format!("--target='{}-apple-macosx{}'", arch_str, os_lvl);
            Ok((triple, os_lvl, "".to_string()))
        }

        Platform::Ios => {
            let os_lvl = if let Some(os_str) = target_os_level {
                parse_apple_mobile_level(os_str, "iOS")?
            } else {
                let detected = std::env::var("IPHONEOS_DEPLOYMENT_TARGET").ok();
                if let Some(ref d) = detected {
                    if let Ok(parsed) = parse_apple_mobile_level(d, "iOS") {
                        parsed
                    } else {
                        "26.0".to_string()
                    }
                } else {
                    "26.0".to_string()
                }
            };
            let arch_str = if a == Arch::Arm64E { "arm64e" } else { "aarch64" };
            let sim = if target_is_simulator { "-simulator" } else { "" };
            let triple = format!("--target='{}-apple-ios{}{}'", arch_str, os_lvl, sim);
            Ok((triple, os_lvl, "".to_string()))
        }

        Platform::Tvos => {
            if a == Arch::Arm64E {
                return Err("arm64e is not supported with TVOS".to_string());
            }
            let os_lvl = if let Some(os_str) = target_os_level {
                parse_apple_mobile_level(os_str, "TVOS")?
            } else {
                let detected = std::env::var("TVOS_DEPLOYMENT_TARGET").ok();
                if let Some(ref d) = detected {
                    if let Ok(parsed) = parse_apple_mobile_level(d, "TVOS") {
                        parsed
                    } else {
                        "26.0".to_string()
                    }
                } else {
                    "26.0".to_string()
                }
            };
            let sim = if target_is_simulator { "-simulator" } else { "" };
            let triple = format!("--target='aarch64-apple-tvos{}{}'", os_lvl, sim);
            Ok((triple, os_lvl, "".to_string()))
        }

        Platform::Xros => {
            if a == Arch::Arm64E {
                return Err("arm64e is not supported with XrOS".to_string());
            }
            let os_lvl = if let Some(os_str) = target_os_level {
                parse_apple_mobile_level(os_str, "XrOS")?
            } else {
                let detected = std::env::var("XROS_DEPLOYMENT_TARGET").ok();
                if let Some(ref d) = detected {
                    if let Ok(parsed) = parse_apple_mobile_level(d, "XrOS") {
                        parsed
                    } else {
                        "26.0".to_string()
                    }
                } else {
                    "26.0".to_string()
                }
            };
            let sim = if target_is_simulator { "-simulator" } else { "" };
            let triple = format!("--target='aarch64-apple-xros{}{}'", os_lvl, sim);
            Ok((triple, os_lvl, "".to_string()))
        }

        Platform::Ps4 => {
            Ok(("--target='x86_64-sie-ps4'".to_string(), "".to_string(), "".to_string()))
        }

        Platform::Ps5 => {
            Ok(("--target='x86_64-sie-ps5'".to_string(), "".to_string(), "".to_string()))
        }

        Platform::Switch2 => {
            Ok(("--target='aarch64-nintendo-nx2'".to_string(), "".to_string(), "".to_string()))
        }

        Platform::Native => unreachable!(),
    }
}

fn serialize_opt_string_as_empty<S>(opt: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match opt {
        Some(s) => serializer.serialize_str(s),
        None => serializer.serialize_str(""),
    }
}

fn deserialize_opt_string_empty_if_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(Some(opt.unwrap_or_default()))
}

fn default_some_empty() -> Option<String> {
    Some(String::new())
}

/// Single Architecture Feature Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchFeaturesReport {
    pub platform: String,
    pub arch: String,
    pub extensions: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tune_cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_cpu_arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_length: Option<String>,
    #[serde(
        default = "default_some_empty",
        serialize_with = "serialize_opt_string_as_empty",
        deserialize_with = "deserialize_opt_string_empty_if_none"
    )]
    pub target_msvc_arch: Option<String>,
    #[serde(
        default = "default_some_empty",
        serialize_with = "serialize_opt_string_as_empty",
        deserialize_with = "deserialize_opt_string_empty_if_none"
    )]
    pub target_msvc_vlen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_clan_arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_clang_isaarch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_clang_cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_clang_tune_cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_clang_vlen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_clang_extraargs: Option<String>,
    #[serde(
        default = "default_some_empty",
        serialize_with = "serialize_opt_string_as_empty",
        deserialize_with = "deserialize_opt_string_empty_if_none"
    )]
    pub target_clang_triple: Option<String>,
    #[serde(
        default = "default_some_empty",
        serialize_with = "serialize_opt_string_as_empty",
        deserialize_with = "deserialize_opt_string_empty_if_none"
    )]
    pub target_os_level: Option<String>,
    #[serde(
        default = "default_some_empty",
        serialize_with = "serialize_opt_string_as_empty",
        deserialize_with = "deserialize_opt_string_empty_if_none"
    )]
    pub target_runtime_level: Option<String>,
    #[serde(default)]
    pub target_is_simulator: bool,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub features: BTreeMap<String, bool>,
}

fn extract_major_minor(version_str: &str) -> String {
    let trimmed = version_str.trim();
    let parts: Vec<&str> = trimmed.split('.').collect();
    if parts.len() >= 2 {
        format!("{}.{}", parts[0], parts[1])
    } else {
        trimmed.to_string()
    }
}

impl ArchFeaturesReport {
    /// Generates canonical file name formatted per platform specification:
    /// - Android: `{base}-ndk-{target_os_level.Major.Minor}.json`
    /// - FreeBSD: `{base}-freebsd-{target_os_level.Major.Minor}.json`
    /// - Linux|Steamdeck|Steammachine: `{base}-glibc-{target_runtime_level.Major.Minor}.json`
    /// - Windows|Xboxone|Xboxxs: `{base}-msvc-{target_runtime_level.Major.Minor}.json`
    /// - Macosx: `{base}-macosx-{target_os_level.Major.Minor}.json`
    /// - IOS: `{base}-ios-{target_os_level.Major.Minor}{if target_is_simulator ? "-simulator" : ""}.json`
    /// - TVOS: `{base}-tvos-{target_os_level.Major.Minor}{if target_is_simulator ? "-simulator" : ""}.json`
    /// - XrOS: `{base}-xros-{target_os_level.Major.Minor}{if target_is_simulator ? "-simulator" : ""}.json`
    /// - Ps4|Ps5|Switch2 or unknown/empty level: `{base}.json`
    pub fn filename(&self) -> String {
        let target = self.target_cpu.as_deref().unwrap_or("none").replace(' ', "-");
        let min_arch = self.min_cpu_arch.as_deref().unwrap_or("none").replace(' ', "-");
        let vl = self.vector_length.as_deref().unwrap_or("none").replace(' ', "-");
        let base = if target.starts_with("x86-64") {
            format!("{}-{}-generic-{}-{}-{}", self.platform, self.arch, target, min_arch, vl)
        } else {
            format!("{}-{}-{}-{}-{}", self.platform, self.arch, target, min_arch, vl)
        };

        let os_level = self
            .target_os_level
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let runtime_level = self
            .target_runtime_level
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());

        match self.platform.to_ascii_lowercase().as_str() {
            "android" => {
                if let Some(lvl) = os_level {
                    format!("{}-ndk-{}.json", base, extract_major_minor(lvl))
                } else {
                    format!("{}.json", base)
                }
            }
            "freebsd" => {
                if let Some(lvl) = os_level {
                    format!("{}-freebsd-{}.json", base, extract_major_minor(lvl))
                } else {
                    format!("{}.json", base)
                }
            }
            "linux" | "steamdeck" | "steammachine" => {
                if let Some(lvl) = runtime_level {
                    format!("{}-glibc-{}.json", base, extract_major_minor(lvl))
                } else {
                    format!("{}.json", base)
                }
            }
            "windows" | "xboxone" | "xboxxs" => {
                if let Some(lvl) = runtime_level {
                    format!("{}-msvc-{}.json", base, extract_major_minor(lvl))
                } else {
                    format!("{}.json", base)
                }
            }
            "macosx" | "macos" => {
                if let Some(lvl) = os_level {
                    format!("{}-macosx-{}.json", base, extract_major_minor(lvl))
                } else {
                    format!("{}.json", base)
                }
            }
            "ios" => {
                if let Some(lvl) = os_level {
                    let sim = if self.target_is_simulator { "-simulator" } else { "" };
                    format!("{}-ios-{}{}.json", base, extract_major_minor(lvl), sim)
                } else {
                    let sim = if self.target_is_simulator { "-simulator" } else { "" };
                    if !sim.is_empty() {
                        format!("{}{}.json", base, sim)
                    } else {
                        format!("{}.json", base)
                    }
                }
            }
            "tvos" => {
                if let Some(lvl) = os_level {
                    let sim = if self.target_is_simulator { "-simulator" } else { "" };
                    format!("{}-tvos-{}{}.json", base, extract_major_minor(lvl), sim)
                } else {
                    let sim = if self.target_is_simulator { "-simulator" } else { "" };
                    if !sim.is_empty() {
                        format!("{}{}.json", base, sim)
                    } else {
                        format!("{}.json", base)
                    }
                }
            }
            "xros" | "visionos" => {
                if let Some(lvl) = os_level {
                    let sim = if self.target_is_simulator { "-simulator" } else { "" };
                    format!("{}-xros-{}{}.json", base, extract_major_minor(lvl), sim)
                } else {
                    let sim = if self.target_is_simulator { "-simulator" } else { "" };
                    if !sim.is_empty() {
                        format!("{}{}.json", base, sim)
                    } else {
                        format!("{}.json", base)
                    }
                }
            }
            _ => format!("{}.json", base),
        }
    }

    /// Alias for filename()
    pub fn default_filename(&self) -> String {
        self.filename()
    }

    /// Returns the default destination path in the OS-specific folder
    pub fn default_filepath(&self) -> PathBuf {
        get_default_output_dir().join(self.filename())
    }

    /// Creates a report for current host detected capabilities
    pub fn from_host(features: &CPUFeatures) -> Self {
        Self::from_host_with_vl(features, None)
    }

    /// Creates a report for current host detected capabilities with optional requested vector length
    pub fn from_host_with_vl(features: &CPUFeatures, requested_vl: Option<CpuArchitectureVectorLength>) -> Self {
        let platform = Platform::current().to_string();
        let arch = Arch::current().to_string();
        let extensions = features.to_extensions_string();
        let map = features.to_map();

        let effective_vl = match requested_vl {
            None | Some(CpuArchitectureVectorLength::None) => Platform::current().default_vector_length(),
            other => other,
        };
        let vl_enum = features.resolve_vector_length(effective_vl);
        let (min_arch, target_msvc_arch, target_msvc_vlen, target_clan_arch, target_clang_isaarch, target_clang_cpu, target_clang_vlen, target_clang_extraargs) = match features {
            CPUFeatures::X86_64(x) => {
                let min_enum = x.minimum_architecture();
                (
                    Some(min_enum.to_string()),
                    Some(x86_64::MSVCX64ArchTarget::name(min_enum).to_string()),
                    Some(x86_64::MSVCX64VLen::name(min_enum, vl_enum).to_string()),
                    Some("-m'arch=native'".to_string()),
                    Some(x.generate_clang_isaarch(vl_enum, &[])),
                    Some("".to_string()),
                    Some(x86_64::ClangX64VLen::name(min_enum, vl_enum).to_string()),
                    Some("".to_string()),
                )
            }
            CPUFeatures::Arm64(a) => {
                let min_enum = a.minimum_architecture();
                let msvc_min = arm64::MinimumCpuArchitectureArm64MSVCNames::name(min_enum);
                let msvc_isa = arm64::MSVCTargetCpuArchitectureArm64ISANames::name(TargetCpuArchitectureArm64::Native);
                let target_msvc_arch = if msvc_isa.is_empty() {
                    format!("/arch:{}", msvc_min)
                } else {
                    format!("/arch:{}+{}", msvc_min, msvc_isa)
                };
                let disabled_isas = get_target_arm64_disabled_isas(TargetCpuArchitectureArm64::Native);
                (
                    Some(min_enum.to_string()),
                    Some(target_msvc_arch),
                    Some("".to_string()),
                    Some(a.generate_clang_isaarch(min_enum, &disabled_isas)),
                    Some("".to_string()),
                    Some("-m'cpu=native'".to_string()),
                    Some("-m'prefer-vector-width=128'".to_string()),
                    Some("".to_string()),
                )
            }
            CPUFeatures::Riscv64(r) => {
                let has_experimental = parse_extension_tokens(&extensions)
                    .iter()
                    .any(|t| t.parse::<Riscv64ISA>().map(|i| i.is_experimental()).unwrap_or(false))
                    || Riscv64ISA::all().iter().any(|&i| i.is_experimental() && r.has_feature(i));
                let extraargs = if has_experimental {
                    "-m'enable-experimental-extensions'".to_string()
                } else {
                    "".to_string()
                };
                (
                    Some("none".to_string()),
                    Some("".to_string()),
                    Some("".to_string()),
                    Some(r.generate_clang_arch(&[])),
                    Some(r.generate_clang_isaarch(&[])),
                    Some("-m'cpu=native'".to_string()),
                    Some("-m'prefer-vector-width=128'".to_string()),
                    Some(extraargs),
                )
            }
        };

        let (target_msvc_arch, target_msvc_vlen) = if Platform::current().uses_msvc() {
            (target_msvc_arch, target_msvc_vlen)
        } else {
            (Some("".to_string()), Some("".to_string()))
        };

        let vl = Some(vl_enum.to_string());

        let (target_clang_triple, target_os_level, target_runtime_level) = compute_target_clang_triple(
            Platform::current(),
            Arch::current(),
            None,
            None,
            false,
        )
        .unwrap_or_else(|_| ("".to_string(), "".to_string(), "".to_string()));

        Self {
            platform,
            arch,
            extensions,
            target_cpu: Some("native".to_string()),
            target_tune_cpu: Some("".to_string()),
            min_cpu_arch: min_arch,
            vector_length: vl,
            target_msvc_arch,
            target_msvc_vlen,
            target_clan_arch,
            target_clang_isaarch,
            target_clang_cpu,
            target_clang_tune_cpu: Some("".to_string()),
            target_clang_vlen,
            target_clang_extraargs,
            target_clang_triple: Some(target_clang_triple),
            target_os_level: Some(target_os_level),
            target_runtime_level: Some(target_runtime_level),
            target_is_simulator: false,
            features: map,
        }
    }

    /// Evaluates user passed platform, arch, and TargetCpuArchitecture
    pub fn evaluate(
        platform: Option<Platform>,
        arch: Option<Arch>,
        target_cpu_str: Option<&str>,
    ) -> Result<Self, String> {
        Self::evaluate_full(platform, arch, target_cpu_str, None, None, None, None, None)
    }

    /// Evaluates user passed platform, arch, TargetCpuArchitecture, and optional requested vector length
    pub fn evaluate_with_vl(
        platform: Option<Platform>,
        arch: Option<Arch>,
        target_cpu_str: Option<&str>,
        requested_vl: Option<CpuArchitectureVectorLength>,
    ) -> Result<Self, String> {
        Self::evaluate_full(platform, arch, target_cpu_str, None, None, None, None, requested_vl)
    }

    /// Evaluates user passed platform, arch, TargetCpuArchitecture, target tune CPU, minimum CPU architecture, enabled/disabled extensions, and optional requested vector length
    pub fn evaluate_full(
        platform: Option<Platform>,
        arch: Option<Arch>,
        target_cpu_str: Option<&str>,
        target_tune_cpu_str: Option<&str>,
        min_cpu_arch_str: Option<&str>,
        enabled_ext_str: Option<&str>,
        disabled_ext_str: Option<&str>,
        requested_vl: Option<CpuArchitectureVectorLength>,
    ) -> Result<Self, String> {
        Self::evaluate_target_triple(
            platform,
            arch,
            target_cpu_str,
            target_tune_cpu_str,
            min_cpu_arch_str,
            enabled_ext_str,
            disabled_ext_str,
            requested_vl,
            None,
            None,
            false,
        )
    }

    /// Evaluates configuration including target OS level, target runtime level, and target simulator option
    pub fn evaluate_target_triple(
        platform: Option<Platform>,
        arch: Option<Arch>,
        target_cpu_str: Option<&str>,
        target_tune_cpu_str: Option<&str>,
        min_cpu_arch_str: Option<&str>,
        enabled_ext_str: Option<&str>,
        disabled_ext_str: Option<&str>,
        requested_vl: Option<CpuArchitectureVectorLength>,
        target_os_level: Option<&str>,
        target_runtime_level: Option<&str>,
        target_is_simulator: bool,
    ) -> Result<Self, String> {
        let p = match platform {
            Some(plat) => plat.resolve(),
            None => Platform::current(),
        };
        let a = match arch {
            Some(arch_val) => arch_val.resolve(),
            None => match p {
                Platform::Switch2 | Platform::Ios | Platform::Tvos | Platform::Xros => Arch::Arm64,
                Platform::Xboxone | Platform::Xboxxs | Platform::Ps4 | Platform::Ps5 | Platform::Steamdeck | Platform::Steammachine => Arch::X86_64,
                _ => {
                    if p.is_arch_compatible(Arch::current()) {
                        Arch::current()
                    } else if p.is_arch_compatible(Arch::X86_64) {
                        Arch::X86_64
                    } else if p.is_arch_compatible(Arch::Arm64) {
                        Arch::Arm64
                    } else {
                        Arch::Riscv64
                    }
                }
            },
        };

        if !p.is_arch_compatible(a) {
            return Err(format!("Architecture {} is incompatible with platform {}", a, p));
        }

        let target_str_norm = if let Some(console_target) = p.console_target_cpu() {
            Some(console_target.to_string())
        } else {
            target_cpu_str
                .map(|s| s.trim().to_ascii_lowercase())
                .filter(|s| !s.is_empty())
        };

        let target_tune_norm = target_tune_cpu_str
            .map(|s| s.trim().to_ascii_lowercase())
            .filter(|s| !s.is_empty());

        let min_arch_norm = min_cpu_arch_str
            .map(|s| s.trim().to_ascii_lowercase())
            .filter(|s| !s.is_empty());

        match target_str_norm.as_deref() {
            Some("native") => {
                if a == Arch::current() {
                    if enabled_ext_str.is_some() || disabled_ext_str.is_some() {
                        eprintln!(
                            "\x1b[33mwarning: adding or disabling extensions for non-generic CPU target 'native' is ignored\x1b[0m"
                        );
                    }
                    let features = CPUFeatures::detect_host();
                    let mut report = Self::from_host_with_vl(&features, requested_vl);
                    report.platform = p.to_string();
                    if !p.uses_msvc() {
                        report.target_msvc_arch = Some("".to_string());
                        report.target_msvc_vlen = Some("".to_string());
                    }
                    report.arch = a.to_string();
                    report.target_cpu = Some("native".to_string());
                    report.target_tune_cpu = Some("".to_string());
                    report.target_clang_tune_cpu = Some("".to_string());
                    let (triple, os_lvl, rt_lvl) = compute_target_clang_triple(
                        p,
                        a,
                        target_os_level,
                        target_runtime_level,
                        target_is_simulator,
                    )?;
                    report.target_clang_triple = Some(triple);
                    report.target_os_level = Some(os_lvl);
                    report.target_runtime_level = Some(rt_lvl);
                    report.target_is_simulator = target_is_simulator;
                    Ok(report)
                } else {
                    // Cannot run host instruction detection on a foreign architecture -> fallback to generic
                    let mut report = Self::from_target_cpu_triple(
                        p,
                        a,
                        "generic",
                        None,
                        min_arch_norm.as_deref(),
                        enabled_ext_str,
                        disabled_ext_str,
                        requested_vl,
                        target_os_level,
                        target_runtime_level,
                        target_is_simulator,
                    )?;
                    report.target_tune_cpu = Some("".to_string());
                    report.target_clang_tune_cpu = Some("".to_string());
                    Ok(report)
                }
            }
            Some(target_name) => {
                Self::from_target_cpu_triple(
                    p,
                    a,
                    target_name,
                    target_tune_norm.as_deref(),
                    min_arch_norm.as_deref(),
                    enabled_ext_str,
                    disabled_ext_str,
                    requested_vl,
                    target_os_level,
                    target_runtime_level,
                    target_is_simulator,
                )
            }
            None => {
                if p == Platform::current() && a == Arch::current() && min_arch_norm.is_none() && enabled_ext_str.is_none() && disabled_ext_str.is_none() {
                    let features = CPUFeatures::detect_host();
                    let mut report = Self::from_host_with_vl(&features, requested_vl);
                    report.platform = p.to_string();
                    report.arch = a.to_string();
                    report.target_cpu = Some("native".to_string());
                    report.target_tune_cpu = Some("".to_string());
                    report.target_clang_tune_cpu = Some("".to_string());
                    let (triple, os_lvl, rt_lvl) = compute_target_clang_triple(
                        p,
                        a,
                        target_os_level,
                        target_runtime_level,
                        target_is_simulator,
                    )?;
                    report.target_clang_triple = Some(triple);
                    report.target_os_level = Some(os_lvl);
                    report.target_runtime_level = Some(rt_lvl);
                    report.target_is_simulator = target_is_simulator;
                    Ok(report)
                } else if min_arch_norm.is_some() || enabled_ext_str.is_some() || disabled_ext_str.is_some() {
                    Self::from_target_cpu_triple(
                        p,
                        a,
                        "generic",
                        target_tune_norm.as_deref(),
                        min_arch_norm.as_deref(),
                        enabled_ext_str,
                        disabled_ext_str,
                        requested_vl,
                        target_os_level,
                        target_runtime_level,
                        target_is_simulator,
                    )
                } else {
                    let mut report = Self::from_target_with_options(
                        p,
                        a,
                        requested_vl,
                        target_os_level,
                        target_runtime_level,
                        target_is_simulator,
                    )?;
                    if let Some(tune_s) = target_tune_norm.as_deref() {
                        match a {
                            Arch::X86_64 => {
                                let tune_x64: TargetCpuArchitectureX64 = tune_s.parse()?;
                                if tune_x64 == TargetCpuArchitectureX64::Native {
                                    return Err("Native target is not allowed for target_tune_cpu".to_string());
                                }
                                let tune_str = TargetCpuArchitectureX64Names::name(tune_x64).to_string();
                                report.target_tune_cpu = Some(tune_str.clone());
                                report.target_clang_tune_cpu = Some(format!("-m'tune={}'", tune_str));
                            }
                            Arch::Arm64 | Arch::Arm64EC | Arch::Arm64E => {
                                let tune_arm: TargetCpuArchitectureArm64 = tune_s.parse()?;
                                if tune_arm == TargetCpuArchitectureArm64::Native {
                                    return Err("Native target is not allowed for target_tune_cpu".to_string());
                                }
                                if !p.is_target_arm64_compatible(tune_arm) {
                                    return Err(format!(
                                        "Target tune CPU {} is incompatible with platform {}",
                                        tune_s, p
                                    ));
                                }
                                let tune_str = TargetCpuArchitectureArm64Names::name(tune_arm).to_string();
                                report.target_tune_cpu = Some(tune_str.clone());
                                report.target_clang_tune_cpu = Some(format!("-m'tune={}'", tune_str));
                            }
                            Arch::Riscv64 => {
                                let tune_rv: TargetCpuArchitectureRiscv64 = tune_s.parse()?;
                                if tune_rv == TargetCpuArchitectureRiscv64::Native {
                                    return Err("Native target is not allowed for target_tune_cpu".to_string());
                                }
                                let tune_str = TargetCpuArchitectureRiscv64Names::name(tune_rv).to_string();
                                report.target_tune_cpu = Some(tune_str.clone());
                                report.target_clang_tune_cpu = Some(format!("-m'tune={}'", tune_str));
                            }
                            Arch::Native => unreachable!(),
                        }
                    }
                    Ok(report)
                }
            }
        }
    }

    /// Creates a report for a specific known TargetCpuArchitecture string
    pub fn from_target_cpu(platform: Platform, arch: Arch, target_cpu_name: &str) -> Result<Self, String> {
        Self::from_target_cpu_full(platform, arch, target_cpu_name, None, None, None, None, None)
    }

    /// Creates a report for a specific known TargetCpuArchitecture string with optional vector length
    pub fn from_target_cpu_with_vl(
        platform: Platform,
        arch: Arch,
        target_cpu_name: &str,
        requested_vl: Option<CpuArchitectureVectorLength>,
    ) -> Result<Self, String> {
        Self::from_target_cpu_full(platform, arch, target_cpu_name, None, None, None, None, requested_vl)
    }

    /// Creates a report for a specific known TargetCpuArchitecture string with optional target tune CPU, minimum architecture, enabled/disabled extensions, and vector length
    pub fn from_target_cpu_full(
        platform: Platform,
        arch: Arch,
        target_cpu_name: &str,
        target_tune_cpu_str: Option<&str>,
        min_cpu_arch_str: Option<&str>,
        enabled_ext_str: Option<&str>,
        disabled_ext_str: Option<&str>,
        requested_vl: Option<CpuArchitectureVectorLength>,
    ) -> Result<Self, String> {
        Self::from_target_cpu_triple(
            platform,
            arch,
            target_cpu_name,
            target_tune_cpu_str,
            min_cpu_arch_str,
            enabled_ext_str,
            disabled_ext_str,
            requested_vl,
            None,
            None,
            false,
        )
    }

    /// Creates a report for a specific known TargetCpuArchitecture string with optional target options and triple levels
    pub fn from_target_cpu_triple(
        platform: Platform,
        arch: Arch,
        target_cpu_name: &str,
        target_tune_cpu_str: Option<&str>,
        min_cpu_arch_str: Option<&str>,
        enabled_ext_str: Option<&str>,
        disabled_ext_str: Option<&str>,
        requested_vl: Option<CpuArchitectureVectorLength>,
        target_os_level: Option<&str>,
        target_runtime_level: Option<&str>,
        target_is_simulator: bool,
    ) -> Result<Self, String> {
        let platform = platform.resolve();
        let arch = arch.resolve();
        if !platform.is_arch_compatible(arch) {
            return Err(format!(
                "Architecture {} is incompatible with platform {}",
                arch, platform
            ));
        }

        let (target_clang_triple, target_os_level_val, target_runtime_level_val) = compute_target_clang_triple(
            platform,
            arch,
            target_os_level,
            target_runtime_level,
            target_is_simulator,
        )?;

        let target_cpu_name = if let Some(console_target) = platform.console_target_cpu() {
            console_target
        } else {
            target_cpu_name
        };

        match arch {
            Arch::X86_64 => {
                let target_x64: TargetCpuArchitectureX64 = target_cpu_name.parse()?;
                let is_generic = target_x64 == TargetCpuArchitectureX64::Generic || target_x64 == TargetCpuArchitectureX64::None;

                let enabled_tokens = enabled_ext_str.map(parse_extension_tokens).unwrap_or_default();
                let disabled_tokens = disabled_ext_str.map(parse_extension_tokens).unwrap_or_default();

                let mut enabled_isas = Vec::new();
                for token in enabled_tokens {
                    let isa = token.parse::<X64ISA>()?;
                    if !enabled_isas.contains(&isa) {
                        enabled_isas.push(isa);
                    }
                }

                let mut disabled_isas = Vec::new();
                for token in disabled_tokens {
                    let isa = token.parse::<X64ISA>()?;
                    if !disabled_isas.contains(&isa) {
                        disabled_isas.push(isa);
                    }
                }

                let (mut feat, target_name_str, min_enum) = if is_generic {
                    let min_x64 = if let Some(m_str) = min_cpu_arch_str {
                        m_str.parse::<MinimumCpuArchitectureX64>()?
                    } else {
                        MinimumCpuArchitectureX64::None
                    };
                    let f = X64CPUFeatures::from_min_arch(min_x64);
                    let target_str = match min_x64 {
                        MinimumCpuArchitectureX64::None | MinimumCpuArchitectureX64::AVX => "x86-64-v2",
                        MinimumCpuArchitectureX64::AVX2 | MinimumCpuArchitectureX64::AVX10_1 | MinimumCpuArchitectureX64::AVX10_2 => "x86-64-v3",
                        MinimumCpuArchitectureX64::AVX512 => "x86-64-v4",
                    };
                    (f, target_str.to_string(), min_x64)
                } else {
                    let f = X64CPUFeatures::from_target(target_x64);
                    let target_str = TargetCpuArchitectureX64Names::name(target_x64).to_string();
                    let min_e = f.minimum_architecture();
                    (f, target_str, min_e)
                };

                let active_disabled: Vec<X64ISA>;
                if is_generic {
                    for &isa in &enabled_isas {
                        if !disabled_isas.contains(&isa) {
                            feat.set_feature(isa, true);
                        }
                    }
                    for &isa in &disabled_isas {
                        feat.set_feature(isa, false);
                    }
                    active_disabled = disabled_isas;
                } else {
                    if !enabled_isas.is_empty() || !disabled_isas.is_empty() {
                        eprintln!(
                            "\x1b[33mwarning: adding or disabling extensions for non-generic CPU target '{}' is ignored\x1b[0m",
                            target_cpu_name
                        );
                    }
                    active_disabled = Vec::new();
                }

                let extensions = if !is_generic && target_x64 != TargetCpuArchitectureX64::Native {
                    let e = x86_64::ClangTargetCpuArchitectureX64ISANames::name(target_x64);
                    if e.is_empty() {
                        feat.to_extensions_string()
                    } else {
                        e.to_string()
                    }
                } else {
                    feat.to_extensions_string()
                };

                let effective_min_enum = if is_generic && min_cpu_arch_str.is_none() {
                    feat.minimum_architecture()
                } else {
                    min_enum
                };

                let min_arch = effective_min_enum.to_string();
                let effective_vl = match requested_vl {
                    None | Some(CpuArchitectureVectorLength::None) => platform.default_vector_length(),
                    other => other,
                };
                let vl_enum = effective_min_enum.resolve_vector_length(effective_vl);
                let vl = vl_enum.to_string();
                let (target_msvc_arch, target_msvc_vlen) = if platform.uses_msvc() {
                    (
                        Some(x86_64::MSVCX64ArchTarget::name(effective_min_enum).to_string()),
                        Some(x86_64::MSVCX64VLen::name(effective_min_enum, vl_enum).to_string()),
                    )
                } else {
                    (Some("".to_string()), Some("".to_string()))
                };
                let target_clan_arch = Some(format!("-m'arch={}'", target_name_str));
                let target_clang_isaarch = Some(feat.generate_clang_isaarch(vl_enum, &active_disabled));
                let target_clang_vlen = Some(x86_64::ClangX64VLen::name(effective_min_enum, vl_enum).to_string());
                let map = feat.to_map();

                let tune_target_str = if target_x64 == TargetCpuArchitectureX64::Native {
                    "".to_string()
                } else if let Some(tune_s) = target_tune_cpu_str {
                    let tune_x64: TargetCpuArchitectureX64 = tune_s.parse()?;
                    if tune_x64 == TargetCpuArchitectureX64::Native {
                        return Err("Native target is not allowed for target_tune_cpu".to_string());
                    }
                    TargetCpuArchitectureX64Names::name(tune_x64).to_string()
                } else {
                    target_name_str.clone()
                };

                let target_clang_tune_cpu = if tune_target_str.is_empty() {
                    Some("".to_string())
                } else {
                    Some(format!("-m'tune={}'", tune_target_str))
                };

                Ok(Self {
                    platform: platform.to_string(),
                    arch: arch.to_string(),
                    extensions,
                    target_cpu: Some(target_name_str),
                    target_tune_cpu: Some(tune_target_str),
                    min_cpu_arch: Some(min_arch),
                    vector_length: Some(vl),
                    target_msvc_arch,
                    target_msvc_vlen,
                    target_clan_arch,
                    target_clang_isaarch,
                    target_clang_cpu: Some("".to_string()),
                    target_clang_tune_cpu,
                    target_clang_vlen,
                    target_clang_extraargs: Some("".to_string()),
                    target_clang_triple: Some(target_clang_triple.clone()),
                    target_os_level: Some(target_os_level_val.clone()),
                    target_runtime_level: Some(target_runtime_level_val.clone()),
                    target_is_simulator,
                    features: map,
                })
            }

            Arch::Arm64 | Arch::Arm64EC | Arch::Arm64E => {
                let target_arm64: TargetCpuArchitectureArm64 = target_cpu_name.parse()?;
                if !platform.is_target_arm64_compatible(target_arm64) {
                    return Err(format!(
                        "Target CPU {} is incompatible with platform {}",
                        target_cpu_name, platform
                    ));
                }
                let is_generic = target_arm64 == TargetCpuArchitectureArm64::Generic || target_arm64 == TargetCpuArchitectureArm64::None;

                let enabled_tokens = enabled_ext_str.map(parse_extension_tokens).unwrap_or_default();
                let disabled_tokens = disabled_ext_str.map(parse_extension_tokens).unwrap_or_default();

                let mut enabled_isas = Vec::new();
                for token in enabled_tokens {
                    let isa = token.parse::<Arm64ISA>()?;
                    if !enabled_isas.contains(&isa) {
                        enabled_isas.push(isa);
                    }
                }

                let mut disabled_isas = Vec::new();
                for token in disabled_tokens {
                    let isa = token.parse::<Arm64ISA>()?;
                    if !disabled_isas.contains(&isa) {
                        disabled_isas.push(isa);
                    }
                }

                let (mut feat, target_name_str, min_enum) = if is_generic {
                    let min_arm = if let Some(m_str) = min_cpu_arch_str {
                        m_str.parse::<MinimumCpuArchitectureArm64>()?
                    } else {
                        MinimumCpuArchitectureArm64::ARMv8_A
                    };
                    let f = Arm64CPUFeatures::from_min_arch(min_arm);
                    let target_str = "generic".to_string();
                    (f, target_str, min_arm)
                } else {
                    let f = Arm64CPUFeatures::from_target(target_arm64);
                    let target_str = TargetCpuArchitectureArm64Names::name(target_arm64).to_string();
                    let min_e = f.minimum_architecture();
                    (f, target_str, min_e)
                };

                let mut active_disabled = if is_generic {
                    for &isa in &enabled_isas {
                        if !disabled_isas.contains(&isa) {
                            feat.set_feature(isa, true);
                        }
                    }
                    for &isa in &disabled_isas {
                        feat.set_feature(isa, false);
                    }
                    disabled_isas
                } else {
                    if !enabled_isas.is_empty() || !disabled_isas.is_empty() {
                        eprintln!(
                            "\x1b[33mwarning: adding or disabling extensions for non-generic CPU target '{}' is ignored\x1b[0m",
                            target_cpu_name
                        );
                    }
                    Vec::new()
                };

                for isa in get_target_arm64_disabled_isas(target_arm64) {
                    if !active_disabled.contains(&isa) {
                        active_disabled.push(isa);
                    }
                }

                let extensions = if !is_generic && target_arm64 != TargetCpuArchitectureArm64::Native {
                    let e = arm64::ClangTargetCpuArchitectureArm64ISANames::name(target_arm64);
                    if e.is_empty() {
                        feat.to_extensions_string()
                    } else {
                        e.to_string()
                    }
                } else {
                    feat.to_extensions_string()
                };

                let effective_min_enum = if is_generic && min_cpu_arch_str.is_none() {
                    feat.minimum_architecture()
                } else {
                    min_enum
                };

                let min_arch = effective_min_enum.to_string();
                let vl = feat.resolve_vector_length(requested_vl).to_string();
                let (target_msvc_arch, target_msvc_vlen) = if platform.uses_msvc() {
                    let msvc_min = arm64::MinimumCpuArchitectureArm64MSVCNames::name(effective_min_enum);
                    let msvc_isa = arm64::MSVCTargetCpuArchitectureArm64ISANames::name(target_arm64);
                    let arch_str = if msvc_isa.is_empty() {
                        format!("/arch:{}", msvc_min)
                    } else {
                        format!("/arch:{}+{}", msvc_min, msvc_isa)
                    };
                    (Some(arch_str), Some("".to_string()))
                } else {
                    (Some("".to_string()), Some("".to_string()))
                };
                let target_clan_arch = Some(feat.generate_clang_isaarch(effective_min_enum, &active_disabled));
                let target_clang_isaarch = Some("".to_string());
                let target_clang_cpu = Some(format!("-m'cpu={}'", target_name_str));
                let map = feat.to_map();

                let tune_target_str = if target_arm64 == TargetCpuArchitectureArm64::Native {
                    "".to_string()
                } else if let Some(tune_s) = target_tune_cpu_str {
                    let tune_arm: TargetCpuArchitectureArm64 = tune_s.parse()?;
                    if tune_arm == TargetCpuArchitectureArm64::Native {
                        return Err("Native target is not allowed for target_tune_cpu".to_string());
                    }
                    if !platform.is_target_arm64_compatible(tune_arm) {
                        return Err(format!(
                            "Target tune CPU {} is incompatible with platform {}",
                            tune_s, platform
                        ));
                    }
                    TargetCpuArchitectureArm64Names::name(tune_arm).to_string()
                } else {
                    target_name_str.clone()
                };

                let target_clang_tune_cpu = if tune_target_str.is_empty() {
                    Some("".to_string())
                } else {
                    Some(format!("-m'tune={}'", tune_target_str))
                };

                Ok(Self {
                    platform: platform.to_string(),
                    arch: arch.to_string(),
                    extensions,
                    target_cpu: Some(target_name_str),
                    target_tune_cpu: Some(tune_target_str),
                    min_cpu_arch: Some(min_arch),
                    vector_length: Some(vl),
                    target_msvc_arch,
                    target_msvc_vlen,
                    target_clan_arch,
                    target_clang_isaarch,
                    target_clang_cpu,
                    target_clang_tune_cpu,
                    target_clang_vlen: Some("-m'prefer-vector-width=128'".to_string()),
                    target_clang_extraargs: Some("".to_string()),
                    target_clang_triple: Some(target_clang_triple.clone()),
                    target_os_level: Some(target_os_level_val.clone()),
                    target_runtime_level: Some(target_runtime_level_val.clone()),
                    target_is_simulator,
                    features: map,
                })
            }

            Arch::Riscv64 => {
                let target_riscv: TargetCpuArchitectureRiscv64 = target_cpu_name.parse()?;
                let is_generic = target_riscv == TargetCpuArchitectureRiscv64::Generic || target_riscv == TargetCpuArchitectureRiscv64::None;

                let enabled_tokens = enabled_ext_str.map(parse_extension_tokens).unwrap_or_default();
                let disabled_tokens = disabled_ext_str.map(parse_extension_tokens).unwrap_or_default();

                let mut enabled_isas = Vec::new();
                for token in enabled_tokens {
                    let isa = token.parse::<Riscv64ISA>()?;
                    if !enabled_isas.contains(&isa) {
                        enabled_isas.push(isa);
                    }
                }

                let mut disabled_isas = Vec::new();
                for token in disabled_tokens {
                    let isa = token.parse::<Riscv64ISA>()?;
                    if !disabled_isas.contains(&isa) {
                        disabled_isas.push(isa);
                    }
                }

                let mut feat = Riscv64CPUFeatures::from_target(target_riscv);

                let active_disabled: Vec<Riscv64ISA>;
                if is_generic {
                    for &isa in &enabled_isas {
                        if !disabled_isas.contains(&isa) {
                            feat.set_feature(isa, true);
                        }
                    }
                    for &isa in &disabled_isas {
                        feat.set_feature(isa, false);
                    }
                    active_disabled = disabled_isas;
                } else {
                    if !enabled_isas.is_empty() || !disabled_isas.is_empty() {
                        eprintln!(
                            "\x1b[33mwarning: adding or disabling extensions for non-generic CPU target '{}' is ignored\x1b[0m",
                            target_cpu_name
                        );
                    }
                    active_disabled = Vec::new();
                }

                let extensions = if !is_generic && target_riscv != TargetCpuArchitectureRiscv64::Native {
                    let ext = riscv64::ClangTargetCpuArchitectureRiscv64ISANames::name(target_riscv);
                    if ext.is_empty() {
                        feat.to_extensions_string()
                    } else {
                        ext.to_string()
                    }
                } else {
                    feat.to_extensions_string()
                };

                let target_name_str = TargetCpuArchitectureRiscv64Names::name(target_riscv).to_string();
                let vl = feat.resolve_vector_length(requested_vl).to_string();
                let target_clan_arch = Some(feat.generate_clang_arch(&active_disabled));
                let target_clang_isaarch = Some(feat.generate_clang_isaarch(&active_disabled));
                let target_clang_cpu = Some(format!("-m'cpu={}'", target_name_str));
                let map = feat.to_map();

                let has_experimental = if is_generic {
                    enabled_isas
                        .iter()
                        .any(|i| i.is_experimental() && !active_disabled.contains(i))
                        || parse_extension_tokens(&extensions)
                            .iter()
                            .any(|t| t.parse::<Riscv64ISA>().map(|i| i.is_experimental() && !active_disabled.contains(&i)).unwrap_or(false))
                } else {
                    parse_extension_tokens(&extensions)
                        .iter()
                        .any(|t| t.parse::<Riscv64ISA>().map(|i| i.is_experimental()).unwrap_or(false))
                };
                let target_clang_extraargs = if has_experimental {
                    Some("-m'enable-experimental-extensions'".to_string())
                } else {
                    Some("".to_string())
                };

                let tune_target_str = if target_riscv == TargetCpuArchitectureRiscv64::Native {
                    "".to_string()
                } else if let Some(tune_s) = target_tune_cpu_str {
                    let tune_rv: TargetCpuArchitectureRiscv64 = tune_s.parse()?;
                    if tune_rv == TargetCpuArchitectureRiscv64::Native {
                        return Err("Native target is not allowed for target_tune_cpu".to_string());
                    }
                    TargetCpuArchitectureRiscv64Names::name(tune_rv).to_string()
                } else {
                    target_name_str.clone()
                };

                let target_clang_tune_cpu = if tune_target_str.is_empty() {
                    Some("".to_string())
                } else {
                    Some(format!("-m'tune={}'", tune_target_str))
                };

                Ok(Self {
                    platform: platform.to_string(),
                    arch: arch.to_string(),
                    extensions,
                    target_cpu: Some(target_name_str),
                    target_tune_cpu: Some(tune_target_str),
                    min_cpu_arch: Some("none".to_string()),
                    vector_length: Some(vl),
                    target_msvc_arch: Some("".to_string()),
                    target_msvc_vlen: Some("".to_string()),
                    target_clan_arch,
                    target_clang_isaarch,
                    target_clang_cpu,
                    target_clang_tune_cpu,
                    target_clang_vlen: Some("-m'prefer-vector-width=128'".to_string()),
                    target_clang_extraargs,
                    target_clang_triple: Some(target_clang_triple),
                    target_os_level: Some(target_os_level_val),
                    target_runtime_level: Some(target_runtime_level_val),
                    target_is_simulator,
                    features: map,
                })
            }
            Arch::Native => unreachable!(),
        }
    }

    /// Creates a report for a specific platform and architecture configuration
    pub fn from_target(platform: Platform, arch: Arch) -> Result<Self, String> {
        Self::from_target_with_vl(platform, arch, None)
    }

    /// Creates a report for a specific platform and architecture configuration with optional vector length
    /// Creates a report for a specific platform and architecture configuration with optional vector length
    pub fn from_target_with_vl(
        platform: Platform,
        arch: Arch,
        requested_vl: Option<CpuArchitectureVectorLength>,
    ) -> Result<Self, String> {
        Self::from_target_with_options(platform, arch, requested_vl, None, None, false)
    }

    /// Creates a report for a specific platform and architecture configuration with target options
    pub fn from_target_with_options(
        platform: Platform,
        arch: Arch,
        requested_vl: Option<CpuArchitectureVectorLength>,
        target_os_level: Option<&str>,
        target_runtime_level: Option<&str>,
        target_is_simulator: bool,
    ) -> Result<Self, String> {
        let platform = platform.resolve();
        let arch = arch.resolve();
        let (extensions, x64_target, arm64_target) = TargetProfile::get_features(platform, arch)?;

        let (target_clang_triple, target_os_level_val, target_runtime_level_val) = compute_target_clang_triple(
            platform,
            arch,
            target_os_level,
            target_runtime_level,
            target_is_simulator,
        )?;

        match arch {
            Arch::X86_64 => {
                let features = X64CPUFeatures::from_extensions_str(&extensions);
                let target_cpu = TargetCpuArchitectureX64Names::name(x64_target);
                let min_enum = features.minimum_architecture();
                let min_arch = min_enum.to_string();
                let effective_vl = match requested_vl {
                    None | Some(CpuArchitectureVectorLength::None) => platform.default_vector_length(),
                    other => other,
                };
                let vl_enum = features.resolve_vector_length(effective_vl);
                let vl = vl_enum.to_string();
                let (target_msvc_arch, target_msvc_vlen) = if platform.uses_msvc() {
                    (
                        Some(x86_64::MSVCX64ArchTarget::name(min_enum).to_string()),
                        Some(x86_64::MSVCX64VLen::name(min_enum, vl_enum).to_string()),
                    )
                } else {
                    (Some("".to_string()), Some("".to_string()))
                };
                let target_clan_arch = Some(format!("-m'arch={}'", target_cpu));
                let target_clang_isaarch = Some(features.generate_clang_isaarch(vl_enum, &[]));
                let target_clang_vlen = Some(x86_64::ClangX64VLen::name(min_enum, vl_enum).to_string());
                let map = features.to_map();

                let tune_target_str = if x64_target == TargetCpuArchitectureX64::Native {
                    "".to_string()
                } else {
                    target_cpu.to_string()
                };
                let target_clang_tune_cpu = if tune_target_str.is_empty() {
                    Some("".to_string())
                } else {
                    Some(format!("-m'tune={}'", tune_target_str))
                };

                Ok(Self {
                    platform: platform.to_string(),
                    arch: arch.to_string(),
                    extensions,
                    target_cpu: Some(target_cpu.to_string()),
                    target_tune_cpu: Some(tune_target_str),
                    min_cpu_arch: Some(min_arch),
                    vector_length: Some(vl),
                    target_msvc_arch,
                    target_msvc_vlen,
                    target_clan_arch,
                    target_clang_isaarch,
                    target_clang_cpu: Some("".to_string()),
                    target_clang_tune_cpu,
                    target_clang_vlen,
                    target_clang_extraargs: Some("".to_string()),
                    target_clang_triple: Some(target_clang_triple),
                    target_os_level: Some(target_os_level_val),
                    target_runtime_level: Some(target_runtime_level_val),
                    target_is_simulator,
                    features: map,
                })
            }

            Arch::Arm64 | Arch::Arm64EC | Arch::Arm64E => {
                let features = Arm64CPUFeatures::from_extensions_str(&extensions);
                let target_cpu = TargetCpuArchitectureArm64Names::name(arm64_target);
                let min_enum = features.minimum_architecture();
                let min_arch = min_enum.to_string();
                let vl = features.resolve_vector_length(requested_vl).to_string();
                let (target_msvc_arch, target_msvc_vlen) = if platform.uses_msvc() {
                    let msvc_min = arm64::MinimumCpuArchitectureArm64MSVCNames::name(min_enum);
                    let msvc_isa = arm64::MSVCTargetCpuArchitectureArm64ISANames::name(arm64_target);
                    let arch_str = if msvc_isa.is_empty() {
                        format!("/arch:{}", msvc_min)
                    } else {
                        format!("/arch:{}+{}", msvc_min, msvc_isa)
                    };
                    (Some(arch_str), Some("".to_string()))
                } else {
                    (Some("".to_string()), Some("".to_string()))
                };
                let disabled_isas = get_target_arm64_disabled_isas(arm64_target);
                let target_clan_arch = Some(features.generate_clang_isaarch(min_enum, &disabled_isas));
                let target_clang_isaarch = Some("".to_string());
                let target_clang_cpu = Some(format!("-m'cpu={}'", target_cpu));
                let map = features.to_map();

                let tune_target_str = if arm64_target == TargetCpuArchitectureArm64::Native {
                    "".to_string()
                } else {
                    target_cpu.to_string()
                };
                let target_clang_tune_cpu = if tune_target_str.is_empty() {
                    Some("".to_string())
                } else {
                    Some(format!("-m'tune={}'", tune_target_str))
                };

                Ok(Self {
                    platform: platform.to_string(),
                    arch: arch.to_string(),
                    extensions,
                    target_cpu: Some(target_cpu.to_string()),
                    target_tune_cpu: Some(tune_target_str),
                    min_cpu_arch: Some(min_arch),
                    vector_length: Some(vl),
                    target_msvc_arch,
                    target_msvc_vlen,
                    target_clan_arch,
                    target_clang_isaarch,
                    target_clang_cpu,
                    target_clang_tune_cpu,
                    target_clang_vlen: Some("-m'prefer-vector-width=128'".to_string()),
                    target_clang_extraargs: Some("".to_string()),
                    target_clang_triple: Some(target_clang_triple),
                    target_os_level: Some(target_os_level_val),
                    target_runtime_level: Some(target_runtime_level_val),
                    target_is_simulator,
                    features: map,
                })
            }

            Arch::Riscv64 => {
                let features = Riscv64CPUFeatures::from_extensions_str(&extensions);
                let vl = features.resolve_vector_length(requested_vl).to_string();
                let target_clan_arch = Some(features.generate_clang_arch(&[]));
                let target_clang_isaarch = Some(features.generate_clang_isaarch(&[]));
                let target_clang_cpu = Some("-m'cpu=generic-rv64'".to_string());
                let map = features.to_map();

                let has_experimental = parse_extension_tokens(&extensions)
                    .iter()
                    .any(|t| t.parse::<Riscv64ISA>().map(|i| i.is_experimental()).unwrap_or(false));
                let target_clang_extraargs = if has_experimental {
                    Some("-m'enable-experimental-extensions'".to_string())
                } else {
                    Some("".to_string())
                };

                let target_cpu = "generic-rv64".to_string();
                let target_clang_tune_cpu = Some(format!("-m'tune={}'", target_cpu));

                Ok(Self {
                    platform: platform.to_string(),
                    arch: arch.to_string(),
                    extensions,
                    target_cpu: Some(target_cpu.clone()),
                    target_tune_cpu: Some(target_cpu),
                    min_cpu_arch: Some("none".to_string()),
                    vector_length: Some(vl),
                    target_msvc_arch: Some("".to_string()),
                    target_msvc_vlen: Some("".to_string()),
                    target_clan_arch,
                    target_clang_isaarch,
                    target_clang_cpu,
                    target_clang_tune_cpu,
                    target_clang_vlen: Some("-m'prefer-vector-width=128'".to_string()),
                    target_clang_extraargs,
                    target_clang_triple: Some(target_clang_triple),
                    target_os_level: Some(target_os_level_val),
                    target_runtime_level: Some(target_runtime_level_val),
                    target_is_simulator,
                    features: map,
                })
            }
            Arch::Native => unreachable!(),
        }
    }

    /// Serializes report to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Writes report to a file, automatically creating parent directories if needed
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = self.to_json().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if let Some(parent) = path.as_ref().parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

/// Full Matrix Platform & Architecture Testing Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformArchMatrixReport {
    pub matrix: Vec<ArchFeaturesReport>,
}

impl PlatformArchMatrixReport {
    /// Generates full compatibility matrix across all platforms & architectures
    pub fn generate() -> Self {
        let mut matrix = Vec::new();
        for &platform in Platform::all() {
            for &arch in Arch::all() {
                if platform.is_arch_compatible(arch) {
                    if let Ok(report) = ArchFeaturesReport::from_target(platform, arch) {
                        matrix.push(report);
                    }
                }
            }
        }
        Self { matrix }
    }

    /// Saves all matrix reports as individual files in the given directory using the canonical filename pattern
    pub fn save_all_to_dir<P: AsRef<Path>>(&self, dir: P) -> std::io::Result<Vec<PathBuf>> {
        let dir_path = dir.as_ref();
        fs::create_dir_all(dir_path)?;
        let mut saved_paths = Vec::new();

        for report in &self.matrix {
            let filename = report.filename();
            let file_path = dir_path.join(filename);
            report.save_to_file(&file_path)?;
            saved_paths.push(file_path);
        }

        Ok(saved_paths)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let json = self.to_json().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if let Some(parent) = path.as_ref().parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
