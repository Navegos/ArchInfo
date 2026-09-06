use archinfo::*;

#[test]
fn test_platform_architecture_matrix() {
    assert!(Platform::Windows.is_arch_compatible(Arch::X86_64));
    assert!(Platform::Windows.is_arch_compatible(Arch::Arm64));
    assert!(!Platform::Windows.is_arch_compatible(Arch::Riscv64));

    assert!(Platform::Linux.is_arch_compatible(Arch::X86_64));
    assert!(Platform::Linux.is_arch_compatible(Arch::Arm64));
    assert!(Platform::Linux.is_arch_compatible(Arch::Riscv64));

    assert!(Platform::Android.is_arch_compatible(Arch::X86_64));
    assert!(Platform::Android.is_arch_compatible(Arch::Arm64));
    assert!(Platform::Android.is_arch_compatible(Arch::Riscv64));

    assert!(Platform::Freebsd.is_arch_compatible(Arch::X86_64));
    assert!(Platform::Freebsd.is_arch_compatible(Arch::Arm64));
    assert!(Platform::Freebsd.is_arch_compatible(Arch::Riscv64));

    assert!(Platform::Macosx.is_arch_compatible(Arch::X86_64));
    assert!(Platform::Macosx.is_arch_compatible(Arch::Arm64));

    assert!(Platform::Ios.is_arch_compatible(Arch::Arm64));
    assert!(!Platform::Ios.is_arch_compatible(Arch::X86_64));

    assert!(Platform::Tvos.is_arch_compatible(Arch::Arm64));
    assert!(Platform::Xros.is_arch_compatible(Arch::Arm64));

    assert!(Platform::Nx2.is_arch_compatible(Arch::Arm64));
    assert!(Platform::Switch2.is_arch_compatible(Arch::Arm64));
    assert!(!Platform::Switch2.is_arch_compatible(Arch::X86_64));

    assert!(Platform::Ps4.is_arch_compatible(Arch::X86_64));
    assert!(!Platform::Ps4.is_arch_compatible(Arch::Arm64));

    assert!(Platform::Ps5.is_arch_compatible(Arch::X86_64));
    assert!(!Platform::Ps5.is_arch_compatible(Arch::Arm64));

    assert!(Platform::Xboxone.is_arch_compatible(Arch::X86_64));
    assert!(Platform::Xboxxs.is_arch_compatible(Arch::X86_64));

    assert!(Platform::Steamdeck.is_arch_compatible(Arch::X86_64));
    assert!(!Platform::Steamdeck.is_arch_compatible(Arch::Arm64));

    assert!(Platform::Steammachine.is_arch_compatible(Arch::X86_64));
}

#[test]
fn test_evaluate_zero_arguments() {
    // When no arguments are passed (None, None, None), ArchInfo defaults to native host detection
    let report = ArchFeaturesReport::evaluate(None, None, None).unwrap();
    assert_eq!(report.platform, Platform::current().to_string());
    assert_eq!(report.arch, Arch::current().to_string());
    assert_eq!(report.target_cpu, Some("native".to_string()));
    assert!(!report.extensions.is_empty());
    assert!(!report.features.is_empty());
}

#[test]
fn test_native_platform_and_arch() {
    // Parsing "native", "host", "current"
    assert_eq!("native".parse::<Platform>().unwrap(), Platform::Native);
    assert_eq!("host".parse::<Platform>().unwrap(), Platform::Native);
    assert_eq!("current".parse::<Platform>().unwrap(), Platform::Native);

    assert_eq!("native".parse::<Arch>().unwrap(), Arch::Native);
    assert_eq!("host".parse::<Arch>().unwrap(), Arch::Native);
    assert_eq!("current".parse::<Arch>().unwrap(), Arch::Native);

    // Resolving returns current host
    assert_eq!(Platform::Native.resolve(), Platform::current());
    assert_eq!(Arch::Native.resolve(), Arch::current());

    // Compatibility checks with Native
    assert!(Platform::Native.is_arch_compatible(Arch::Native));
    assert!(Platform::Native.is_arch_compatible(Arch::current()));
    assert!(Platform::current().is_arch_compatible(Arch::Native));

    // Both native: -p native -a native
    let report = ArchFeaturesReport::evaluate(Some(Platform::Native), Some(Arch::Native), None).unwrap();
    assert_eq!(report.platform, Platform::current().to_string());
    assert_eq!(report.arch, Arch::current().to_string());
    assert_eq!(report.target_cpu, Some("native".to_string()));

    // Native platform only: -p native
    let report_p = ArchFeaturesReport::evaluate(Some(Platform::Native), None, None).unwrap();
    assert_eq!(report_p.platform, Platform::current().to_string());
    assert_eq!(report_p.arch, Arch::current().to_string());

    // Native arch only: -a native
    let report_a = ArchFeaturesReport::evaluate(None, Some(Arch::Native), None).unwrap();
    assert_eq!(report_a.platform, Platform::current().to_string());
    assert_eq!(report_a.arch, Arch::current().to_string());

    // Host platform + native arch
    let report_hp = ArchFeaturesReport::evaluate(Some(Platform::current()), Some(Arch::Native), None).unwrap();
    assert_eq!(report_hp.platform, Platform::current().to_string());
    assert_eq!(report_hp.arch, Arch::current().to_string());

    // Native platform + host arch
    let report_ha = ArchFeaturesReport::evaluate(Some(Platform::Native), Some(Arch::current()), None).unwrap();
    assert_eq!(report_ha.platform, Platform::current().to_string());
    assert_eq!(report_ha.arch, Arch::current().to_string());

    // from_target with Native
    let report_target = ArchFeaturesReport::from_target(Platform::Native, Arch::Native).unwrap();
    assert_eq!(report_target.platform, Platform::current().to_string());
    assert_eq!(report_target.arch, Arch::current().to_string());
}

#[test]
fn test_default_output_dir() {
    let dir = get_default_output_dir();
    let dir_str = dir.to_string_lossy();
    assert!(dir_str.contains("Unreal Engine"));
    assert!(dir_str.contains("ArchInfo"));

    let report = ArchFeaturesReport::evaluate(Some(Platform::Windows), Some(Arch::X86_64), Some("native")).unwrap();
    let file_path = report.default_filepath();
    let path_str = file_path.to_string_lossy();
    assert!(path_str.contains("Unreal Engine"));
    assert!(path_str.contains("ArchInfo"));
    assert!(path_str.ends_with(".json"));
}

#[test]
fn test_canonical_filenames() {
    let sw2 = ArchFeaturesReport::from_target(Platform::Switch2, Arch::Arm64).unwrap();
    assert_eq!(sw2.filename(), "switch2-aarch64-cortex-a78c-armv8.4-a-vl128.json");

    let ps5 = ArchFeaturesReport::from_target(Platform::Ps5, Arch::X86_64).unwrap();
    assert_eq!(ps5.filename(), "ps5-x86_64-znver2-avx2-vl256.json");

    let steamdeck = ArchFeaturesReport::from_target(Platform::Steamdeck, Arch::X86_64).unwrap();
    assert_eq!(steamdeck.filename(), "steamdeck-x86_64-znver2-avx2-vl256.json");

    let ps4 = ArchFeaturesReport::from_target(Platform::Ps4, Arch::X86_64).unwrap();
    assert_eq!(ps4.filename(), "ps4-x86_64-btver2-avx-vl128.json");

    let m4 = ArchFeaturesReport::evaluate(Some(Platform::Macosx), Some(Arch::Arm64), Some("apple-m4")).unwrap();
    assert_eq!(m4.filename(), "macosx-aarch64-apple-m4-armv9.2-a-vl128.json");
}

#[test]
fn test_evaluate_native_detection() {
    let report = ArchFeaturesReport::evaluate(Some(Platform::Windows), Some(Arch::X86_64), Some("native")).unwrap();
    assert_eq!(report.platform, "windows");
    assert_eq!(report.arch, "x86_64");
    assert_eq!(report.target_cpu, Some("native".to_string()));
    assert!(!report.extensions.is_empty());
    assert!(report.extensions.contains("sse"));
    assert!(!report.features.is_empty());
}

