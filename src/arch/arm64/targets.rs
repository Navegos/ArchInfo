use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Specifies target processor for compiling and optimizing to specifics of Arm64 micro-architectures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TargetCpuArchitectureArm64 {
    #[default]
    None,
    Generic,
    Native,

    // AArch64.v8 A Profile
    // ARMv8-A
    // Cortex_A34,    // X Ultra-low power in-order (Too weak for UE5)
    // Cortex_A35,    // X Ultra-low power in-order (Too weak for UE5)
    // Cortex_A53,    // X Legacy low-tier in-order (Too weak for UE5)
    // Cortex_A57,    // X Legacy, prone to severe thermal throttling (Nintendo Switch era, too old for UE5)
    Cortex_A72,       // ! Borderline (Raspberry Pi 4 class; handles minimal Linux/Android UE5 builds at very low settings)
    Cortex_A73,       // ! Borderline (Legacy mid-tier mobile)
    // Cyclone,       // X Legacy Apple A7 variant (Too old for modern macOS/iOS UE5)
    // Apple_A7,      // X Too old (No modern Vulkan/Metal feature levels)
    // Apple_A8,      // X Too old
    // Apple_A9,      // X Too old
    Exynos_M3,        //  Capable (Samsung custom OoO mobile core)
    Falkor,           //  Capable (Qualcomm server core; great for Linux server/game builds)
    Kryo,             //  Capable (Qualcomm Snapdragon core for Android/Windows on ARM)
    // ThunderX,      // X Early Cavium server core (Terrible single-thread IPC for games)
    // ThunderXT81,   // X Legacy server variant
    // ThunderXT83,   // X Legacy server variant
    // ThunderXT88,   // X Legacy server variant

    // ARMv8.1-A
    Apple_A10,        // ! Minimum viable iOS mobile core
    ThunderX2T99,     //  Capable (Server builds)

    // ARMv8.2-A
    // Cortex_A55,    // X In-order efficiency core (Will severely bottleneck UE5 physics/game thread)
    // Cortex_A65,    // X Throughput-focused embedded core
    // Cortex_A65AE,  // X Automotive safety embedded core
    Cortex_A75,       //  Capable (Mobile/Android baseline)
    Cortex_A76,       //  Capable (Common baseline in modern low-end Android/Windows laptops)
    Cortex_A76AE,     //  Capable (Automotive/Embedded variant)
    Cortex_A77,       //  Capable (Solid OoO performance)
    Cortex_A78,       //  Capable (Excellent baseline for modern Android / Windows on ARM devices)
    Cortex_A78AE,     //  Capable
    Cortex_A78C,      //  Capable (Tablet/Laptop optimized)
    Cortex_X1,        //  Capable (High-performance flagship mobile core)
    Cortex_X1C,       //  Capable (Laptop variant)
    // Neoverse_E1,   // X Data routing/telecom focus architecture
    Neoverse_N1,      //  Capable (AWS Graviton2 instance target for Linux dedicated game servers)
    Graviton2,        //  Capable (Linux cloud game servers)
    Apple_A11,        //  Capable (Mobile)
    Exynos_M4,        //  Capable
    Exynos_M5,        //  Capable
    TSV110,           //  Capable
    A64FX,            //  Capable (Fujitsu HPC target)
    Carmel,           //  Capable (Nvidia Jetson AGX Xavier target; excellent for embedded Linux UE5)

    // ARMv8.3-A
    Saphira,          //  Capable
    ThunderX3T110,    //  Capable
    Apple_A12,        //  Capable
    // Apple_S4,      // X Apple Watch chip (Cannot run desktop/mobile UE5 games)
    // Apple_S5,      // X Apple Watch chip

    // ARMv8.4-A
    Neoverse_V1,      //  Capable (High-performance server/compute)
    Neoverse_512TVB,  //  Capable
    Graviton3,        //  Capable (Linux cloud server)
    Apple_A13,        //  Capable
    Apple_A14,        //  Capable
    Apple_M1,         //  Capable (Excellent baseline for macOS / Windows on ARM via translation)
    // Apple_S6,      // X Apple Watch chip
    // Apple_S7,      // X Apple Watch chip
    // Apple_S8,      // X Apple Watch chip

    // ARMv8.6-A
    Ampere1,          //  Capable (Enterprise Cloud Linux servers)
    Ampere1A,         //  Capable
    Apple_A15,        //  Capable (iOS / iPadOS)
    Apple_A16,        //  Capable
    Apple_A17,        //  Capable
    Apple_M2,         //  Capable (Native macOS target)
    Apple_M3,         //  Capable (Native macOS target)
    // Apple_S9,      // X Apple Watch chip
    // Apple_S10,     // X Apple Watch chip

    // ARMv8.7-A
    Ampere1B,         //  Capable
    Oryon_1,          //  Capable (Snapdragon X Elite; Tier-1 target for Windows on ARM UE5 games)
    Hip12,            //  Capable

    // AArch64.v8 R Profile
    // Cortex_R82,    // X Real-time profile, missing necessary MMU support for standard desktop OSs
    // Cortex_R82AE,  // X Automotive Real-time profile

    // AArch64.v9 A Profile
    // ARMv9-A
    // Cortex_A510,   // X In-order efficiency core (Too weak to act as a standalone primary game execution core)
    Cortex_A710,      //  Capable (Modern Android/Windows flagship cluster core)
    Cortex_A715,      //  Capable
    Cortex_X2,        //  Capable
    Cortex_X3,        //  Capable
    // Neoverse_E2,   // X Low power infrastructure edge core
    Neoverse_N2,      //  Capable
    Neoverse_V2,      //  Capable
    Cobalt_100,       //  Capable (Microsoft Azure custom cloud silicon)
    Grace,            //  Capable (Nvidia superchip platform)

    // ARMv9.2-A
    Ampere1c,         //  Capable
    Cortex_A320,      //  Capable
    // Cortex_A520,   // X In-order efficiency core
    // Cortex_A520AE, // X In-order efficiency core
    Cortex_A720,      //  Capable
    Cortex_A720AE,    //  Capable
    Cortex_A725,      //  Capable
    Cortex_X4,        //  Capable
    Cortex_X925,      //  Capable
    // Neoverse_E3,   // X Low power networking target
    Neoverse_N3,      //  Capable
    Neoverse_V3,      //  Capable
    Neoverse_V3AE,    //  Capable
    Armagicpu,        //  Capable
    Apple_A18,        //  Capable
    Apple_A19,        //  Capable
    Apple_M4,         //  Capable (Native macOS/iPadOS high-end target)
    Apple_M5,         //  Capable
    Gb10,             //  Capable (Google Tensor G4 baseline equivalents)
    Olympus,          //  Capable
    Rigel,            //  Capable

    // ARMv9.3-A
    Fujitsu_Monaka,   //  Capable
    C1_Nano,          //  Capable
    C1_Premium,       //  Capable
    C1_Pro,           //  Capable
    C1_Ultra,         //  Capable
}


