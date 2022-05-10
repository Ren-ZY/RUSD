#![feature(backtrace)]
#![feature(box_patterns)]
#![feature(rustc_private)]
#![feature(try_blocks)]
#![feature(never_type)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_typeck;
// #[macro_use]
// extern crate log as log_crate;

pub mod log;
pub mod call_graph;
pub mod johnson_find_cycles;

use crate::log::Verbosity;
use rustc_middle::ty::TyCtxt;
use crate::call_graph::call_graph_builder::*;
//use crate::johnson_find_cycles::elementary_cycles_search::*;

pub static RUSTSODA_DEFAULT_ARGS: &[&str] = &["-Zalways-encode-mir", "-Zmir-opt-level=0"];


#[derive(Debug, Clone, Copy)]
pub struct RustSodaConfig {
    pub verbosity: Verbosity, 
}

impl Default for RustSodaConfig {
    fn default() -> Self {
        RustSodaConfig {
            verbosity: Verbosity::Normal, 
        }
    }
}

/// Returns the "default sysroot" that Rtf will use if no `--sysroot` flag is set.
/// Should be a compile-time constant.
pub fn compile_time_sysroot() -> Option<String> {
    // option_env! is replaced to a constant at compile time
    if option_env!("RUSTC_STAGE").is_some() {
        // This is being built as part of rustc, and gets shipped with rustup.
        // We can rely on the sysroot computation in librustc.
        return None;
    }

    // For builds outside rustc, we need to ensure that we got a sysroot
    // that gets used as a default. The sysroot computation in librustc would
    // end up somewhere in the build dir.
    // Taken from PR <https://github.com/Manishearth/rust-clippy/pull/911>.
    let home = option_env!("RUSTUP_HOME").or(option_env!("MULTIRUST_HOME"));
    let toolchain = option_env!("RUSTUP_TOOLCHAIN").or(option_env!("MULTIRUST_TOOLCHAIN"));
    Some(match (home, toolchain) {
        (Some(home), Some(toolchain)) => format!("{}/toolchains/{}", home, toolchain),
        _ => option_env!("RUST_SYSROOT")
            .expect("To build rustsoda without rustup, set the `RUST_SYSROOT` env var at build time")
            .to_owned(),
    })
}

pub fn main_entry(tcx: TyCtxt, _config: RustSodaConfig) {
     let call_graph_info = call_graph_builder(tcx);
     println!("total function num: {}", call_graph_info.functions.borrow().len());
     // call_graph_info.print_call_graph();
     get_adj_list_and_find_cycles(tcx, &call_graph_info);     
}
