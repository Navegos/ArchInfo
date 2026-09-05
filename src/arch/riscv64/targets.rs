use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Specifies target processor for compiling and optimizing to specifics of Riscv64 micro-architectures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TargetCpuArchitectureRiscv64 {
    #[default]
    None,
    Generic,
    Generic_RV64,
    Native,
    // Andes_AX25,          // X Embedded Linux gateway core (Too weak for UE5)
    // Andes_AX45,          // X In-order embedded pipeline (Too weak for UE5)
    // Andes_AX45MPV,       // X In-order embedded pipeline (Too weak for UE5)
    // Andes_NX45,          // X No MMU (Cannot run Linux)
    MIPS_P8700,             //  Capable (High-performance Out-of-Order application core)
    // Rocket_RV64,         // X Academic/FPGA-only prototyping target (Too weak for UE5)
    Sifive_P450,            //  Capable (Out-of-Order application core)
    Sifive_P470,            //  Capable (Out-of-Order application core)
    Sifive_P550,            //  Capable (Out-of-Order application core, found in EIC7700 hardware)
    Sifive_P670,            //  Capable (High-performance Out-of-Order core)
    Sifive_P870,            //  Capable (High-performance Out-of-Order core)
    // Sifive_S21,          // X No MMU (Cannot run Linux)
    // Sifive_S51,          // X No MMU (Cannot run Linux)
    // Sifive_S54,          // X No MMU (Cannot run Linux)
    // Sifive_S76,          // X No MMU (Cannot run Linux)
    // Sifive_U54,          // X Legacy, weak application core (Unusable for modern UE5)
    // Sifive_U74,          // X Dual-issue in-order core (StarFive JH7110; too slow for UE5)
    // Sifive_X180,         // X AI/Vector focus core, weak scalar performance
    // Sifive_X280,         // X AI/Vector focus core, weak scalar performance
    // Sifive_X390,         // X AI/Vector focus core, weak scalar performance
    // Spacemit_A100,       // X Low performance in-order SoC core
    // Spacemit_X60,        // X Low performance in-order SoC core (Milk-V Jupiter)
    // Spacemit_X100,       // X Low performance in-order SoC core
    // Syntacore_SCR3_RV64, // X No MMU (Cannot run Linux)
    // Syntacore_SCR4_RV64, // X No MMU (Cannot run Linux)
    // Syntacore_SCR5_RV64, // X Embedded Linux core (Too weak for UE5)
    // Syntacore_SCR7,      // X Mid-range core, lacks high-end single-core IPC for UE5
    TT_Ascalon_X,           //  Capable (Tenstorrent server-class Out-of-Order core)
    Veyron_V1,              //  Capable (Ventana datacenter-class Out-of-Order core)
    Xiangshan_Kunminghu,    //  Capable (High-performance open-source Out-of-Order core)
    Xiangshan_Nanhu,        //  Capable (High-performance open-source Out-of-Order core)
    XT_C910v2,              //  Capable (T-Head Out-of-Order core, Milk-V Pioneer)
    XT_C920v2,              //  Capable (T-Head Out-of-Order core with Vector extensions)
}

/// Provides a mapping between TargetCpuArchitectureRiscv64 and string representations.
pub struct TargetCpuArchitectureRiscv64Names;