/// Minimum baseline CPU architecture for Arm64 code generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MinimumCpuArchitectureArm64 {
    #[default]
    None,
    ARMv8_A,
    ARMv8_1A,
    ARMv8_2A,
    ARMv8_3A,
    ARMv8_4A,
    ARMv8_5A,
    ARMv8_6A,
    ARMv8_7A,
    ARMv8_8A,
    ARMv8_9A,
    ARMv8_R,
    ARMv9_A,
    ARMv9_1A,
    ARMv9_2A,
    ARMv9_3A,
    ARMv9_4A,
    ARMv9_5A,
    ARMv9_6A,
    ARMv9_7A,
}

impl fmt::Display for MinimumCpuArchitectureArm64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MinimumCpuArchitectureArm64::None => write!(f, "none"),
            MinimumCpuArchitectureArm64::ARMv8_A => write!(f, "armv8-a"),
            MinimumCpuArchitectureArm64::ARMv8_1A => write!(f, "armv8.1-a"),
            MinimumCpuArchitectureArm64::ARMv8_2A => write!(f, "armv8.2-a"),
            MinimumCpuArchitectureArm64::ARMv8_3A => write!(f, "armv8.3-a"),
            MinimumCpuArchitectureArm64::ARMv8_4A => write!(f, "armv8.4-a"),
            MinimumCpuArchitectureArm64::ARMv8_5A => write!(f, "armv8.5-a"),
            MinimumCpuArchitectureArm64::ARMv8_6A => write!(f, "armv8.6-a"),
            MinimumCpuArchitectureArm64::ARMv8_7A => write!(f, "armv8.7-a"),
            MinimumCpuArchitectureArm64::ARMv8_8A => write!(f, "armv8.8-a"),
            MinimumCpuArchitectureArm64::ARMv8_9A => write!(f, "armv8.9-a"),
            MinimumCpuArchitectureArm64::ARMv8_R => write!(f, "armv8-r"),
            MinimumCpuArchitectureArm64::ARMv9_A => write!(f, "armv9-a"),
            MinimumCpuArchitectureArm64::ARMv9_1A => write!(f, "armv9.1-a"),
            MinimumCpuArchitectureArm64::ARMv9_2A => write!(f, "armv9.2-a"),
            MinimumCpuArchitectureArm64::ARMv9_3A => write!(f, "armv9.3-a"),
            MinimumCpuArchitectureArm64::ARMv9_4A => write!(f, "armv9.4-a"),
            MinimumCpuArchitectureArm64::ARMv9_5A => write!(f, "armv9.5-a"),
            MinimumCpuArchitectureArm64::ARMv9_6A => write!(f, "armv9.6-a"),
            MinimumCpuArchitectureArm64::ARMv9_7A => write!(f, "armv9.7-a"),
        }
    }
}

