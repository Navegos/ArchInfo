pub mod isa;
pub mod linux;
pub mod names;
pub mod targets;

use std::collections::{BTreeMap, HashSet};
use serde::{Deserialize, Serialize};

pub use isa::Riscv64ISA;
pub use names::*;
pub use targets::*;

/// Represents the detected or configured CPU features for Riscv64 architecture.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Riscv64CPUFeatures {
    pub i: bool,
    pub m: bool,
    pub a: bool,
    pub f: bool,
    pub d: bool,
    pub c: bool,
    pub b: bool,
    pub v: bool,
    pub zba: bool,
    pub zbb: bool,
    pub zbc: bool,
    pub zbs: bool,
    pub zbkb: bool,
    pub zbkc: bool,
    pub zbkx: bool,
    pub zknd: bool,
    pub zkne: bool,
    pub zknh: bool,
    pub zkr: bool,
    pub zksed: bool,
    pub zksh: bool,
    pub zkt: bool,
    pub zic64b: bool,
    pub zicbom: bool,
    pub zicbop: bool,
    pub zicboz: bool,
    pub zicsr: bool,
    pub zifencei: bool,
    pub zihintpause: bool,
    pub zacas: bool,
    pub zicond: bool,
    pub zve32f: bool,
    pub zve64d: bool,
    pub zvbb: bool,
    pub zvbc: bool,
    pub zvkb: bool,
    pub zvkg: bool,
    pub zvkned: bool,
    pub zvl128b: bool,
    pub zvl256b: bool,
    pub zvl512b: bool,
}

impl Riscv64CPUFeatures {
    /// Detects features on the host RISC-V machine
    pub fn detect_host() -> Self {
        #[allow(unused_mut)]
        let mut f = Self::default();

        #[cfg(all(target_os = "linux", target_arch = "riscv64"))]
        {
            let probe = linux::LinuxRiscv64Probe::query();
            f.i = probe.has_i();
            f.m = probe.has_m();
            f.a = probe.has_a();
            f.f = probe.has_f();
            f.d = probe.has_d();
            f.c = probe.has_c();
            f.v = probe.has_v();
            f.zicsr = true;
            f.zifencei = true;
        }

        f
    }

    /// Constructs features from a '+' delimited extension string
    pub fn from_extensions_str(ext_str: &str) -> Self {
        let tokens: HashSet<&str> = ext_str.split('+').filter(|s| !s.is_empty()).collect();
        let mut f = Self::default();
        f.i = tokens.contains("i");
        f.m = tokens.contains("m");
        f.a = tokens.contains("a");
        f.f = tokens.contains("f");
        f.d = tokens.contains("d");
        f.c = tokens.contains("c");
        f.b = tokens.contains("b");
        f.v = tokens.contains("v");
        f.zba = tokens.contains("zba");
        f.zbb = tokens.contains("zbb");
        f.zbc = tokens.contains("zbc");
        f.zbs = tokens.contains("zbs");
        f.zbkb = tokens.contains("zbkb");
        f.zbkc = tokens.contains("zbkc");
        f.zbkx = tokens.contains("zbkx");
        f.zknd = tokens.contains("zknd");
        f.zkne = tokens.contains("zkne");
        f.zknh = tokens.contains("zknh");
        f.zkr = tokens.contains("zkr");
        f.zksed = tokens.contains("zksed");
        f.zksh = tokens.contains("zksh");
        f.zkt = tokens.contains("zkt");
        f.zic64b = tokens.contains("zic64b");
        f.zicbom = tokens.contains("zicbom");
        f.zicbop = tokens.contains("zicbop");
        f.zicboz = tokens.contains("zicboz");
        f.zicsr = tokens.contains("zicsr");
        f.zifencei = tokens.contains("zifencei");
        f.zihintpause = tokens.contains("zihintpause");
        f.zacas = tokens.contains("zacas");
        f.zicond = tokens.contains("zicond");
        f.zve32f = tokens.contains("zve32f");
        f.zve64d = tokens.contains("zve64d");
        f.zvbb = tokens.contains("zvbb");
        f.zvbc = tokens.contains("zvbc");
        f.zvkb = tokens.contains("zvkb");
        f.zvkg = tokens.contains("zvkg");
        f.zvkned = tokens.contains("zvkned");
        f.zvl128b = tokens.contains("zvl128b");
        f.zvl256b = tokens.contains("zvl256b");
        f.zvl512b = tokens.contains("zvl512b");
        f
    }

