// Copyright Epic Games, Inc. All Rights Reserved.

use archinfo::{
    get_default_output_dir, Arch, ArchFeaturesReport, CpuArchitectureVectorLength,
    Platform, PlatformArchMatrixReport,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "archinfo",
    author = "Epic Games, Inc.",
    version = "0.1.0",
    about = "Target CPU Architecture & Hardware Instruction Set Probing Tool for Unreal Engine"
)]
struct Cli {
    /// Target operating system platform (native, windows, linux, mac, ios, android, ps5, switch, xboxxs, etc.)
    #[arg(short = 'p', long)]
    platform: Option<String>,

    /// Target hardware architecture (native, x86_64, arm64, riscv64)
    #[arg(short = 'a', long)]
    arch: Option<String>,

    /// Target microarchitecture (native, generic, znver3, alderlake, apple-m4, cortex-a78c, etc.)
    #[arg(short = 't', long = "target", visible_alias = "target-cpu", alias = "target_cpu")]
    target: Option<String>,

    /// Target tune CPU microarchitecture (generic, znver3, alderlake, apple-m4, cortex-a78c, etc.)
    #[arg(short = 'u', long = "target-tune", visible_alias = "target-tune-cpu", alias = "tune-cpu", alias = "tune")]
    target_tune_cpu: Option<String>,

    /// Minimum CPU architecture baseline (AVX, AVX2, AVX512, AVX10.1, AVX10.2 for x86_64; ARMv8-A..ARMv9.7-A for arm64)
    #[arg(short = 'm', long = "min-cpu-arch", visible_alias = "min-arch", alias = "minimum-cpu-architecture")]
    min_cpu_arch: Option<String>,

    /// Enabled ISA extensions to include (Generic target only, e.g. "aes+sha2" or "avx512f,avx512vl")
    #[arg(short = 'e', long = "enable-extensions", visible_alias = "enabled-extensions", alias = "enable-ext")]
    enable_extensions: Option<String>,

    /// Disabled ISA extensions to exclude (Generic target only, e.g. "sse4.1" or "sve+sve2")
    #[arg(short = 'd', long = "disable-extensions", visible_alias = "disabled-extensions", alias = "disable-ext")]
    disable_extensions: Option<String>,

    /// Preferred vector length for SIMD code generation (128, 256, 512, vl128, vl256, vl512)
    #[arg(short = 'v', long = "vector-length", visible_alias = "vl", alias = "vector")]
    vector_length: Option<String>,

    /// Path to output JSON file or directory. If omitted, uses standard Unreal Engine/ArchInfo directory.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Save result using canonical filename '{platform}-{arch}-{targetcpu}-{mincpuarch}-{vectorlength}.json' in default or selected output folder
    #[arg(short, long)]
    save: bool,

    /// Generates full compatibility matrix across all platforms and architectures
    #[arg(short = 'M', long)]
    matrix: bool,

    /// Force host CPU detection
    #[arg(short = 'D', long)]
    detect: bool,

    /// Format to print to console (json, extensions, clang, msvc, filename, dir)
    #[arg(short, long, default_value = "json")]
    format: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let requested_vl = match cli.vector_length.as_deref() {
        Some(s) => Some(s.parse::<CpuArchitectureVectorLength>()?),
        None => None,
    };

    if cli.matrix {
        let matrix_report = PlatformArchMatrixReport::generate();
        let json_str = matrix_report.to_json()?;

        if cli.save {
            let out_dir = cli.output.unwrap_or_else(get_default_output_dir);
            let saved = matrix_report.save_all_to_dir(&out_dir)?;
            println!("Saved {} architecture profiles to {}", saved.len(), out_dir.display());
        } else if let Some(ref path) = cli.output {
            if path.is_dir() || path.to_string_lossy().ends_with('/') || path.to_string_lossy().ends_with('\\') {
                let saved = matrix_report.save_all_to_dir(path)?;
                println!("Saved {} architecture profiles to {}", saved.len(), path.display());
            } else {
                matrix_report.save_to_file(path)?;
                println!("Wrote platform-architecture matrix to {}", path.display());
            }
        } else {
            println!("{}", json_str);
        }
        return Ok(());
    }

    let platform = match cli.platform.as_deref() {
        Some(p) => Some(p.parse::<Platform>()?),
        None => None,
    };

    let arch = match cli.arch.as_deref() {
        Some(a) => Some(a.parse::<Arch>()?),
        None => None,
    };

    let target_opt = if cli.detect {
        Some("native")
    } else {
        cli.target.as_deref()
    };

    // When no arguments are passed, evaluate defaults to native host platform, host arch, and live CPUFeatures probing
    let report = ArchFeaturesReport::evaluate_full(
        platform,
        arch,
        target_opt,
        cli.target_tune_cpu.as_deref(),
        cli.min_cpu_arch.as_deref(),
        cli.enable_extensions.as_deref(),
        cli.disable_extensions.as_deref(),
        requested_vl,
    )?;

    // Console output based on format
    match cli.format.to_lowercase().as_str() {
        "dir" | "folder" => {
            println!("{}", get_default_output_dir().display());
        }
        "filepath" | "path" => {
            println!("{}", report.default_filepath().display());
        }
        "filename" | "name" => {
            println!("{}", report.filename());
        }
        "extensions" | "ext" => {
            println!("{}", report.extensions);
        }
        "clang" => {
            let mut flags = Vec::new();
            if let Some(ref cpu) = report.target_cpu {
                flags.push(format!("-mcpu={}", cpu));
            }
            if !report.extensions.is_empty() {
                for ext in report.extensions.split('+') {
                    flags.push(format!("-target-feature +{}", ext));
                }
            }
            if let Some(ref vlen) = report.target_clang_vlen {
                if !vlen.is_empty() {
                    flags.push(vlen.clone());
                }
            }
            println!("{}", flags.join(" "));
        }
        "msvc" => {
            let mut flags = Vec::new();
            if let Some(ref target_msvc) = report.target_msvc_arch {
                flags.push(target_msvc.clone());
            } else if let Some(ref min_arch) = report.min_cpu_arch {
                if let Ok(arch_enum) = min_arch.parse::<archinfo::MinimumCpuArchitectureX64>() {
                    let msvc_name = archinfo::MSVCX64ISANames::name(arch_enum);
                    flags.push(format!("/arch:{}", msvc_name));
                } else {
                    flags.push(format!("/arch:{}", min_arch.to_uppercase()));
                }
            } else {
                flags.push("/arch:AVX2".to_string());
            }
            if let Some(ref vlen) = report.target_msvc_vlen {
                if !vlen.is_empty() {
                    flags.push(vlen.clone());
                }
            }
            println!("{}", flags.join(" "));
        }
        _ => {
            let json_str = report.to_json()?;
            println!("{}", json_str);
        }
    }

    // Determine destination filename / path
    let out_path = if let Some(ref p) = cli.output {
        if p.is_dir() || p.to_string_lossy().ends_with('/') || p.to_string_lossy().ends_with('\\') {
            Some(p.join(report.filename()))
        } else {
            Some(p.clone())
        }
    } else if cli.save {
        Some(report.default_filepath())
    } else {
        None
    };

    if let Some(path) = out_path {
        report.save_to_file(&path)?;
        println!("Wrote report to {}", path.display());
    }

    Ok(())
}
