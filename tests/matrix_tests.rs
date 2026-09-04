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

    assert!(Platform::Nx.is_arch_compatible(Arch::Arm64));
    assert!(Platform::Switch.is_arch_compatible(Arch::Arm64));
    assert!(!Platform::Switch.is_arch_compatible(Arch::X86_64));

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
    let sw1 = ArchFeaturesReport::from_target(Platform::Switch, Arch::Arm64).unwrap();
    assert_eq!(sw1.filename(), "switch-aarch64-cortex-a57-armv8.1-a-vl128.json");

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
    assert!(p470_report.extensions.contains("v"));
    assert!(p470_report.extensions.contains("zvbb"));
    assert_eq!(p470_report.features.get("v"), Some(&true));
    assert_eq!(p470_report.features.get("zvbb"), Some(&true));
    assert_eq!(p470_report.filename(), "linux-riscv64-sifive-p470-none-vl128.json");
}

#[test]
fn test_switch_profiles() {
    let (sw1_ext, _, sw1_arm) = TargetProfile::get_features(Platform::Switch, Arch::Arm64).unwrap();
    assert_eq!(sw1_arm, TargetCpuArchitectureArm64::Cortex_A57);
    assert!(sw1_ext.contains("crypto"));
    assert!(sw1_ext.contains("aes"));
    assert!(sw1_ext.contains("crc"));

    let (sw2_ext, _, sw2_arm) = TargetProfile::get_features(Platform::Switch2, Arch::Arm64).unwrap();
    assert_eq!(sw2_arm, TargetCpuArchitectureArm64::Cortex_A78C);
    assert!(sw2_ext.contains("pauth"));
    assert!(sw2_ext.contains("lse"));
    assert!(sw2_ext.contains("dotprod"));
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
    assert!(json.contains("\"platform\": \"switch\""));
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
    // 1. AVX10: normal is 256. If user sets 512 -> VL512. Otherwise VL256.
    let avx10_default = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("diamondrapids"), None).unwrap();
    assert_eq!(avx10_default.vector_length, Some("vl256".to_string()));

    let avx10_512 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("diamondrapids"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(avx10_512.vector_length, Some("vl512".to_string()));

    let avx10_256 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("diamondrapids"), Some(CpuArchitectureVectorLength::VL256)).unwrap();
    assert_eq!(avx10_256.vector_length, Some("vl256".to_string()));

    // 2. AVX-512F: normal is 512. If user sets 256 -> VL256. Otherwise VL512.
    let avx512_default = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("skylake-avx512"), None).unwrap();
    assert_eq!(avx512_default.vector_length, Some("vl512".to_string()));

    let avx512_req256 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("skylake-avx512"), Some(CpuArchitectureVectorLength::VL256)).unwrap();
    assert_eq!(avx512_req256.vector_length, Some("vl256".to_string()));

    let avx512_req128 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("skylake-avx512"), Some(CpuArchitectureVectorLength::VL128)).unwrap();
    assert_eq!(avx512_req128.vector_length, Some("vl512".to_string()));

    let avx512_req512 = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("skylake-avx512"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(avx512_req512.vector_length, Some("vl512".to_string()));

    // 3. AVX2: normal is 256.
    let avx2_default = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("znver3"), None).unwrap();
    assert_eq!(avx2_default.vector_length, Some("vl256".to_string()));

    // 4. AVX, SSE4.2, Arm64, Riscv64: fixed normal is 128
    let sse_report = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some(CpuArchitectureVectorLength::VL256)).unwrap();
    assert_eq!(sse_report.vector_length, Some("vl128".to_string()));

    let arm_report = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Switch2), Some(Arch::Arm64), Some("cortex-a78c"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(arm_report.vector_length, Some("vl128".to_string()));

    let riscv_report = ArchFeaturesReport::evaluate_with_vl(Some(Platform::Linux), Some(Arch::Riscv64), Some("sifive-p470"), Some(CpuArchitectureVectorLength::VL512)).unwrap();
    assert_eq!(riscv_report.vector_length, Some("vl128".to_string()));
}