#[test]
fn test_evaluate_generic_and_known_targets_x64() {
    // Generic default (None / SSE4.2 -> x86-64-v2)
    let gen_report = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::X86_64), Some("generic")).unwrap();
    assert_eq!(gen_report.target_cpu, Some("x86-64-v2".to_string()));
    assert!(gen_report.extensions.contains("sse4.2") || gen_report.extensions.contains("sse4_2"));
    assert_eq!(gen_report.features.get("sse4_2"), Some(&true));
    assert_eq!(gen_report.min_cpu_arch, Some("sse4.2".to_string()));
    assert_eq!(gen_report.filename(), "linux-x86_64-generic-x86-64-v2-sse4.2-vl128.json");

    // Znver3
    let zen3_report = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::X86_64), Some("znver3")).unwrap();
    assert_eq!(zen3_report.target_cpu, Some("znver3".to_string()));
    assert!(zen3_report.extensions.contains("avx2"));
    assert!(zen3_report.extensions.contains("vaes"));
    assert_eq!(zen3_report.features.get("avx2"), Some(&true));
    assert_eq!(zen3_report.features.get("vaes"), Some(&true));
    assert_eq!(zen3_report.min_cpu_arch, Some("avx2".to_string()));
    assert_eq!(zen3_report.filename(), "linux-x86_64-znver3-avx2-vl256.json");
}

#[test]
fn test_evaluate_known_targets_arm64() {
    // Cortex-A78C (Switch 2)
    let a78c_report = ArchFeaturesReport::evaluate(Some(Platform::Switch2), Some(Arch::Arm64), Some("cortex-a78c")).unwrap();
    assert_eq!(a78c_report.target_cpu, Some("cortex-a78c".to_string()));
    assert_eq!(a78c_report.target_clang_cpu, Some("-m'cpu=cortex-a78c'".to_string()));
    assert_eq!(a78c_report.target_clang_isaarch, Some("".to_string()));
    assert!(a78c_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=armv8.4-a+"));
    assert!(a78c_report.extensions.contains("pauth"));
    assert!(a78c_report.extensions.contains("dotprod"));
    assert_eq!(a78c_report.features.get("pauth"), Some(&true));
    assert_eq!(a78c_report.features.get("dotprod"), Some(&true));
    assert_eq!(a78c_report.filename(), "switch2-aarch64-cortex-a78c-armv8.4-a-vl128.json");

    // Apple M4
    let m4_report = ArchFeaturesReport::evaluate(Some(Platform::Macosx), Some(Arch::Arm64), Some("apple-m4")).unwrap();
    assert_eq!(m4_report.target_cpu, Some("apple-m4".to_string()));
    assert_eq!(m4_report.target_clang_cpu, Some("-m'cpu=apple-m4'".to_string()));
    assert_eq!(m4_report.target_clang_isaarch, Some("".to_string()));
    assert!(m4_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=armv9.2-a+"));
    assert!(m4_report.extensions.contains("sme"));
    assert!(m4_report.extensions.contains("sme2"));
    assert!(m4_report.extensions.contains("wfxt"));
    assert_eq!(m4_report.features.get("sme"), Some(&true));
    assert_eq!(m4_report.features.get("wfxt"), Some(&true));
    assert_eq!(m4_report.filename(), "macosx-aarch64-apple-m4-armv9.2-a-vl128.json");
}

#[test]
fn test_evaluate_known_targets_riscv64() {
    // SiFive P470
    let p470_report = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::Riscv64), Some("sifive-p470")).unwrap();
    assert_eq!(p470_report.target_cpu, Some("sifive-p470".to_string()));
    assert_eq!(p470_report.target_clang_cpu, Some("-m'cpu=sifive-p470'".to_string()));
    assert_eq!(p470_report.target_clang_isaarch, Some("".to_string()));
    assert!(p470_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=rv64"));
    assert!(p470_report.target_clan_arch.as_ref().unwrap().contains("_v"));
    assert!(p470_report.target_clan_arch.as_ref().unwrap().contains("_zvbb"));
    assert!(p470_report.target_clan_arch.as_ref().unwrap().contains("_zvl32b"));
    assert!(p470_report.target_clan_arch.as_ref().unwrap().contains("_zvl64b"));
    assert!(p470_report.extensions.contains("v"));
    assert!(p470_report.extensions.contains("zvbb"));
    assert!(p470_report.extensions.contains("zvl32b"));
    assert!(p470_report.extensions.contains("zvl64b"));
    assert_eq!(p470_report.features.get("v"), Some(&true));
    assert_eq!(p470_report.features.get("zvbb"), Some(&true));
    assert_eq!(p470_report.features.get("zvl32b"), Some(&true));
    assert_eq!(p470_report.features.get("zvl64b"), Some(&true));
    assert_eq!(p470_report.filename(), "linux-riscv64-sifive-p470-none-vl128.json");
}

#[test]
fn test_switch_profiles() {
    // Platform::Nx2 is an alias to Platform::Switch2
    assert_eq!(Platform::Nx2, Platform::Switch2);
    assert_eq!("nx2".parse::<Platform>().unwrap(), Platform::Switch2);
    assert_eq!("switch2".parse::<Platform>().unwrap(), Platform::Switch2);
    assert_eq!(serde_json::from_str::<Platform>("\"nx2\"").unwrap(), Platform::Switch2);
    assert_eq!(serde_json::from_str::<Platform>("\"switch2\"").unwrap(), Platform::Switch2);

    let (sw2_ext, _, sw2_arm) = TargetProfile::get_features(Platform::Switch2, Arch::Arm64).unwrap();
    assert_eq!(sw2_arm, TargetCpuArchitectureArm64::Cortex_A78C);
    assert!(sw2_ext.contains("pauth"));
    assert!(sw2_ext.contains("lse"));
    assert!(sw2_ext.contains("dotprod"));

    // Verify evaluating with nx2 alias produces switch2 in report and json output
    let report_nx2 = ArchFeaturesReport::evaluate(Some("nx2".parse().unwrap()), Some(Arch::Arm64), None).unwrap();
    let report_sw2 = ArchFeaturesReport::evaluate(Some(Platform::Switch2), Some(Arch::Arm64), None).unwrap();
    assert_eq!(report_nx2.platform, "switch2");
    assert_eq!(report_sw2.platform, "switch2");
    assert_eq!(report_nx2.to_json().unwrap(), report_sw2.to_json().unwrap());
    assert!(report_nx2.to_json().unwrap().contains("\"platform\": \"switch2\""));
}

#[test]
fn test_steamdeck_profile() {
    let (sd_ext, sd_x64, _) = TargetProfile::get_features(Platform::Steamdeck, Arch::X86_64).unwrap();
    assert_eq!(sd_x64, TargetCpuArchitectureX64::Znver2);
    assert!(sd_ext.contains("avx2"));
    assert!(sd_ext.contains("fma"));
    assert!(sd_ext.contains("bmi2"));
    assert!(sd_ext.contains("sha"));
}

#[test]
fn test_ps5_and_xbox_profiles() {
    let (ps5_ext, ps5_x64, _) = TargetProfile::get_features(Platform::Ps5, Arch::X86_64).unwrap();
    assert_eq!(ps5_x64, TargetCpuArchitectureX64::Znver2);
    assert!(ps5_ext.contains("avx2"));

    let (xb_ext, xb_x64, _) = TargetProfile::get_features(Platform::Xboxxs, Arch::X86_64).unwrap();
    assert_eq!(xb_x64, TargetCpuArchitectureX64::Znver2);
    assert!(xb_ext.contains("avx2"));
}

#[test]
fn test_json_report_generation() {
    let report = ArchFeaturesReport::from_target(Platform::Windows, Arch::X86_64).unwrap();
    assert_eq!(report.platform, "windows");
    assert_eq!(report.arch, "x86_64");
    assert!(!report.extensions.is_empty());
    assert!(report.extensions.contains("sse"));
    assert!(report.extensions.contains("aes"));
    assert_eq!(report.target_msvc_arch, Some("/arch:SSE4.2".to_string()));
    assert_eq!(report.target_clan_arch, Some("-m'arch=x86-64-v2'".to_string()));
    assert_eq!(report.target_clang_cpu, Some("".to_string()));

    let json = report.to_json().unwrap();
    assert!(json.contains("\"platform\": \"windows\""));
    assert!(json.contains("\"arch\": \"x86_64\""));
    assert!(json.contains("\"extensions\":"));
    assert!(json.contains("\"target_msvc_arch\": \"/arch:SSE4.2\""));
    assert!(json.contains("\"target_clan_arch\": \"-m'arch=x86-64-v2'\""));
    assert!(json.contains("\"target_clang_cpu\": \"\""));

    // Test PS5 / Zen 2 target has /arch:AVX2 and -m'arch=znver2'
    let ps5_report = ArchFeaturesReport::from_target(Platform::Ps5, Arch::X86_64).unwrap();
    assert_eq!(ps5_report.target_msvc_arch, Some("/arch:AVX2".to_string()));
    assert_eq!(ps5_report.target_clan_arch, Some("-m'arch=znver2'".to_string()));
    assert_eq!(ps5_report.target_clang_cpu, Some("".to_string()));
}

#[test]
fn test_full_matrix_report() {
    let matrix = PlatformArchMatrixReport::generate();
    assert!(!matrix.matrix.is_empty());

    let json = matrix.to_json().unwrap();
    assert!(json.contains("\"matrix\":"));
    assert!(json.contains("\"platform\": \"switch2\""));
    assert!(json.contains("\"platform\": \"steamdeck\""));
    assert!(json.contains("\"platform\": \"ps5\""));
    assert!(json.contains("\"platform\": \"xboxxs\""));
}

#[test]
fn test_x64_vector_lengths() {
    // Generic / SSE4.2 -> VL128
    let gen_f = X64CPUFeatures::from_target(TargetCpuArchitectureX64::Generic);
    assert_eq!(gen_f.vector_length(), CpuArchitectureVectorLength::VL128);

    // AVX only (less than AVX2) -> VL128
    let avx_f = X64CPUFeatures::from_target(TargetCpuArchitectureX64::Sandybridge);
    assert_eq!(avx_f.vector_length(), CpuArchitectureVectorLength::VL128);

    // Jaguar / BTVER2 (PS4 / XboxOne) -> VL128
    let btver2_f = X64CPUFeatures::from_target(TargetCpuArchitectureX64::Btver2);
    assert_eq!(btver2_f.vector_length(), CpuArchitectureVectorLength::VL128);

    // AVX2 (Zen 2 / PS5 / Haswell) -> VL256
    let avx2_f = X64CPUFeatures::from_target(TargetCpuArchitectureX64::Znver2);
    assert_eq!(avx2_f.vector_length(), CpuArchitectureVectorLength::VL256);

    // AVX-512 -> VL512
    let avx512_f = X64CPUFeatures::from_target(TargetCpuArchitectureX64::Skylake_avx512);
    assert_eq!(avx512_f.vector_length(), CpuArchitectureVectorLength::VL512);

    // AVX10.1 / AVX10.2 normal -> VL256
    let avx10_f = X64CPUFeatures::from_target(TargetCpuArchitectureX64::Diamondrapids);
    assert_eq!(avx10_f.vector_length(), CpuArchitectureVectorLength::VL256);

    // Arm64 normal -> VL128
    let arm_f = Arm64CPUFeatures::from_target(TargetCpuArchitectureArm64::Cortex_A78C);
    assert_eq!(arm_f.vector_length(), CpuArchitectureVectorLength::VL128);

    // Riscv64 normal -> VL128
    let riscv_f = Riscv64CPUFeatures::from_target(TargetCpuArchitectureRiscv64::Sifive_P470);
    assert_eq!(riscv_f.vector_length(), CpuArchitectureVectorLength::VL128);

    // Clang preferred vector length flags (-m'prefer-vector-width=...')
    assert_eq!(archinfo::ClangCpuArchitecturePreferredVectorLength::name(CpuArchitectureVectorLength::None), "");
    assert_eq!(archinfo::ClangCpuArchitecturePreferredVectorLength::name(CpuArchitectureVectorLength::VL128), "-m'prefer-vector-width=128'");
    assert_eq!(archinfo::ClangCpuArchitecturePreferredVectorLength::name(CpuArchitectureVectorLength::VL256), "-m'prefer-vector-width=256'");
    assert_eq!(archinfo::ClangCpuArchitecturePreferredVectorLength::name(CpuArchitectureVectorLength::VL512), "-m'prefer-vector-width=512'");

    // Clang vector length suffix flags (for -mavx10.1-256 / -mavx10.1-512)
    assert_eq!(archinfo::ClangCpuArchitectureVectorLengthNames::name(CpuArchitectureVectorLength::None), "");
    assert_eq!(archinfo::ClangCpuArchitectureVectorLengthNames::name(CpuArchitectureVectorLength::VL128), "");
    assert_eq!(archinfo::ClangCpuArchitectureVectorLengthNames::name(CpuArchitectureVectorLength::VL256), "-256");
    assert_eq!(archinfo::ClangCpuArchitectureVectorLengthNames::name(CpuArchitectureVectorLength::VL512), "-512");

    // MSVC preferred vector length flags (/vlen=256 / /vlen=512)
    assert_eq!(archinfo::MSVCCpuArchitecturePreferredVectorLength::name(CpuArchitectureVectorLength::None), "");
    assert_eq!(archinfo::MSVCCpuArchitecturePreferredVectorLength::name(CpuArchitectureVectorLength::VL128), "");
    assert_eq!(archinfo::MSVCCpuArchitecturePreferredVectorLength::name(CpuArchitectureVectorLength::VL256), "/vlen=256");
    assert_eq!(archinfo::MSVCCpuArchitecturePreferredVectorLength::name(CpuArchitectureVectorLength::VL512), "/vlen=512");

    // MSVC /vlen override argument resolution (MSVCX64VLen)
    assert_eq!(archinfo::MSVCX64VLen::name(MinimumCpuArchitectureX64::None, CpuArchitectureVectorLength::VL128), "");
    assert_eq!(archinfo::MSVCX64VLen::name(MinimumCpuArchitectureX64::AVX, CpuArchitectureVectorLength::VL128), "");
    assert_eq!(archinfo::MSVCX64VLen::name(MinimumCpuArchitectureX64::AVX2, CpuArchitectureVectorLength::VL256), "");
    assert_eq!(archinfo::MSVCX64VLen::name(MinimumCpuArchitectureX64::AVX512, CpuArchitectureVectorLength::VL512), "");
    assert_eq!(archinfo::MSVCX64VLen::name(MinimumCpuArchitectureX64::AVX512, CpuArchitectureVectorLength::VL256), "/vlen=256");
    assert_eq!(archinfo::MSVCX64VLen::name(MinimumCpuArchitectureX64::AVX10_1, CpuArchitectureVectorLength::VL256), "");
    assert_eq!(archinfo::MSVCX64VLen::name(MinimumCpuArchitectureX64::AVX10_1, CpuArchitectureVectorLength::VL512), "/vlen=512");
    assert_eq!(archinfo::MSVCX64VLen::name(MinimumCpuArchitectureX64::AVX10_2, CpuArchitectureVectorLength::VL256), "");
    assert_eq!(archinfo::MSVCX64VLen::name(MinimumCpuArchitectureX64::AVX10_2, CpuArchitectureVectorLength::VL512), "/vlen=512");

    // Clang prefer-vector-width override argument resolution (ClangX64VLen)
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::None, CpuArchitectureVectorLength::VL128), "-m'prefer-vector-width=128'");
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::AVX, CpuArchitectureVectorLength::VL128), "-m'prefer-vector-width=128'");
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::AVX2, CpuArchitectureVectorLength::VL256), "-m'prefer-vector-width=256'");
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::AVX2, CpuArchitectureVectorLength::VL128), "-m'prefer-vector-width=128'");
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::AVX512, CpuArchitectureVectorLength::VL512), "-m'prefer-vector-width=512'");
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::AVX512, CpuArchitectureVectorLength::VL256), "-m'prefer-vector-width=256'");
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::AVX10_1, CpuArchitectureVectorLength::VL256), "-m'prefer-vector-width=256'");
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::AVX10_1, CpuArchitectureVectorLength::VL512), "-m'prefer-vector-width=512'");
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::AVX10_2, CpuArchitectureVectorLength::VL256), "-m'prefer-vector-width=256'");
    assert_eq!(archinfo::ClangX64VLen::name(MinimumCpuArchitectureX64::AVX10_2, CpuArchitectureVectorLength::VL512), "-m'prefer-vector-width=512'");
}

