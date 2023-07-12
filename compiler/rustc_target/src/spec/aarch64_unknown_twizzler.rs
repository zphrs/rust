use crate::spec::{Target, TargetOptions};

const LINKER_SCRIPT: &str = include_str!("./aarch64_unknown_twizzler_linker_script.ld");

pub fn target() -> Target {
    // don't use probe-stack=inline-asm until rust#83139 and rust#84667 are resolved
    // - rust#83139 has been resolved since may 2021
    //   - bug exists in llvm, but not rust
    // - #84667 also resolved in may 2021
    //   - should be in stable llvm 16 mar 2022
    // base.stack_probes = StackProbeType::Call;

    Target {
        llvm_target: "aarch64-unknown-twizzler".into(),
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".into(),
        arch: "aarch64".into(),
        options: TargetOptions {
            max_atomic_width: Some(128),
            // this option requires linkers where `linker_is_gnu` is true.
            link_script: Some(LINKER_SCRIPT.into()),
            ..super::twizzler_base::opts()
        },
    }
}