    /// Constructs features from a known TargetCpuArchitectureRiscv64
    pub fn from_target(target: TargetCpuArchitectureRiscv64) -> Self {
        match target {
            TargetCpuArchitectureRiscv64::Native => Self::detect_host(),
            other => {
                let ext = ClangTargetCpuArchitectureRiscv64ISANames::name(other);
                Self::from_extensions_str(ext)
            }
        }
    }

    pub fn has_feature(&self, isa: Riscv64ISA) -> bool {
        match isa {
            Riscv64ISA::I => self.i,
            Riscv64ISA::M => self.m,
            Riscv64ISA::A => self.a,
            Riscv64ISA::F => self.f,
            Riscv64ISA::D => self.d,
            Riscv64ISA::C => self.c,
            Riscv64ISA::B => self.b,
            Riscv64ISA::V => self.v,
            Riscv64ISA::Zba => self.zba,
            Riscv64ISA::Zbb => self.zbb,
            Riscv64ISA::Zbc => self.zbc,
            Riscv64ISA::Zbs => self.zbs,
            Riscv64ISA::Zbkb => self.zbkb,
            Riscv64ISA::Zbkc => self.zbkc,
            Riscv64ISA::Zbkx => self.zbkx,
            Riscv64ISA::Zknd => self.zknd,
            Riscv64ISA::Zkne => self.zkne,
            Riscv64ISA::Zknh => self.zknh,
            Riscv64ISA::Zkr => self.zkr,
            Riscv64ISA::Zksed => self.zksed,
            Riscv64ISA::Zksh => self.zksh,
            Riscv64ISA::Zkt => self.zkt,
            Riscv64ISA::Zic64b => self.zic64b,
            Riscv64ISA::Zicbom => self.zicbom,
            Riscv64ISA::Zicbop => self.zicbop,
            Riscv64ISA::Zicboz => self.zicboz,
            Riscv64ISA::Zicsr => self.zicsr,
            Riscv64ISA::Zifencei => self.zifencei,
            Riscv64ISA::Zihintpause => self.zihintpause,
            Riscv64ISA::Zacas => self.zacas,
            Riscv64ISA::Zicond => self.zicond,
            Riscv64ISA::Zve32f => self.zve32f,
            Riscv64ISA::Zve64d => self.zve64d,
            Riscv64ISA::Zvbb => self.zvbb,
            Riscv64ISA::Zvbc => self.zvbc,
            Riscv64ISA::Zvkb => self.zvkb,
            Riscv64ISA::Zvkg => self.zvkg,
            Riscv64ISA::Zvkned => self.zvkned,
            Riscv64ISA::Zvl128b => self.zvl128b,
            Riscv64ISA::Zvl256b => self.zvl256b,
            Riscv64ISA::Zvl512b => self.zvl512b,
            _ => false,
        }
    }