#[test]
fn test_vector_length_argument_resolution() {
    // 1. AVX10: normal is 256. If user sets 512 -> VL512. If user sets 128 -> VL128. If user sets 256 -> VL256. Otherwise VL256.
    let avx10_default = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("diamondrapids"), None).unwrap();
    assert_eq!(avx10_default.vector_length, Some("vl256".to_string()));

    let avx10_512 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("diamondrapids"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(avx10_512.vector_length, Some("vl512".to_string()));

    let avx10_256 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("diamondrapids"), Some(CpuArchitectureVectorLength::VL256)).unwrap();
    assert_eq!(avx10_256.vector_length, Some("vl256".to_string()));

    let avx10_128 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("diamondrapids"), Some(CpuArchitectureVectorLength::VL128)).unwrap();
    assert_eq!(avx10_128.vector_length, Some("vl128".to_string()));

    // 2. AVX-512F: normal is 512. If user sets 512 -> VL512. If user sets 256 -> VL256. If user sets 128 -> VL128. Otherwise VL512.
    let avx512_default = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("skylake-avx512"), None).unwrap();
    assert_eq!(avx512_default.vector_length, Some("vl512".to_string()));

    let avx512_req256 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("skylake-avx512"), Some(CpuArchitectureVectorLength::VL256)).unwrap();
    assert_eq!(avx512_req256.vector_length, Some("vl256".to_string()));

    let avx512_req128 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("skylake-avx512"), Some(CpuArchitectureVectorLength::VL128)).unwrap();
    assert_eq!(avx512_req128.vector_length, Some("vl128".to_string()));

    let avx512_req512 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("skylake-avx512"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(avx512_req512.vector_length, Some("vl512".to_string()));

    // 3. AVX2: normal is 256. If user sets 128 -> VL128. If user sets 256 -> VL256. Cannot select higher (VL512 -> VL256).
    let avx2_default = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("znver3"), None).unwrap();
    assert_eq!(avx2_default.vector_length, Some("vl256".to_string()));

    let avx2_req128 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("znver3"), Some(CpuArchitectureVectorLength::VL128)).unwrap();
    assert_eq!(avx2_req128.vector_length, Some("vl128".to_string()));

    let avx2_req256 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("znver3"), Some(CpuArchitectureVectorLength::VL256)).unwrap();
    assert_eq!(avx2_req256.vector_length, Some("vl256".to_string()));

    let avx2_req512 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("znver3"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(avx2_req512.vector_length, Some("vl256".to_string()));

    // 4. AVX, SSE4.2, Arm64, Riscv64: fixed normal is 128, ignores user input
    let sse_report = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some(CpuArchitectureVectorLength::VL256)).unwrap();
    assert_eq!(sse_report.vector_length, Some("vl128".to_string()));

    let sse_req512 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(sse_req512.vector_length, Some("vl128".to_string()));

    let avx_req512 = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), None, Some("avx"), None, None, Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(avx_req512.vector_length, Some("vl128".to_string()));

    let arm_report = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Switch2), Some(Arch::Arm64), Some("cortex-a78c"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(arm_report.vector_length, Some("vl128".to_string()));

    let riscv_report = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::Riscv64), Some("sifive-p470"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(riscv_report.vector_length, Some("vl128".to_string()));
}