#[test]
fn test_user_min_arch_selection_x64_generic() {
    // 1. Generic + AVX
    let avx_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some("avx"), None, None, None).unwrap();
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
    let avx2_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some("avx2"), None, None, None).unwrap();
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
    let avx2_vl128_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some("avx2"), None, None, Some(CpuArchitectureVectorLength::VL128)).unwrap();
    assert_eq!(avx2_vl128_report.vector_length, Some("vl128".to_string()));
    assert_eq!(avx2_vl128_report.target_msvc_vlen, Some("".to_string()));
    assert_eq!(avx2_vl128_report.target_clang_vlen, Some("-m'prefer-vector-width=128'".to_string()));

    // 3. Generic + AVX512 (default VL512 -> target_msvc_vlen: "", target_clang_vlen: "-m'prefer-vector-width=512'")
    let avx512_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some("avx512"), None, None, None).unwrap();
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
    let avx512_vl256_report = ArchFeaturesReport::evaluate_full(Some(Platform::Windows), Some(Arch::X86_64), Some("generic"), Some("avx512"), None, None, Some(CpuArchitectureVectorLength::VL256)).unwrap();
    assert_eq!(avx512_vl256_report.vector_length, Some("vl256".to_string()));
    assert_eq!(avx512_vl256_report.target_msvc_arch, Some("/arch:AVX512".to_string()));
    assert_eq!(avx512_vl256_report.target_msvc_vlen, Some("/vlen=256".to_string()));
    assert_eq!(avx512_vl256_report.target_clang_vlen, Some("-m'prefer-vector-width=256'".to_string()));

    // AVX512 with VL128 should fail with an error
    let avx512_vl128_err = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some("avx512"), None, None, Some(CpuArchitectureVectorLength::VL128));
    assert!(avx512_vl128_err.is_err());

    // 4. Generic + AVX10.1 (default VL256 -> target_msvc_vlen: "", target_clang_vlen: "-m'prefer-vector-width=256'", clang: -m'avx10.1')
    let avx10_1_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some("avx10.1"), None, None, None).unwrap();
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

    // AVX10.1 with VL128 should fail with an error
    let avx10_1_vl128_err = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("generic"), Some("avx10.1"), None, None, Some(CpuArchitectureVectorLength::VL128));
    assert!(avx10_1_vl128_err.is_err());

    // 5. Generic + AVX10.2 with user requested VL512 -> /arch:AVX10.2 /vlen=512, clang: -m'avx10.2-512' -m'prefer-vector-width=512'
    let avx10_2_report = ArchFeaturesReport::evaluate_full(Some(Platform::Windows), Some(Arch::X86_64), Some("generic"), Some("avx10.2"), None, None, Some(CpuArchitectureVectorLength::VL512)).unwrap();
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
    let znver3_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::X86_64), Some("znver3"), Some("avx512"), None, None, None).unwrap();
    assert_eq!(znver3_report.target_cpu, Some("znver3".to_string()));
    assert_eq!(znver3_report.min_cpu_arch, Some("avx2".to_string()));
    assert_eq!(znver3_report.target_clan_arch, Some("-m'arch=znver3'".to_string()));
    assert_eq!(znver3_report.filename(), "linux-x86_64-znver3-avx2-vl256.json");
}

#[test]
fn test_user_min_arch_selection_arm64_generic() {
    // 1. Generic + ARMv8-A
    let v8_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), Some("armv8-a"), None, None, None).unwrap();
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
    let v82_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), Some("armv8.2-a"), None, None, None).unwrap();
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
    let v8r_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), Some("armv8-r"), None, None, None).unwrap();
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
    let v89_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), Some("armv8.9-a"), None, None, None).unwrap();
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
    let v9_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), Some("armv9-a"), None, None, None).unwrap();
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
    let v94_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Arm64), Some("generic"), Some("armv9.4-a"), None, None, None).unwrap();
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
    let v97_report = ArchFeaturesReport::evaluate_full(Some(Platform::Windows), Some(Arch::Arm64), Some("generic"), Some("armv9.7-a"), None, None, None).unwrap();
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
    let rv_report = ArchFeaturesReport::evaluate_full(Some(Platform::Linux), Some(Arch::Riscv64), Some("generic"), Some("avx512"), None, None, None).unwrap();
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
    assert_eq!(ClangRiscv64NOISANames::name(Riscv64ISA::Zba), "no-zba");
}

#[test]
fn test_user_enabled_disabled_extensions_x64_generic() {
    // Enable extensions with duplicates: "sha+sha+apxf", disable "sse4.1+sse4.2"
    let report = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("generic"),
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
        Some("zba+zbb"),
        Some("c"),
        None,
    ).unwrap();

    assert!(report.extensions.contains("zba"));
    assert!(report.extensions.contains("zbb"));
    assert!(!report.extensions.contains("+c+"));
    assert_eq!(report.features.get("zba"), Some(&true));
    assert_eq!(report.features.get("c"), Some(&false));
    assert_eq!(report.target_clang_isaarch, Some("".to_string()));
    assert_eq!(report.target_clang_cpu, Some("-m'cpu=generic-rv64'".to_string()));

    let clan_arch = report.target_clan_arch.unwrap();
    assert!(clan_arch.starts_with("-m'arch=rv64"));
    assert!(clan_arch.contains("_zba"));
    assert!(clan_arch.contains("_zbb"));
    assert!(clan_arch.contains("_no-c"));
}

#[test]
fn test_non_generic_ignores_extensions() {
    // Non-generic target should ignore enable/disable extension arguments
    let report = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("znver3"),
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
}

#[test]
fn test_invalid_extensions_return_error() {
    let res = ArchFeaturesReport::evaluate_full(
        Some(Platform::Linux),
        Some(Arch::X86_64),
        Some("generic"),
        None,
        Some("non_existent_isa_feature"),
        None,
        None,
    );
    assert!(res.is_err());
}