    pub fn set_feature(&mut self, isa: Riscv64ISA, enabled: bool) {
        match isa {
            Riscv64ISA::I => self.i = enabled,
            Riscv64ISA::M => self.m = enabled,
            Riscv64ISA::A => self.a = enabled,
            Riscv64ISA::F => self.f = enabled,
            Riscv64ISA::D => self.d = enabled,
            Riscv64ISA::C => self.c = enabled,
            Riscv64ISA::B => self.b = enabled,
            Riscv64ISA::V => self.v = enabled,
            Riscv64ISA::Zba => self.zba = enabled,
            Riscv64ISA::Zbb => self.zbb = enabled,
            Riscv64ISA::Zbc => self.zbc = enabled,
            Riscv64ISA::Zbs => self.zbs = enabled,
            Riscv64ISA::Zbkb => self.zbkb = enabled,
            Riscv64ISA::Zbkc => self.zbkc = enabled,
            Riscv64ISA::Zbkx => self.zbkx = enabled,
            Riscv64ISA::Zknd => self.zknd = enabled,
            Riscv64ISA::Zkne => self.zkne = enabled,
            Riscv64ISA::Zknh => self.zknh = enabled,
            Riscv64ISA::Zkr => self.zkr = enabled,
            Riscv64ISA::Zksed => self.zksed = enabled,
            Riscv64ISA::Zksh => self.zksh = enabled,
            Riscv64ISA::Zkt => self.zkt = enabled,
            Riscv64ISA::Zic64b => self.zic64b = enabled,
            Riscv64ISA::Zicbom => self.zicbom = enabled,
            Riscv64ISA::Zicbop => self.zicbop = enabled,
            Riscv64ISA::Zicboz => self.zicboz = enabled,
            Riscv64ISA::Zicsr => self.zicsr = enabled,
            Riscv64ISA::Zifencei => self.zifencei = enabled,
            Riscv64ISA::Zihintpause => self.zihintpause = enabled,
            Riscv64ISA::Zacas => self.zacas = enabled,
            Riscv64ISA::Zicond => self.zicond = enabled,
            Riscv64ISA::Zve32f => self.zve32f = enabled,
            Riscv64ISA::Zve64d => self.zve64d = enabled,
            Riscv64ISA::Zvbb => self.zvbb = enabled,
            Riscv64ISA::Zvbc => self.zvbc = enabled,
            Riscv64ISA::Zvkb => self.zvkb = enabled,
            Riscv64ISA::Zvkg => self.zvkg = enabled,
            Riscv64ISA::Zvkned => self.zvkned = enabled,
            Riscv64ISA::Zvl128b => self.zvl128b = enabled,
            Riscv64ISA::Zvl256b => self.zvl256b = enabled,
            Riscv64ISA::Zvl512b => self.zvl512b = enabled,
            _ => {}
        }
    }

    /// Generates Clang ISA argument (-m'arch=rv64<exts>') using ClangRiscv64ISANames and ClangRiscv64NOISANames
    pub fn generate_clang_isaarch(&self, disabled_isas: &[Riscv64ISA]) -> String {
        let mut parts = Vec::new();
        for &isa in Riscv64ISA::all() {
            if self.has_feature(isa) && !disabled_isas.contains(&isa) {
                let name = ClangRiscv64ISANames::name(isa);
                if !name.is_empty() && name != "none" {
                    parts.push(name.to_string());
                }
            }
        }
        for &isa in disabled_isas {
            let no_name = ClangRiscv64NOISANames::name(isa);
            if !no_name.is_empty() {
                parts.push(no_name);
            }
        }
        format!("-m'arch=rv64{}'", parts.join("_"))
    }

    /// Evaluates optimal vector length (fixed normal VL128 for Riscv64)
    pub fn vector_length(&self) -> crate::vector_length::CpuArchitectureVectorLength {
        crate::vector_length::CpuArchitectureVectorLength::VL128
    }

    /// Evaluates vector length (fixed normal VL128 for Riscv64)
    pub fn resolve_vector_length(&self, _requested: Option<crate::vector_length::CpuArchitectureVectorLength>) -> crate::vector_length::CpuArchitectureVectorLength {
        crate::vector_length::CpuArchitectureVectorLength::VL128
    }

    /// Generates '+' delimited enabled extensions string
    pub fn to_extensions_string(&self) -> String {
        let mut extensions = Vec::new();
        for &isa in Riscv64ISA::all() {
            if self.has_feature(isa) {
                let name = isa.as_str();
                if !name.is_empty() && name != "none" {
                    extensions.push(name);
                }
            }
        }
        extensions.join("+")
    }

    /// Creates a key-value dictionary of boolean features
    pub fn to_map(&self) -> BTreeMap<String, bool> {
        let mut map = BTreeMap::new();
        for &isa in Riscv64ISA::all() {
            let name = isa.as_str();
            if !name.is_empty() && name != "none" {
                map.insert(name.replace(['-', '.'], "_"), self.has_feature(isa));
            }
        }
        map
    }
}