#[test]
fn test_minimum_cpu_architecture_vector_length_security() {
    let lengths = [
        None,
        Some(CpuArchitectureVectorLength::None),
        Some(CpuArchitectureVectorLength::VL128),
        Some(CpuArchitectureVectorLength::VL256),
        Some(CpuArchitectureVectorLength::VL512),
    ];

    // SSE4.2 (None) and AVX: fixed vl128, ignores user input
    for &req in &lengths {
        assert_eq!(MinimumCpuArchitectureX64::None.resolve_vector_length(req), CpuArchitectureVectorLength::VL128);
        assert_eq!(MinimumCpuArchitectureX64::AVX.resolve_vector_length(req), CpuArchitectureVectorLength::VL128);
    }

    // AVX2: vl256 or vl128; defaults to vl256; cannot select higher than 256 (vl512 -> vl256)
    assert_eq!(MinimumCpuArchitectureX64::AVX2.resolve_vector_length(None), CpuArchitectureVectorLength::VL256);
    assert_eq!(MinimumCpuArchitectureX64::AVX2.resolve_vector_length(Some(CpuArchitectureVectorLength::None)), CpuArchitectureVectorLength::VL256);
    assert_eq!(MinimumCpuArchitectureX64::AVX2.resolve_vector_length(Some(CpuArchitectureVectorLength::VL128)), CpuArchitectureVectorLength::VL128);
    assert_eq!(MinimumCpuArchitectureX64::AVX2.resolve_vector_length(Some(CpuArchitectureVectorLength::VL256)), CpuArchitectureVectorLength::VL256);
    assert_eq!(MinimumCpuArchitectureX64::AVX2.resolve_vector_length(Some(CpuArchitectureVectorLength::VL512)), CpuArchitectureVectorLength::VL256);

    // AVX512: vl512 or vl256 or vl128; defaults to vl512
    assert_eq!(MinimumCpuArchitectureX64::AVX512.resolve_vector_length(None), CpuArchitectureVectorLength::VL512);
    assert_eq!(MinimumCpuArchitectureX64::AVX512.resolve_vector_length(Some(CpuArchitectureVectorLength::None)), CpuArchitectureVectorLength::VL512);
    assert_eq!(MinimumCpuArchitectureX64::AVX512.resolve_vector_length(Some(CpuArchitectureVectorLength::VL128)), CpuArchitectureVectorLength::VL128);
    assert_eq!(MinimumCpuArchitectureX64::AVX512.resolve_vector_length(Some(CpuArchitectureVectorLength::VL256)), CpuArchitectureVectorLength::VL256);
    assert_eq!(MinimumCpuArchitectureX64::AVX512.resolve_vector_length(Some(CpuArchitectureVectorLength::VL512)), CpuArchitectureVectorLength::VL512);

    // AVX10.1 and AVX10.2: vl512 or vl256 or vl128; defaults to vl256
    for &avx10 in &[MinimumCpuArchitectureX64::AVX10_1, MinimumCpuArchitectureX64::AVX10_2] {
        assert_eq!(avx10.resolve_vector_length(None), CpuArchitectureVectorLength::VL256);
        assert_eq!(avx10.resolve_vector_length(Some(CpuArchitectureVectorLength::None)), CpuArchitectureVectorLength::VL256);
        assert_eq!(avx10.resolve_vector_length(Some(CpuArchitectureVectorLength::VL128)), CpuArchitectureVectorLength::VL128);
        assert_eq!(avx10.resolve_vector_length(Some(CpuArchitectureVectorLength::VL256)), CpuArchitectureVectorLength::VL256);
        assert_eq!(avx10.resolve_vector_length(Some(CpuArchitectureVectorLength::VL512)), CpuArchitectureVectorLength::VL512);
    }
}