impl FromStr for MinimumCpuArchitectureArm64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.trim().to_ascii_lowercase().replace('_', ".");
        match norm.as_str() {
            "none" | "default" => Ok(MinimumCpuArchitectureArm64::None),
            "armv8-a" | "armv8.a" | "armv8" | "armv8.0-a" | "armv8.0" => Ok(MinimumCpuArchitectureArm64::ARMv8_A),
            "armv8.1-a" | "armv8.1a" | "armv8.1" => Ok(MinimumCpuArchitectureArm64::ARMv8_1A),
            "armv8.2-a" | "armv8.2a" | "armv8.2" => Ok(MinimumCpuArchitectureArm64::ARMv8_2A),
            "armv8.3-a" | "armv8.3a" | "armv8.3" => Ok(MinimumCpuArchitectureArm64::ARMv8_3A),
            "armv8.4-a" | "armv8.4a" | "armv8.4" => Ok(MinimumCpuArchitectureArm64::ARMv8_4A),
            "armv8.5-a" | "armv8.5a" | "armv8.5" => Ok(MinimumCpuArchitectureArm64::ARMv8_5A),
            "armv8.6-a" | "armv8.6a" | "armv8.6" => Ok(MinimumCpuArchitectureArm64::ARMv8_6A),
            "armv8.7-a" | "armv8.7a" | "armv8.7" => Ok(MinimumCpuArchitectureArm64::ARMv8_7A),
            "armv8.8-a" | "armv8.8a" | "armv8.8" => Ok(MinimumCpuArchitectureArm64::ARMv8_8A),
            "armv8.9-a" | "armv8.9a" | "armv8.9" => Ok(MinimumCpuArchitectureArm64::ARMv8_9A),
            "armv8-r" | "armv8.r" | "armv8r" => Ok(MinimumCpuArchitectureArm64::ARMv8_R),
            "armv9-a" | "armv9.a" | "armv9" | "armv9.0-a" | "armv9.0" => Ok(MinimumCpuArchitectureArm64::ARMv9_A),
            "armv9.1-a" | "armv9.1a" | "armv9.1" => Ok(MinimumCpuArchitectureArm64::ARMv9_1A),
            "armv9.2-a" | "armv9.2a" | "armv9.2" => Ok(MinimumCpuArchitectureArm64::ARMv9_2A),
            "armv9.3-a" | "armv9.3a" | "armv9.3" => Ok(MinimumCpuArchitectureArm64::ARMv9_3A),
            "armv9.4-a" | "armv9.4a" | "armv9.4" => Ok(MinimumCpuArchitectureArm64::ARMv9_4A),
            "armv9.5-a" | "armv9.5a" | "armv9.5" => Ok(MinimumCpuArchitectureArm64::ARMv9_5A),
            "armv9.6-a" | "armv9.6a" | "armv9.6" => Ok(MinimumCpuArchitectureArm64::ARMv9_6A),
            "armv9.7-a" | "armv9.7a" | "armv9.7" => Ok(MinimumCpuArchitectureArm64::ARMv9_7A),
            other => Err(format!("Unknown MinimumCpuArchitectureArm64: '{}'", other)),
        }
    }
}

