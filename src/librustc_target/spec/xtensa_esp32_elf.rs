// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use spec::{LinkerFlavor, PanicStrategy, Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: "xtensa-unknown-elf".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "e-m:e-p:32:32-i1:8:32-i8:8:32-i16:16:32-i64:64-f64:64-a:0:32-n32".to_string(),
        arch: "xtensa".to_string(),
        target_os: "unknown".to_string(),
        target_env: "".to_string(),
        target_vendor: "".to_string(),
        linker_flavor: LinkerFlavor::Gcc,

        options: TargetOptions {
            // CPU
            cpu: "esp32".to_string(),

            executables: false,

            // The LLVM backend currently can't generate object files. To
            // workaround this LLVM generates assembly files which then we feed
            // to gcc to get object files. For this reason we have a hard
            // dependency on this specific gcc.
            asm_args: vec![],
            linker: Some("xtensa-esp32-elf-gcc".to_string()),
            no_integrated_as: true,

            // There are no atomic instructions available in the MSP430
            // instruction set
            max_atomic_width: Some(0),

            // Because these devices have very little resources having an
            // unwinder is too onerous so we default to "abort" because the
            // "unwind" strategy is very rare.
            panic_strategy: PanicStrategy::Abort,

            // Similarly, one almost always never wants to use relocatable
            // code because of the extra costs it involves.
            relocation_model: "static".to_string(),

            // Right now we invoke an external assembler and this isn't
            // compatible with multiple codegen units, and plus we probably
            // don't want to invoke that many gcc instances.
            default_codegen_units: Some(1),

            // Since MSP430 doesn't meaningfully support faulting on illegal
            // instructions, LLVM generates a call to abort() function instead
            // of a trap instruction. Such calls are 4 bytes long, and that is
            // too much overhead for such small target.
            trap_unreachable: false,

            // See the thumb_base.rs file for an explanation of this value
            emit_debug_gdb_scripts: false,
            
            .. Default::default( )
        }
    })
}