#[test]
fn test_user_min_arch_selection_x64_generic() {
    // 1. Generic + AVX
    let avx_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), None, Some("avx"), None, None, None).unwrap();
    assert_eq!(avx_report.target_cpu, Some("x86-64-v2".to_string()));
    assert_eq!(avx_report.min_cpu_arch, Some("avx".to_string()));
    assert_eq!(avx_report.vector_length, Some("vl128".to_string()));
    assert_eq!(avx_report.target_msvc_arch, Some("/arch:AVX".to_string()));
    assert_eq!(avx_report.target_msvc_vlen, Some("".to_string()));
    assert_eq!(avx_report.target_clan_arch, Some("-m'arch=x86-64-v2'".to_string()));
    assert!(avx_report.extensions.contains("avx"));
    assert!(avx_report.target_clang_isaarch.as_ref().unwrap().contains("-m'avx'"));
    assert_eq!(avx_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(avx_report.filename(), "linux-x86_64-generic-x86-64-v2-avx-vl128.json");

    // 2. Generic + AVX2 (default VL256 -> target_clang_vlen: "-m'prefer-vector-width=256'")
    let avx2_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), None, Some("avx2"), None, None, None).unwrap();
    assert_eq!(avx2_report.target_cpu, Some("x86-64-v3".to_string()));
    assert_eq!(avx2_report.min_cpu_arch, Some("avx2".to_string()));
    assert_eq!(avx2_report.vector_length, Some("vl256".to_string()));
    assert_eq!(avx2_report.target_msvc_arch, Some("/arch:AVX2".to_string()));
    assert_eq!(avx2_report.target_msvc_vlen, Some("".to_string()));
    assert_eq!(avx2_report.target_clan_arch, Some("-m'arch=x86-64-v3'".to_string()));
    assert!(avx2_report.extensions.contains("avx2"));
    assert!(avx2_report.extensions.contains("fma"));
    assert!(avx2_report.target_clang_isaarch.as_ref().unwrap().contains("-m'avx2'"));
    assert_eq!(avx2_report.target_clang_vlen, Some("-m'prefer-vector-width=256'".to_string()));
    assert_eq!(avx2_report.filename(), "linux-x86_64-generic-x86-64-v3-avx2-vl256.json");

    // AVX2 with VL128 override -> target_clang_vlen: "-m'prefer-vector-width=128'"
    let avx2_vl128_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), None, Some("avx2"), None, None, Some(CpuArchitectureVectorLength::VL128)).unwrap();
    assert_eq!(avx2_vl128_report.vector_length, Some("vl128".to_string()));
    assert_eq!(avx2_vl128_report.target_msvc_vlen, Some("".to_string()));
    assert_eq!(avx2_vl128_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));

    // 3. Generic + AVX512 (default VL512 -> target_msvc_vlen: "", target_clang_vlen: "-m'prefer-vector-width=512'")
    let avx512_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), None, Some("avx512"), None, None, None).unwrap();
    assert_eq!(avx512_report.target_cpu, Some("x86-64-v4".to_string()));
    assert_eq!(avx512_report.min_cpu_arch, Some("avx512".to_string()));
    assert_eq!(avx512_report.vector_length, Some("vl512".to_string()));
    assert_eq!(avx512_report.target_msvc_arch, Some("/arch:AVX512".to_string()));
    assert_eq!(avx512_report.target_msvc_vlen, Some("".to_string()));
    assert_eq!(avx512_report.target_clan_arch, Some("-m'arch=x86-64-v4'".to_string()));
    assert!(avx512_report.extensions.contains("avx512f"));
    assert!(avx512_report.target_clang_isaarch.as_ref().unwrap().contains("-m'avx512f'"));
    assert_eq!(avx512_report.target_clang_vlen, Some("-m'prefer-vector-width=512'".to_string()));
    assert_eq!(avx512_report.filename(), "linux-x86_64-generic-x86-64-v4-avx512-vl512.json");

    // AVX512 with user requested VL256 -> /arch:AVX512 /vlen=256, clang: -m'prefer-vector-width=256'
    let avx512_vl256_report = ArchFeaturesReport::evaluate_full(Some(Platform::Windows), Some(Arch::X86_64), Some("generic"), None, Some("avx512"), None, None, Some(CpuArchitectureVectorLength::VL256)).unwrap();
    assert_eq!(avx512_vl256_report.vector_length, Some("vl256".to_string()));
    assert_eq!(avx512_vl256_report.target_msvc_arch, Some("/arch:AVX512".to_string()));
    assert_eq!(avx512_vl256_report.target_msvc_vlen, Some("/vlen=256".to_string()));
    assert_eq!(avx512_vl256_report.target_clang_vlen, Some("-m'prefer-vector-width=256'".to_string()));

    // AVX512 with VL128 override -> target_clang_vlen: "-m'prefer-vector-width=128'"
    let avx512_vl128_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), None, Some("avx512"), None, None, Some(CpuArchitectureVectorLength::VL128)).unwrap();
    assert_eq!(avx512_vl128_report.vector_length, Some("vl128".to_string()));
    assert_eq!(avx512_vl128_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(avx512_vl128_report.filename(), "linux-x86_64-generic-x86-64-v4-avx512-vl128.json");

    // 4. Generic + AVX10.1 (default VL256 -> target_msvc_vlen: "", target_clang_vlen: "-m'prefer-vector-width=256'", clang: -m'avx10.1')
    let avx10_1_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), None, Some("avx10.1"), None, None, None).unwrap();
    assert_eq!(avx10_1_report.target_cpu, Some("x86-64-v3".to_string()));
    assert_eq!(avx10_1_report.min_cpu_arch, Some("avx10.1".to_string()));
    assert_eq!(avx10_1_report.vector_length, Some("vl256".to_string()));
    assert_eq!(avx10_1_report.target_msvc_arch, Some("/arch:AVX10.1".to_string()));
    assert_eq!(avx10_1_report.target_msvc_vlen, Some("".to_string()));
    assert_eq!(avx10_1_report.target_clan_arch, Some("-m'arch=x86-64-v3'".to_string()));
    assert!(avx10_1_report.extensions.contains("avx10.1"));
    assert!(avx10_1_report.target_clang_isaarch.as_ref().unwrap().contains("-m'avx10.1'"));
    assert!(!avx10_1_report.target_clang_isaarch.as_ref().unwrap().contains("-m'avx10.1-256'"));
    assert_eq!(avx10_1_report.target_clang_vlen, Some("-m'prefer-vector-width=256'".to_string()));
    assert_eq!(avx10_1_report.filename(), "linux-x86_64-generic-x86-64-v3-avx10.1-vl256.json");

    // AVX10.1 with VL128 override -> target_clang_vlen: "-m'prefer-vector-width=128'"
    let avx10_1_vl128_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), None, Some("avx10.1"), None, None, Some(CpuArchitectureVectorLength::VL128)).unwrap();
    assert_eq!(avx10_1_vl128_report.vector_length, Some("vl128".to_string()));
    assert_eq!(avx10_1_vl128_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(avx10_1_vl128_report.filename(), "linux-x86_64-generic-x86-64-v3-avx10.1-vl128.json");

    // 5. Generic + AVX10.2 with user requested VL512 -> /arch:AVX10.2 /vlen=512, clang: -m'avx10.2-512' -m'prefer-vector-width=512'
    let avx10_2_report = ArchFeaturesReport::evaluate_full(Some(Platform::Windows), Some(Arch::X86_64), Some("generic"), None, Some("avx10.2"), None, None, Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(avx10_2_report.target_cpu, Some("x86-64-v3".to_string()));
    assert_eq!(avx10_2_report.min_cpu_arch, Some("avx10.2".to_string()));
    assert_eq!(avx10_2_report.vector_length, Some("vl512".to_string()));
    assert_eq!(avx10_2_report.target_msvc_arch, Some("/arch:AVX10.2".to_string()));
    assert_eq!(avx10_2_report.target_msvc_vlen, Some("/vlen=512".to_string()));
    assert_eq!(avx10_2_report.target_clan_arch, Some("-m'arch=x86-64-v3'".to_string()));
    assert!(avx10_2_report.extensions.contains("avx10.2"));
    assert!(avx10_2_report.target_clang_isaarch.as_ref().unwrap().contains("-m'avx10.2-512'"));
    assert_eq!(avx10_2_report.target_clang_vlen, Some("-m'prefer-vector-width=512'".to_string()));
    assert_eq!(avx10_2_report.filename(), "windows-x86_64-generic-x86-64-v3-avx10.2-vl512.json");

    // 6. Non-generic target ignores min_arch override
    let znver3_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("znver3"), None, Some("avx512"), None, None, None).unwrap();
    assert_eq!(znver3_report.target_cpu, Some("znver3".to_string()));
    assert_eq!(znver3_report.min_cpu_arch, Some("avx2".to_string()));
    assert_eq!(znver3_report.target_clan_arch, Some("-m'arch=znver3'".to_string()));
    assert_eq!(znver3_report.filename(), "linux-x86_64-znver3-avx2-vl256.json");
}