/// Mapping between TargetCpuArchitectureArm64 and string representations.
pub struct TargetCpuArchitectureArm64Names;

impl TargetCpuArchitectureArm64Names {
    pub fn name(target: TargetCpuArchitectureArm64) -> &'static str {
        match target {
            TargetCpuArchitectureArm64::None | TargetCpuArchitectureArm64::Generic => "generic",
            TargetCpuArchitectureArm64::Native => "native",
            /* TargetCpuArchitectureArm64::Cortex_A34 => "cortex-a34",
            TargetCpuArchitectureArm64::Cortex_A35 => "cortex-a35",
            TargetCpuArchitectureArm64::Cortex_A53 => "cortex-a53",
            TargetCpuArchitectureArm64::Cortex_A57 => "cortex-a57", */
            TargetCpuArchitectureArm64::Cortex_A72 => "cortex-a72",
            TargetCpuArchitectureArm64::Cortex_A73 => "cortex-a73",
            /* TargetCpuArchitectureArm64::Cyclone => "cyclone",
            TargetCpuArchitectureArm64::Apple_A7 => "apple-a7",
            TargetCpuArchitectureArm64::Apple_A8 => "apple-a8",
            TargetCpuArchitectureArm64::Apple_A9 => "apple-a9", */
            TargetCpuArchitectureArm64::Exynos_M3 => "exynos-m3",
            TargetCpuArchitectureArm64::Falkor => "falkor",
            TargetCpuArchitectureArm64::Kryo => "kryo",
            /* TargetCpuArchitectureArm64::ThunderX => "thunderx",
            TargetCpuArchitectureArm64::ThunderXT81 => "thunderxt81",
            TargetCpuArchitectureArm64::ThunderXT83 => "thunderxt83",
            TargetCpuArchitectureArm64::ThunderXT88 => "thunderxt88", */
            TargetCpuArchitectureArm64::Apple_A10 => "apple-a10",
            TargetCpuArchitectureArm64::ThunderX2T99 => "thunderx2t99",
            /* TargetCpuArchitectureArm64::Cortex_A55 => "cortex-a55",
            TargetCpuArchitectureArm64::Cortex_A65 => "cortex-a65",
            TargetCpuArchitectureArm64::Cortex_A65AE => "cortex-a65ae", */
            TargetCpuArchitectureArm64::Cortex_A75 => "cortex-a75",
            TargetCpuArchitectureArm64::Cortex_A76 => "cortex-a76",
            TargetCpuArchitectureArm64::Cortex_A76AE => "cortex-a76ae",
            TargetCpuArchitectureArm64::Cortex_A77 => "cortex-a77",
            TargetCpuArchitectureArm64::Cortex_A78 => "cortex-a78",
            TargetCpuArchitectureArm64::Cortex_A78AE => "cortex-a78ae",
            TargetCpuArchitectureArm64::Cortex_A78C => "cortex-a78c",
            TargetCpuArchitectureArm64::Cortex_X1 => "cortex-x1",
            TargetCpuArchitectureArm64::Cortex_X1C => "cortex-x1c",
            /* TargetCpuArchitectureArm64::Neoverse_E1 => "neoverse-e1", */
            TargetCpuArchitectureArm64::Neoverse_N1 => "neoverse-n1",
            TargetCpuArchitectureArm64::Graviton2 => "neoverse-n1",
            TargetCpuArchitectureArm64::Apple_A11 => "apple-a11",
            TargetCpuArchitectureArm64::Exynos_M4 => "exynos-m4",
            TargetCpuArchitectureArm64::Exynos_M5 => "exynos-m5",
            TargetCpuArchitectureArm64::TSV110 => "tsv110",
            TargetCpuArchitectureArm64::A64FX => "a64fx",
            TargetCpuArchitectureArm64::Carmel => "carmel",
            TargetCpuArchitectureArm64::Saphira => "saphira",
            TargetCpuArchitectureArm64::ThunderX3T110 => "thunderx3t110",
            TargetCpuArchitectureArm64::Apple_A12 => "apple-a12",
            /* TargetCpuArchitectureArm64::Apple_S4 => "apple-s4",
            TargetCpuArchitectureArm64::Apple_S5 => "apple-s5", */
            TargetCpuArchitectureArm64::Neoverse_V1 => "neoverse-v1",
            TargetCpuArchitectureArm64::Neoverse_512TVB => "neoverse-512tvb",
            TargetCpuArchitectureArm64::Graviton3 => "neoverse-v1",
            TargetCpuArchitectureArm64::Apple_A13 => "apple-a13",
            TargetCpuArchitectureArm64::Apple_A14 => "apple-a14",
            TargetCpuArchitectureArm64::Apple_M1 => "apple-m1",
            /* TargetCpuArchitectureArm64::Apple_S6 => "apple-s6",
            TargetCpuArchitectureArm64::Apple_S7 => "apple-s7",
            TargetCpuArchitectureArm64::Apple_S8 => "apple-s8", */
            TargetCpuArchitectureArm64::Ampere1 => "ampere1",
            TargetCpuArchitectureArm64::Ampere1A => "ampere1a",
            TargetCpuArchitectureArm64::Apple_A15 => "apple-a15",
            TargetCpuArchitectureArm64::Apple_A16 => "apple-a16",
            TargetCpuArchitectureArm64::Apple_A17 => "apple-a17",
            TargetCpuArchitectureArm64::Apple_M2 => "apple-m2",
            TargetCpuArchitectureArm64::Apple_M3 => "apple-m3",
            /* TargetCpuArchitectureArm64::Apple_S9 => "apple-s9",
            TargetCpuArchitectureArm64::Apple_S10 => "apple-s10", */
            TargetCpuArchitectureArm64::Ampere1B => "ampere1b",
            TargetCpuArchitectureArm64::Oryon_1 => "oryon-1",
            TargetCpuArchitectureArm64::Hip12 => "hip12",
            /* TargetCpuArchitectureArm64::Cortex_R82 => "cortex-r82",
            TargetCpuArchitectureArm64::Cortex_R82AE => "cortex-r82ae",
            TargetCpuArchitectureArm64::Cortex_A510 => "cortex-a510", */
            TargetCpuArchitectureArm64::Cortex_A710 => "cortex-a710",
            TargetCpuArchitectureArm64::Cortex_A715 => "cortex-a715",
            TargetCpuArchitectureArm64::Cortex_X2 => "cortex-x2",
            TargetCpuArchitectureArm64::Cortex_X3 => "cortex-x3",
            /* TargetCpuArchitectureArm64::Neoverse_E2 => "cortex-a510", */
            TargetCpuArchitectureArm64::Neoverse_N2 => "neoverse-n2",
            TargetCpuArchitectureArm64::Neoverse_V2 => "neoverse-v2",
            TargetCpuArchitectureArm64::Cobalt_100 => "cobalt-100",
            TargetCpuArchitectureArm64::Grace => "grace",
            TargetCpuArchitectureArm64::Ampere1c => "ampere1c",
            TargetCpuArchitectureArm64::Cortex_A320 => "cortex-a320",
            /* TargetCpuArchitectureArm64::Cortex_A520 => "cortex-a520",
            TargetCpuArchitectureArm64::Cortex_A520AE => "cortex-a520ae", */
            TargetCpuArchitectureArm64::Cortex_A720 => "cortex-a720",
            TargetCpuArchitectureArm64::Cortex_A720AE => "cortex-a720ae",
            TargetCpuArchitectureArm64::Cortex_A725 => "cortex-a725",
            TargetCpuArchitectureArm64::Cortex_X4 => "cortex-x4",
            TargetCpuArchitectureArm64::Cortex_X925 => "cortex-x925",
            /* TargetCpuArchitectureArm64::Neoverse_E3 => "cortex-a520", */
            TargetCpuArchitectureArm64::Neoverse_N3 => "neoverse-n3",
            TargetCpuArchitectureArm64::Neoverse_V3 => "neoverse-v3",
            TargetCpuArchitectureArm64::Neoverse_V3AE => "neoverse-v3ae",
            TargetCpuArchitectureArm64::Armagicpu => "armagicpu",
            TargetCpuArchitectureArm64::Apple_A18 => "apple-a18",
            TargetCpuArchitectureArm64::Apple_A19 => "apple-a19",
            TargetCpuArchitectureArm64::Apple_M4 => "apple-m4",
            TargetCpuArchitectureArm64::Apple_M5 => "apple-m5",
            TargetCpuArchitectureArm64::Gb10 => "gb10",
            TargetCpuArchitectureArm64::Olympus => "olympus",
            TargetCpuArchitectureArm64::Rigel => "rigel",
            TargetCpuArchitectureArm64::Fujitsu_Monaka => "fujitsu-monaka",
            TargetCpuArchitectureArm64::C1_Nano => "c1-nano",
            TargetCpuArchitectureArm64::C1_Premium => "c1-premium",
            TargetCpuArchitectureArm64::C1_Pro => "c1-pro",
            TargetCpuArchitectureArm64::C1_Ultra => "c1-ultra",
        }
    }
}