impl TargetCpuArchitectureRiscv64Names {
    pub fn name(target: TargetCpuArchitectureRiscv64) -> &'static str {
        match target {
            TargetCpuArchitectureRiscv64::None | TargetCpuArchitectureRiscv64::Generic
			| TargetCpuArchitectureRiscv64::Generic_RV64 => "generic-rv64",
            TargetCpuArchitectureRiscv64::Native => "native",
            /* TargetCpuArchitectureRiscv64::Andes_AX25 => "andes-ax25",
            TargetCpuArchitectureRiscv64::Andes_AX45 => "andes-ax45",
            TargetCpuArchitectureRiscv64::Andes_AX45MPV => "andes-ax45mpv",
            TargetCpuArchitectureRiscv64::Andes_NX45 => "andes-nx45", */
            TargetCpuArchitectureRiscv64::MIPS_P8700 => "mips-p8700",
            /* TargetCpuArchitectureRiscv64::Rocket_RV64 => "rocket-rv64", */
            TargetCpuArchitectureRiscv64::Sifive_P450 => "sifive-p450",
            TargetCpuArchitectureRiscv64::Sifive_P470 => "sifive-p470",
            TargetCpuArchitectureRiscv64::Sifive_P550 => "sifive-p550",
            TargetCpuArchitectureRiscv64::Sifive_P670 => "sifive-p670",
            TargetCpuArchitectureRiscv64::Sifive_P870 => "sifive-p870-d",
            /* TargetCpuArchitectureRiscv64::Sifive_S21 => "sifive-s21",
            TargetCpuArchitectureRiscv64::Sifive_S51 => "sifive-s51",
            TargetCpuArchitectureRiscv64::Sifive_S54 => "sifive-s54",
            TargetCpuArchitectureRiscv64::Sifive_S76 => "sifive-s76",
            TargetCpuArchitectureRiscv64::Sifive_U54 => "sifive-u54",
            TargetCpuArchitectureRiscv64::Sifive_U74 => "sifive-u74",
            TargetCpuArchitectureRiscv64::Sifive_X180 => "sifive-x180",
            TargetCpuArchitectureRiscv64::Sifive_X280 => "sifive-x280",
            TargetCpuArchitectureRiscv64::Sifive_X390 => "sifive-x390",
            TargetCpuArchitectureRiscv64::Spacemit_A100 => "spacemit-a100",
            TargetCpuArchitectureRiscv64::Spacemit_X60 => "spacemit-x60",
            TargetCpuArchitectureRiscv64::Spacemit_X100 => "spacemit-X100",
            TargetCpuArchitectureRiscv64::Syntacore_SCR3_RV64 => "syntacore-scr3-rv64",
            TargetCpuArchitectureRiscv64::Syntacore_SCR4_RV64 => "syntacore-scr4-rv64",
            TargetCpuArchitectureRiscv64::Syntacore_SCR5_RV64 => "syntacore-scr5-rv64",
            TargetCpuArchitectureRiscv64::Syntacore_SCR7 => "syntacore-scr7", */
            TargetCpuArchitectureRiscv64::TT_Ascalon_X => "tt-ascalon-x",
            TargetCpuArchitectureRiscv64::Veyron_V1 => "veyron-v1",
            TargetCpuArchitectureRiscv64::Xiangshan_Kunminghu => "xiangshan-kunminghu",
            TargetCpuArchitectureRiscv64::Xiangshan_Nanhu => "xiangshan-nanhu",
            TargetCpuArchitectureRiscv64::XT_C910v2 => "xt-c910v2",
            TargetCpuArchitectureRiscv64::XT_C920v2 => "xt-c920v2",
        }
    }

    #[allow(unused_variables)]
    pub fn requires_experimental_extensions(target: TargetCpuArchitectureRiscv64) -> bool {
        false
        /* matches!(
            target,
            TargetCpuArchitectureRiscv64::Sifive_X180 | TargetCpuArchitectureRiscv64::Sifive_X390
        ) */
    }
}

impl FromStr for TargetCpuArchitectureRiscv64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.to_ascii_lowercase().replace(['_', '.'], "-");
        match norm.as_str() {
            "generic-rv64" | "generic" | "none" | "default" => Ok(TargetCpuArchitectureRiscv64::Generic),
            "native" => Ok(TargetCpuArchitectureRiscv64::Native),
            "mips-p8700" => Ok(TargetCpuArchitectureRiscv64::MIPS_P8700),
            "sifive-p450" => Ok(TargetCpuArchitectureRiscv64::Sifive_P450),
            "sifive-p470" => Ok(TargetCpuArchitectureRiscv64::Sifive_P470),
            "sifive-p550" => Ok(TargetCpuArchitectureRiscv64::Sifive_P550),
            "sifive-p670" => Ok(TargetCpuArchitectureRiscv64::Sifive_P670),
            "sifive-p870-d" => Ok(TargetCpuArchitectureRiscv64::Sifive_P870),
            "tt-ascalon-x" => Ok(TargetCpuArchitectureRiscv64::TT_Ascalon_X),
            "veyron-v1" => Ok(TargetCpuArchitectureRiscv64::Veyron_V1),
            "xiangshan-kunminghu" => Ok(TargetCpuArchitectureRiscv64::Xiangshan_Kunminghu),
            "xiangshan-nanhu" => Ok(TargetCpuArchitectureRiscv64::Xiangshan_Nanhu),
            "xt-c910v2" => Ok(TargetCpuArchitectureRiscv64::XT_C910v2),
            "xt-c920v2" => Ok(TargetCpuArchitectureRiscv64::XT_C920v2),
            other => Err(format!("Unknown RISC-V target CPU: {}", other)),
        }
    }
}