#[test]
fn test_user_min_arch_selection_arm64_generic() {
    // 1. Generic + ARMv8-A
    let v8_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), None, Some("armv8-a"), None, None, None).unwrap();
    assert_eq!(v8_report.target_cpu, Some("generic".to_string()));
    assert_eq!(v8_report.min_cpu_arch, Some("armv8-a".to_string()));
    assert_eq!(v8_report.vector_length, Some("vl128".to_string()));
    assert_eq!(v8_report.target_msvc_arch, Some("/arch:armv8.0".to_string()));
    assert!(v8_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=armv8-a+"));
    assert_eq!(v8_report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(v8_report.target_clang_cpu, Some("-m'cpu=generic'".to_string()));
    assert_eq!(v8_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(v8_report.filename(), "linux-aarch64-generic-armv8-a-vl128.json");

    // 2. Generic + ARMv8.2-A
    let v82_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), None, Some("armv8.2-a"), None, None, None).unwrap();
    assert_eq!(v82_report.target_cpu, Some("generic".to_string()));
    assert_eq!(v82_report.min_cpu_arch, Some("armv8.2-a".to_string()));
    assert_eq!(v82_report.vector_length, Some("vl128".to_string()));
    assert_eq!(v82_report.target_msvc_arch, Some("/arch:armv8.2".to_string()));
    assert!(v82_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=armv8.2-a+"));
    assert!(v82_report.target_clan_arch.as_ref().unwrap().contains("+dotprod"));
    assert_eq!(v82_report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(v82_report.target_clang_cpu, Some("-m'cpu=generic'".to_string()));
    assert!(v82_report.extensions.contains("dotprod"));
    assert_eq!(v82_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(v82_report.filename(), "linux-aarch64-generic-armv8.2-a-vl128.json");

    // 3. Generic + ARMv8-R
    let v8r_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), None, Some("armv8-r"), None, None, None).unwrap();
    assert_eq!(v8r_report.target_cpu, Some("generic".to_string()));
    assert_eq!(v8r_report.min_cpu_arch, Some("armv8-r".to_string()));
    assert_eq!(v8r_report.vector_length, Some("vl128".to_string()));
    assert_eq!(v8r_report.target_msvc_arch, Some("/arch:armv8.4".to_string()));
    assert!(v8r_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=armv8-r+"));
    assert_eq!(v8r_report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(v8r_report.target_clang_cpu, Some("-m'cpu=generic'".to_string()));
    assert_eq!(v8r_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(v8r_report.filename(), "linux-aarch64-generic-armv8-r-vl128.json");

    // 4. Generic + ARMv8.9-A
    let v89_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), None, Some("armv8.9-a"), None, None, None).unwrap();
    assert_eq!(v89_report.target_cpu, Some("generic".to_string()));
    assert_eq!(v89_report.min_cpu_arch, Some("armv8.9-a".to_string()));
    assert_eq!(v89_report.vector_length, Some("vl128".to_string()));
    assert_eq!(v89_report.target_msvc_arch, Some("/arch:armv8.9".to_string()));
    assert!(v89_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=armv8.9-a+"));
    assert_eq!(v89_report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(v89_report.target_clang_cpu, Some("-m'cpu=generic'".to_string()));
    assert!(v89_report.extensions.contains("cssc"));
    assert_eq!(v89_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(v89_report.filename(), "linux-aarch64-generic-armv8.9-a-vl128.json");

    // 5. Generic + ARMv9-A
    let v9_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), None, Some("armv9-a"), None, None, None).unwrap();
    assert_eq!(v9_report.target_cpu, Some("generic".to_string()));
    assert_eq!(v9_report.min_cpu_arch, Some("armv9-a".to_string()));
    assert_eq!(v9_report.vector_length, Some("vl128".to_string()));
    assert_eq!(v9_report.target_msvc_arch, Some("/arch:armv9.0".to_string()));
    assert!(v9_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=armv9-a+"));
    assert_eq!(v9_report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(v9_report.target_clang_cpu, Some("-m'cpu=generic'".to_string()));
    assert!(v9_report.extensions.contains("sve"));
    assert_eq!(v9_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(v9_report.filename(), "linux-aarch64-generic-armv9-a-vl128.json");

    // 6. Generic + ARMv9.4-A
    let v94_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), None, Some("armv9.4-a"), None, None, None).unwrap();
    assert_eq!(v94_report.target_cpu, Some("generic".to_string()));
    assert_eq!(v94_report.min_cpu_arch, Some("armv9.4-a".to_string()));
    assert_eq!(v94_report.vector_length, Some("vl128".to_string()));
    assert_eq!(v94_report.target_msvc_arch, Some("/arch:armv9.4".to_string()));
    assert!(v94_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=armv9.4-a+"));
    assert_eq!(v94_report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(v94_report.target_clang_cpu, Some("-m'cpu=generic'".to_string()));
    assert!(v94_report.extensions.contains("lse128"));
    assert_eq!(v94_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(v94_report.filename(), "linux-aarch64-generic-armv9.4-a-vl128.json");

    // 7. Generic + ARMv9.7-A
    let v97_report = ArchFeaturesReport::evaluate_full(Some(Platform::Windows), Some(Arch::Arm64), Some("generic"), None, Some("armv9.7-a"), None, None, None).unwrap();
    assert_eq!(v97_report.target_cpu, Some("generic".to_string()));
    assert_eq!(v97_report.min_cpu_arch, Some("armv9.7-a".to_string()));
    assert_eq!(v97_report.vector_length, Some("vl128".to_string()));
    assert_eq!(v97_report.target_msvc_arch, Some("/arch:armv9.4".to_string()));
    assert!(v97_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=armv9.7-a+"));
    assert_eq!(v97_report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(v97_report.target_clang_cpu, Some("-m'cpu=generic'".to_string()));
    assert!(v97_report.extensions.contains("d128"));
    assert_eq!(v97_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(v97_report.filename(), "windows-aarch64-generic-armv9.7-a-vl128.json");
}

#[test]
fn test_user_min_arch_selection_riscv64_generic() {
    // Riscv64 ignores min_arch because march is processed differently
    let rv_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Riscv64), Some("generic"), None, Some("avx512"), None, None, None).unwrap();
    assert_eq!(rv_report.min_cpu_arch, Some("none".to_string()));
    assert_eq!(rv_report.vector_length, Some("vl128".to_string()));
    assert_eq!(rv_report.target_msvc_arch, None);
    assert!(rv_report.target_clan_arch.as_ref().unwrap().starts_with("-m'arch=rv64"));
    assert_eq!(rv_report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(rv_report.target_clang_cpu, Some("-m'cpu=generic-rv64'".to_string()));
    assert_eq!(rv_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));
    assert_eq!(rv_report.filename(), "linux-riscv64-generic-rv64-none-vl128.json");
}

#[test]
fn test_apxf_os_support() {
    use archinfo::arch::x86_64::xcr0::Xcr0State;

    // Test Xcr0State without OSXSAVE
    let xcr0_disabled = Xcr0State::query(false);
    assert!(!xcr0_disabled.is_apx_usable());
    assert!(!xcr0_disabled.is_avx_usable());
    assert!(!xcr0_disabled.is_avx512_usable());

    // Test apxf feature enabled from target
    let dnr_report = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::X86_64), Some("diamondrapids")).unwrap();
    assert!(dnr_report.extensions.contains("apxf"));
    assert_eq!(dnr_report.features.get("apxf"), Some(&true));

    // Test from_extensions_str
    let f = X64CPUFeatures::from_extensions_str("sse+sse2+apxf");
    assert!(f.apxf);
    assert!(f.apxf_usable);
    assert!(f.has_feature(X64ISA::Apxf));
}

#[test]
fn test_x64_name_mappings() {
    assert_eq!(MSVCX64ISANames::name(MinimumCpuArchitectureX64::AVX2), "AVX2");
    assert_eq!(MSVCX64ISANames::name(MinimumCpuArchitectureX64::AVX512), "AVX512");
    assert_eq!(MSVCX64ISANames::name(MinimumCpuArchitectureX64::None), "SSE4.2");

    assert_eq!(TargetCpuArchitectureX64Names::name(TargetCpuArchitectureX64::Znver3), "znver3");
    assert_eq!(TargetCpuArchitectureX64Names::name(TargetCpuArchitectureX64::Alderlake), "alderlake");

    // MSVCX64ArchTarget
    assert_eq!(MSVCX64ArchTarget::name(MinimumCpuArchitectureX64::None), "/arch:SSE4.2");
    assert_eq!(MSVCX64ArchTarget::name(MinimumCpuArchitectureX64::AVX), "/arch:AVX");
    assert_eq!(MSVCX64ArchTarget::name(MinimumCpuArchitectureX64::AVX2), "/arch:AVX2");
    assert_eq!(MSVCX64ArchTarget::name(MinimumCpuArchitectureX64::AVX512), "/arch:AVX512");
    assert_eq!(MSVCX64ArchTarget::name(MinimumCpuArchitectureX64::AVX10_1), "/arch:AVX10.1");
    assert_eq!(MSVCX64ArchTarget::name(MinimumCpuArchitectureX64::AVX10_2), "/arch:AVX10.2");

    // ClangTargetCpuArchitectureX64NOISANames
    assert_eq!(ClangTargetCpuArchitectureX64NOISANames::name(TargetCpuArchitectureX64::None), "");
    assert_eq!(ClangTargetCpuArchitectureX64NOISANames::name(TargetCpuArchitectureX64::Generic), "");
    assert_eq!(ClangTargetCpuArchitectureX64NOISANames::name(TargetCpuArchitectureX64::X86_64_v4), "fma4+xop+lwp+tbm");
    assert_eq!(ClangTargetCpuArchitectureX64NOISANames::name(TargetCpuArchitectureX64::Diamondrapids), "avx512bmm+mwaitx+rdpru+fma4+xop+lwp+tbm+clzero+sse4a+avx512vp2intersect");

    // ClangX64ISANames (-m{extension})
    assert_eq!(ClangX64ISANames::name(X64ISA::Sse4_1), "sse4.1");
    assert_eq!(ClangX64ISANames::name(X64ISA::Sse4_2), "sse4.2");
    assert_eq!(ClangX64ISANames::name(X64ISA::Avx10_1), "avx10.1");
    assert_eq!(ClangX64ISANames::name(X64ISA::Avx10_2), "avx10.2");
    assert_eq!(ClangX64ISANames::name(X64ISA::AmxAvx512), "amx-avx512");
    assert_eq!(ClangX64ISANames::name(X64ISA::AmxTile), "amx-tile");
    assert_eq!(ClangX64ISANames::name(X64ISA::Apxf), "apxf");

    // ClangX64NOISANames (-mno-{extension})
    assert_eq!(ClangX64NOISANames::name(X64ISA::Sse4_1), "no-sse4.1");
    assert_eq!(ClangX64NOISANames::name(X64ISA::Sse4_2), "no-sse4.2");
    assert_eq!(ClangX64NOISANames::name(X64ISA::Avx10_1), "no-avx10.1");
    assert_eq!(ClangX64NOISANames::name(X64ISA::Avx10_2), "no-avx10.2");
    assert_eq!(ClangX64NOISANames::name(X64ISA::AmxAvx512), "no-amx-avx512");
    assert_eq!(ClangX64NOISANames::name(X64ISA::AmxTile), "no-amx-tile");
    assert_eq!(ClangX64NOISANames::name(X64ISA::Apxf), "no-apxf");
}

#[test]
fn test_arm64_name_mappings() {
    assert_eq!(MinimumCpuArchitectureArm64ClangNames::name(MinimumCpuArchitectureArm64::ARMv8_2A), "armv8.2-a");
    assert_eq!(MinimumCpuArchitectureArm64MSVCNames::name(MinimumCpuArchitectureArm64::ARMv8_2A), "armv8.2");
    assert_eq!(TargetCpuArchitectureArm64Names::name(TargetCpuArchitectureArm64::Apple_M1), "apple-m1");
    assert_eq!(TargetCpuArchitectureArm64Names::name(TargetCpuArchitectureArm64::Cortex_A78C), "cortex-a78c");
}

#[test]
fn test_riscv64_name_mappings() {
    assert_eq!(TargetCpuArchitectureRiscv64Names::name(TargetCpuArchitectureRiscv64::Sifive_P450), "sifive-p450");
    assert_eq!(TargetCpuArchitectureRiscv64Names::name(TargetCpuArchitectureRiscv64::Xiangshan_Nanhu), "xiangshan-nanhu");
    assert_eq!(ClangRiscv64ISANames::name(Riscv64ISA::Zba), "zba");
    assert_eq!(ClangRiscv64NOISANames::name(Riscv64ISA::Zba), "-zba");
}

#[test]
fn test_riscv64_clang_target_extensions_are_valid_isas() {
    let targets = [
        TargetCpuArchitectureRiscv64::None,
        TargetCpuArchitectureRiscv64::Generic,
        TargetCpuArchitectureRiscv64::Generic_RV64,
        TargetCpuArchitectureRiscv64::Native,
        TargetCpuArchitectureRiscv64::MIPS_P8700,
        TargetCpuArchitectureRiscv64::Sifive_P450,
        TargetCpuArchitectureRiscv64::Sifive_P470,
        TargetCpuArchitectureRiscv64::Sifive_P550,
        TargetCpuArchitectureRiscv64::Sifive_P670,
        TargetCpuArchitectureRiscv64::Sifive_P870,
        TargetCpuArchitectureRiscv64::TT_Ascalon_X,
        TargetCpuArchitectureRiscv64::Veyron_V1,
        TargetCpuArchitectureRiscv64::Xiangshan_Kunminghu,
        TargetCpuArchitectureRiscv64::Xiangshan_Nanhu,
        TargetCpuArchitectureRiscv64::XT_C910v2,
        TargetCpuArchitectureRiscv64::XT_C920v2,
    ];

    for target in targets {
        let ext_str = ClangTargetCpuArchitectureRiscv64ISANames::name(target);
        if ext_str.is_empty() {
            continue;
        }
        for token in ext_str.split('+') {
            assert!(
                token.parse::<Riscv64ISA>().is_ok(),
                "Target {:?} contains unparseable/commented-out ISA extension '{}'",
                target,
                token
            );
            assert_ne!(token, "sha", "Target {:?} contains privileged extension 'sha'", target);
            assert_ne!(token, "h", "Target {:?} contains privileged extension 'h'", target);
            assert_ne!(token, "svinval", "Target {:?} contains privileged extension 'svinval'", target);
        }
    }
}

#[test]
fn test_user_enabled_disabled_extensions_x64_generic() {
    // Enable extensions with duplicates: "sha+sha+apxf", disable "sse4.1+sse4.2"
    let report = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("generic"),
        None,
        Some("avx2"),
        Some("sha+sha+apxf"),
        Some("sse4.1+sse4.2"),
        None,
    ).unwrap();

    assert!(report.extensions.contains("sha"));
    assert!(report.extensions.contains("apxf"));
    assert!(!report.extensions.contains("sse4_1"));
    assert!(!report.extensions.contains("sse4_2"));
    assert_eq!(report.features.get("sha"), Some(&true));
    assert_eq!(report.features.get("apxf"), Some(&true));
    assert_eq!(report.features.get("sse4_1"), Some(&false));
    assert_eq!(report.features.get("sse4_2"), Some(&false));

    let isaarch = report.target_clang_isaarch.unwrap();
    assert!(isaarch.contains("-m'sha'"));
    assert!(isaarch.contains("-m'apxf'"));
    assert!(isaarch.contains("-m'no-sse4.1'"));
    assert!(isaarch.contains("-m'no-sse4.2'"));
}

#[test]
fn test_user_enabled_disabled_extensions_arm64_generic() {
    // Enable "sve+sve+sve2", disable "sve2"
    let report = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Arm64),
        Some("generic"),
        None,
        Some("armv8.2-a"),
        Some("sve+sve+sve2"),
        Some("sve2"),
        None,
    ).unwrap();

    assert!(report.extensions.contains("sve"));
    assert!(!report.extensions.contains("sve2"));
    assert_eq!(report.features.get("sve"), Some(&true));
    assert_eq!(report.features.get("sve2"), Some(&false));
    assert_eq!(report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(report.target_clang_cpu, Some("-m'cpu=generic'".to_string()));

    let clan_arch = report.target_clan_arch.unwrap();
    assert!(clan_arch.starts_with("-m'arch=armv8.2-a+"));
    assert!(clan_arch.contains("+sve"));
    assert!(clan_arch.contains("+nosve2"));
}

#[test]
fn test_user_enabled_disabled_extensions_riscv64_generic() {
    // Enable "zba+zbb", disable "c"
    let report = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Riscv64),
        Some("generic"),
        None,
        None,
        Some("zba+zbb"),
        Some("c"),
        None,
    ).unwrap();

    assert!(report.extensions.contains("zba"));
    assert!(report.extensions.contains("zbb"));
    assert!(!report.extensions.contains("+c+"));
    assert_eq!(report.features.get("zba"), Some(&true));
    assert_eq!(report.features.get("c"), Some(&false));
    assert_eq!(report.target_clang_isaarch, Some("-Xclang -target-feature -Xclang '-c'".to_string()));
    assert_eq!(report.target_clang_cpu, Some("-m'cpu=generic-rv64'".to_string()));

    let clan_arch = report.target_clan_arch.unwrap();
    assert!(clan_arch.starts_with("-m'arch=rv64"));
    assert!(clan_arch.contains("_zba"));
    assert!(clan_arch.contains("_zbb"));
    assert!(!clan_arch.contains("_c"));
    assert!(!clan_arch.contains("_no-c"));

    // Multiple disabled extensions: -Xclang -target-feature -Xclang '-ziccif,-zmmul'
    let report_multi = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Riscv64),
        Some("generic"),
        None,
        None,
        None,
        Some("ziccif,zmmul"),
        None,
    ).unwrap();
    assert_eq!(report_multi.target_clang_isaarch, Some("-Xclang -target-feature -Xclang '-ziccif,-zmmul'".to_string()));
    let clan_arch_multi = report_multi.target_clan_arch.unwrap();
    assert!(!clan_arch_multi.contains("ziccif"));
    assert!(!clan_arch_multi.contains("zmmul"));
}