impl FromStr for TargetCpuArchitectureArm64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.to_ascii_lowercase().replace(['_', '.'], "-");
        match norm.as_str() {
            "generic" | "none" | "default" => Ok(TargetCpuArchitectureArm64::Generic),
            "native" => Ok(TargetCpuArchitectureArm64::Native),
            /* "cortex-a34" => Ok(TargetCpuArchitectureArm64::Cortex_A34),
            "cortex-a35" => Ok(TargetCpuArchitectureArm64::Cortex_A35),
            "cortex-a53" => Ok(TargetCpuArchitectureArm64::Cortex_A53),
            "cortex-a57" => Ok(TargetCpuArchitectureArm64::Cortex_A57), */
            "cortex-a72" => Ok(TargetCpuArchitectureArm64::Cortex_A72),
            "cortex-a73" => Ok(TargetCpuArchitectureArm64::Cortex_A73),
            /* "cyclone" => Ok(TargetCpuArchitectureArm64::Cyclone),
            "apple-a7" => Ok(TargetCpuArchitectureArm64::Apple_A7),
            "apple-a8" => Ok(TargetCpuArchitectureArm64::Apple_A8),
            "apple-a9" => Ok(TargetCpuArchitectureArm64::Apple_A9), */
            "exynos-m3" => Ok(TargetCpuArchitectureArm64::Exynos_M3),
            "falkor" => Ok(TargetCpuArchitectureArm64::Falkor),
            "kryo" => Ok(TargetCpuArchitectureArm64::Kryo),
            /* "thunderx" => Ok(TargetCpuArchitectureArm64::ThunderX),
            "thunderxt81" => Ok(TargetCpuArchitectureArm64::ThunderXT81),
            "thunderxt83" => Ok(TargetCpuArchitectureArm64::ThunderXT83),
            "thunderxt88" => Ok(TargetCpuArchitectureArm64::ThunderXT88), */
            "apple-a10" => Ok(TargetCpuArchitectureArm64::Apple_A10),
            "thunderx2t99" => Ok(TargetCpuArchitectureArm64::ThunderX2T99),
            /* "cortex-a55" => Ok(TargetCpuArchitectureArm64::Cortex_A55),
            "cortex-a65" => Ok(TargetCpuArchitectureArm64::Cortex_A65),
            "cortex-a65ae" => Ok(TargetCpuArchitectureArm64::Cortex_A65AE), */
            "cortex-a75" => Ok(TargetCpuArchitectureArm64::Cortex_A75),
            "cortex-a76" => Ok(TargetCpuArchitectureArm64::Cortex_A76),
            "cortex-a76ae" => Ok(TargetCpuArchitectureArm64::Cortex_A76AE),
            "cortex-a77" => Ok(TargetCpuArchitectureArm64::Cortex_A77),
            "cortex-a78" => Ok(TargetCpuArchitectureArm64::Cortex_A78),
            "cortex-a78ae" => Ok(TargetCpuArchitectureArm64::Cortex_A78AE),
            "cortex-a78c" => Ok(TargetCpuArchitectureArm64::Cortex_A78C),
            "cortex-x1" => Ok(TargetCpuArchitectureArm64::Cortex_X1),
            "cortex-x1c" => Ok(TargetCpuArchitectureArm64::Cortex_X1C),
            /* "neoverse-e1" => Ok(TargetCpuArchitectureArm64::Neoverse_E1), */
            "neoverse-n1" => Ok(TargetCpuArchitectureArm64::Neoverse_N1),
            "graviton2" => Ok(TargetCpuArchitectureArm64::Graviton2),
            "apple-a11" => Ok(TargetCpuArchitectureArm64::Apple_A11),
            "exynos-m4" => Ok(TargetCpuArchitectureArm64::Exynos_M4),
            "exynos-m5" => Ok(TargetCpuArchitectureArm64::Exynos_M5),
            "tsv110" => Ok(TargetCpuArchitectureArm64::TSV110),
            "a64fx" => Ok(TargetCpuArchitectureArm64::A64FX),
            "carmel" => Ok(TargetCpuArchitectureArm64::Carmel),
            "saphira" => Ok(TargetCpuArchitectureArm64::Saphira),
            "thunderx3t110" => Ok(TargetCpuArchitectureArm64::ThunderX3T110),
            "apple-a12" => Ok(TargetCpuArchitectureArm64::Apple_A12),
            /* "apple-s4" => Ok(TargetCpuArchitectureArm64::Apple_S4),
            "apple-s5" => Ok(TargetCpuArchitectureArm64::Apple_S5), */
            "neoverse-v1" => Ok(TargetCpuArchitectureArm64::Neoverse_V1),
            "neoverse-512tvb" => Ok(TargetCpuArchitectureArm64::Neoverse_512TVB),
            "graviton3" => Ok(TargetCpuArchitectureArm64::Graviton3),
            "apple-a13" => Ok(TargetCpuArchitectureArm64::Apple_A13),
            "apple-a14" => Ok(TargetCpuArchitectureArm64::Apple_A14),
            "apple-m1" => Ok(TargetCpuArchitectureArm64::Apple_M1),
            /* "apple-s6" => Ok(TargetCpuArchitectureArm64::Apple_S6),
            "apple-s7" => Ok(TargetCpuArchitectureArm64::Apple_S7),
            "apple-s8" => Ok(TargetCpuArchitectureArm64::Apple_S8), */
            "ampere1" => Ok(TargetCpuArchitectureArm64::Ampere1),
            "ampere1a" => Ok(TargetCpuArchitectureArm64::Ampere1A),
            "apple-a15" => Ok(TargetCpuArchitectureArm64::Apple_A15),
            "apple-a16" => Ok(TargetCpuArchitectureArm64::Apple_A16),
            "apple-a17" => Ok(TargetCpuArchitectureArm64::Apple_A17),
            "apple-m2" => Ok(TargetCpuArchitectureArm64::Apple_M2),
            "apple-m3" => Ok(TargetCpuArchitectureArm64::Apple_M3),
            /* "apple-s9" => Ok(TargetCpuArchitectureArm64::Apple_S9),
            "apple-s10" => Ok(TargetCpuArchitectureArm64::Apple_S10), */
            "ampere1b" => Ok(TargetCpuArchitectureArm64::Ampere1B),
            "oryon-1" | "oryon1" => Ok(TargetCpuArchitectureArm64::Oryon_1),
            "hip12" => Ok(TargetCpuArchitectureArm64::Hip12),
            /* "cortex-r82" => Ok(TargetCpuArchitectureArm64::Cortex_R82),
            "cortex-r82ae" => Ok(TargetCpuArchitectureArm64::Cortex_R82AE),
            "cortex-a510" => Ok(TargetCpuArchitectureArm64::Cortex_A510), */
            "cortex-a710" => Ok(TargetCpuArchitectureArm64::Cortex_A710),
            "cortex-a715" => Ok(TargetCpuArchitectureArm64::Cortex_A715),
            "cortex-x2" => Ok(TargetCpuArchitectureArm64::Cortex_X2),
            "cortex-x3" => Ok(TargetCpuArchitectureArm64::Cortex_X3),
            /* "neoverse-e2" => Ok(TargetCpuArchitectureArm64::Neoverse_E2), */
            "neoverse-n2" => Ok(TargetCpuArchitectureArm64::Neoverse_N2),
            "neoverse-v2" => Ok(TargetCpuArchitectureArm64::Neoverse_V2),
            "cobalt-100" => Ok(TargetCpuArchitectureArm64::Cobalt_100),
            "grace" => Ok(TargetCpuArchitectureArm64::Grace),
            "ampere1c" => Ok(TargetCpuArchitectureArm64::Ampere1c),
            "cortex-a320" => Ok(TargetCpuArchitectureArm64::Cortex_A320),
            /* "cortex-a520" => Ok(TargetCpuArchitectureArm64::Cortex_A520),
            "cortex-a520ae" => Ok(TargetCpuArchitectureArm64::Cortex_A520AE), */
            "cortex-a720" => Ok(TargetCpuArchitectureArm64::Cortex_A720),
            "cortex-a720ae" => Ok(TargetCpuArchitectureArm64::Cortex_A720AE),
            "cortex-a725" => Ok(TargetCpuArchitectureArm64::Cortex_A725),
            "cortex-x4" => Ok(TargetCpuArchitectureArm64::Cortex_X4),
            "cortex-x925" => Ok(TargetCpuArchitectureArm64::Cortex_X925),
            /* "neoverse-e3" => Ok(TargetCpuArchitectureArm64::Neoverse_E3), */
            "neoverse-n3" => Ok(TargetCpuArchitectureArm64::Neoverse_N3),
            "neoverse-v3" => Ok(TargetCpuArchitectureArm64::Neoverse_V3),
            "neoverse-v3ae" => Ok(TargetCpuArchitectureArm64::Neoverse_V3AE),
            "armagicpu" => Ok(TargetCpuArchitectureArm64::Armagicpu),
            "apple-a18" => Ok(TargetCpuArchitectureArm64::Apple_A18),
            "apple-a19" => Ok(TargetCpuArchitectureArm64::Apple_A19),
            "apple-m4" => Ok(TargetCpuArchitectureArm64::Apple_M4),
            "apple-m5" => Ok(TargetCpuArchitectureArm64::Apple_M5),
            "gb10" => Ok(TargetCpuArchitectureArm64::Gb10),
            "olympus" => Ok(TargetCpuArchitectureArm64::Olympus),
            "rigel" => Ok(TargetCpuArchitectureArm64::Rigel),
            "fujitsu-monaka" => Ok(TargetCpuArchitectureArm64::Fujitsu_Monaka),
            "c1-nano" => Ok(TargetCpuArchitectureArm64::C1_Nano),
            "c1-premium" => Ok(TargetCpuArchitectureArm64::C1_Premium),
            "c1-pro" => Ok(TargetCpuArchitectureArm64::C1_Pro),
            "c1-ultra" => Ok(TargetCpuArchitectureArm64::C1_Ultra),
            other => Err(format!("Unknown Arm64 target CPU: {}", other)),
        }
    }
}