#[test]
fn test_non_generic_ignores_extensions() {
    // Non-generic target should ignore enable/disable extension arguments
    let report = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("znver3"),
        None,
        None,
        Some("apxf"),
        Some("avx2"),
        None,
    ).unwrap();

    assert_eq!(report.target_cpu, Some("znver3".to_string()));
    assert!(report.extensions.contains("avx2"));
    assert!(!report.extensions.contains("apxf"));
    assert_eq!(report.features.get("avx2"), Some(&true));
    assert_eq!(report.features.get("apxf"), Some(&false));

    // Arm64 non-generic target ignores enable/disable extensions
    let arm_report = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Arm64),
        Some("cortex-a78"),
        None,
        None,
        Some("sve2"),
        Some("fp16"),
        None,
    ).unwrap();
    assert_eq!(arm_report.target_cpu, Some("cortex-a78".to_string()));
    assert_eq!(arm_report.features.get("sve2"), Some(&false));
    assert_eq!(arm_report.features.get("fp16"), Some(&true));

    // Riscv64 non-generic target ignores enable/disable extensions
    let rv_report = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Riscv64),
        Some("sifive-p450"),
        None,
        None,
        Some("v"),
        Some("m"),
        None,
    ).unwrap();
    assert_eq!(rv_report.target_cpu, Some("sifive-p450".to_string()));
    assert_eq!(rv_report.features.get("v"), Some(&false));
    assert_eq!(rv_report.features.get("m"), Some(&true));
}

#[test]
fn test_invalid_extensions_return_error() {
    let res = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("generic"),
        None,
        None,
        Some("non_existent_isa_feature"),
        None,
        None,
    );
    assert!(res.is_err());
}

#[test]
fn test_target_clang_extraargs() {
    // x86_64 has empty string
    let x64 = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::X86_64), Some("generic")).unwrap();
    assert_eq!(x64.target_clang_extraargs, Some("".to_string()));
    assert!(x64.to_json().unwrap().contains("\"target_clang_extraargs\": \"\""));

    // arm64 has empty string
    let arm = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::Arm64), Some("generic")).unwrap();
    assert_eq!(arm.target_clang_extraargs, Some("".to_string()));

    // riscv64 generic without experimental extensions has empty string
    let rv = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::Riscv64), Some("generic")).unwrap();
    assert_eq!(rv.target_clang_extraargs, Some("".to_string()));

    // riscv64 generic with experimental extension enabled
    let rv_exp = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Riscv64),
        Some("generic"),
        None,
        None,
        Some("zicfilp"),
        None,
        None,
    ).unwrap();
    assert_eq!(rv_exp.target_clang_extraargs, Some("-m'enable-experimental-extensions'".to_string()));
    assert!(rv_exp.to_json().unwrap().contains("\"target_clang_extraargs\": \"-m'enable-experimental-extensions'\""));

    // riscv64 generic with another experimental extension (p)
    let rv_p = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Riscv64),
        Some("generic"),
        None,
        None,
        Some("p"),
        None,
        None,
    ).unwrap();
    assert_eq!(rv_p.target_clang_extraargs, Some("-m'enable-experimental-extensions'".to_string()));

    // riscv64 generic with experimental extension enabled BUT also disabled
    let rv_dis = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Riscv64),
        Some("generic"),
        None,
        None,
        Some("zicfilp"),
        Some("zicfilp"),
        None,
    ).unwrap();
    assert_eq!(rv_dis.target_clang_extraargs, Some("".to_string()));
}

#[test]
fn test_target_tune_cpu() {
    // 1. User does not select target_cpu -> target_tune_cpu is empty ""
    let report_zero = ArchFeaturesReport::evaluate(None, None, None).unwrap();
    assert_eq!(report_zero.target_cpu, Some("native".to_string()));
    assert_eq!(report_zero.target_tune_cpu, Some("".to_string()));

    let report_no_target = ArchFeaturesReport::evaluate(Some(Platform::Windows), Some(Arch::X86_64), None).unwrap();
    assert_eq!(report_no_target.target_tune_cpu, Some("".to_string()));

    // 2. User selects native target_cpu -> target_tune_cpu is empty ""
    let report_native = ArchFeaturesReport::evaluate(Some(Platform::Windows), Some(Arch::X86_64), Some("native")).unwrap();
    assert_eq!(report_native.target_cpu, Some("native".to_string()));
    assert_eq!(report_native.target_tune_cpu, Some("".to_string()));

    // 3. User selects generic x86_64 target_cpu without target_tune_cpu -> defaults to based target_cpu name
    let report_gen_x64 = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::X86_64), Some("generic")).unwrap();
    assert_eq!(report_gen_x64.target_cpu, Some("x86-64-v2".to_string()));
    assert_eq!(report_gen_x64.target_tune_cpu, Some("x86-64-v2".to_string()));

    // Generic x86_64 with min-arch avx2 -> based target_cpu is x86-64-v3
    let report_gen_avx2 = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("generic"),
        None,
        Some("avx2"),
        None,
        None,
        None,
    ).unwrap();
    assert_eq!(report_gen_avx2.target_cpu, Some("x86-64-v3".to_string()));
    assert_eq!(report_gen_avx2.target_tune_cpu, Some("x86-64-v3".to_string()));

    // 4. User selects known x86_64 target_cpu without target_tune_cpu -> defaults to based target_cpu name
    let report_znver3 = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::X86_64), Some("znver3")).unwrap();
    assert_eq!(report_znver3.target_cpu, Some("znver3".to_string()));
    assert_eq!(report_znver3.target_tune_cpu, Some("znver3".to_string()));

    // 5. User selects known x86_64 target_cpu with valid target_tune_cpu
    let report_tune = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("znver3"),
        Some("alderlake"),
        None,
        None,
        None,
        None,
    ).unwrap();
    assert_eq!(report_tune.target_cpu, Some("znver3".to_string()));
    assert_eq!(report_tune.target_tune_cpu, Some("alderlake".to_string()));

    // Generic x86_64 with explicit target_tune_cpu
    let report_gen_tune = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("generic"),
        Some("znver3"),
        None,
        None,
        None,
        None,
    ).unwrap();
    assert_eq!(report_gen_tune.target_cpu, Some("x86-64-v2".to_string()));
    assert_eq!(report_gen_tune.target_tune_cpu, Some("znver3".to_string()));

    // 6. Native target is not allowed for target_tune_cpu -> returns Err
    let err_native_tune_x64 = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("znver3"),
        Some("native"),
        None,
        None,
        None,
        None,
    );
    assert!(err_native_tune_x64.is_err());

    // 7. Arm64: generic and specific targets without and with tune
    let report_arm_gen = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::Arm64), Some("generic")).unwrap();
    assert_eq!(report_arm_gen.target_cpu, Some("generic".to_string()));
    assert_eq!(report_arm_gen.target_tune_cpu, Some("generic".to_string()));

    let report_arm_tune = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Arm64),
        Some("cortex-a78"),
        Some("apple-m4"),
        None,
        None,
        None,
        None,
    ).unwrap();
    assert_eq!(report_arm_tune.target_cpu, Some("cortex-a78".to_string()));
    assert_eq!(report_arm_tune.target_tune_cpu, Some("apple-m4".to_string()));

    let err_native_tune_arm = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Arm64),
        Some("cortex-a78"),
        Some("native"),
        None,
        None,
        None,
        None,
    );
    assert!(err_native_tune_arm.is_err());

    // 8. Riscv64: generic and specific targets without and with tune
    let report_rv_gen = ArchFeaturesReport::evaluate(Some(Platform::Linux), Some(Arch::Riscv64), Some("generic")).unwrap();
    assert_eq!(report_rv_gen.target_cpu, Some("generic-rv64".to_string()));
    assert_eq!(report_rv_gen.target_tune_cpu, Some("generic-rv64".to_string()));

    let report_rv_tune = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Riscv64),
        Some("sifive-p470"),
        Some("veyron-v1"),
        None,
        None,
        None,
        None,
    ).unwrap();
    assert_eq!(report_rv_tune.target_cpu, Some("sifive-p470".to_string()));
    assert_eq!(report_rv_tune.target_tune_cpu, Some("veyron-v1".to_string()));

    let err_native_tune_rv = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::Riscv64),
        Some("sifive-p470"),
        Some("native"),
        None,
        None,
        None,
        None,
    );
    assert!(err_native_tune_rv.is_err());

    // 9. Invalid target_tune_cpu returns error
    let err_invalid_tune = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("znver3"),
        Some("non_existent_cpu"),
        None,
        None,
        None,
        None,
    );
    assert!(err_invalid_tune.is_err());

    // 10. target_clang_tune_cpu tests
    assert_eq!(report_zero.target_clang_tune_cpu, Some("".to_string()));
    assert_eq!(report_native.target_clang_tune_cpu, Some("".to_string()));
    assert_eq!(report_gen_x64.target_clang_tune_cpu, Some("-m'tune=x86-64-v2'".to_string()));
    assert_eq!(report_znver3.target_clang_tune_cpu, Some("-m'tune=znver3'".to_string()));
    assert_eq!(report_tune.target_clang_tune_cpu, Some("-m'tune=alderlake'".to_string()));
    assert_eq!(report_arm_gen.target_clang_tune_cpu, Some("-m'tune=generic'".to_string()));
    assert_eq!(report_arm_tune.target_clang_tune_cpu, Some("-m'tune=apple-m4'".to_string()));
    assert_eq!(report_rv_gen.target_clang_tune_cpu, Some("-m'tune=generic-rv64'".to_string()));
    assert_eq!(report_rv_tune.target_clang_tune_cpu, Some("-m'tune=veyron-v1'".to_string()));

    // 11. JSON output verifies target_tune_cpu is after target_cpu, target_clang_tune_cpu is after target_clang_cpu
    let json = report_tune.to_json().unwrap();
    let target_cpu_idx = json.find("\"target_cpu\"").unwrap();
    let target_tune_cpu_idx = json.find("\"target_tune_cpu\"").unwrap();
    assert!(target_cpu_idx < target_tune_cpu_idx);
    assert!(json.contains("\"target_tune_cpu\": \"alderlake\""));

    let target_clang_cpu_idx = json.find("\"target_clang_cpu\"").unwrap();
    let target_clang_tune_cpu_idx = json.find("\"target_clang_tune_cpu\"").unwrap();
    assert!(target_clang_cpu_idx < target_clang_tune_cpu_idx);
    assert!(json.contains("\"target_clang_tune_cpu\": \"-m'tune=alderlake'\""));

    let json_native = report_native.to_json().unwrap();
    assert!(json_native.contains("\"target_tune_cpu\": \"\""));
    assert!(json_native.contains("\"target_clang_tune_cpu\": \"\""));
}
