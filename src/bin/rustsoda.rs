#![feature(backtrace)]
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_interface;

use rustc_driver::Compilation;
use rustc_interface::{interface::Compiler, Queries};

#[macro_use]
extern crate log;

use std::env;

use rustsoda::log::{setup_logging, Verbosity};
use rustsoda::{compile_time_sysroot, main_entry, progress_info, RustSodaConfig, RUSTSODA_DEFAULT_ARGS};

struct RustSodaCompilerCalls{
    config: RustSodaConfig,
}

impl RustSodaCompilerCalls{
    fn new(config: RustSodaConfig) -> Self {
        RustSodaCompilerCalls{ config }
    }
}

impl rustc_driver::Callbacks for RustSodaCompilerCalls{
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        compiler.session().abort_if_errors();

        setup_logging(self.config.verbosity).expect("RustSoda failed to initialize");

        progress_info!("RustSoda started");

        
        queries.global_ctxt().unwrap().peek_mut().enter(|tcx| {
            main_entry(tcx.clone(), self.config);
        });


        progress_info!("RustSoda finished");

        compiler.session().abort_if_errors();
        Compilation::Stop
    }
}

fn parse_config() -> (RustSodaConfig, Vec<String>) {
    // collect arguments
    let mut config = RustSodaConfig::default();

    let mut rustc_args = vec![];
    for arg in std::env::args() {
        match arg.as_str() {
            "-v" => config.verbosity = Verbosity::Verbose,
            "-vv" => config.verbosity = Verbosity::Trace,
            _ => {
                rustc_args.push(arg);
            }
        }
    }

    (config, rustc_args)
}

/// Execute a compiler with the given CLI arguments and callbacks.
fn run_compiler(
    mut args: Vec<String>,
    callbacks: &mut (dyn rustc_driver::Callbacks + Send),
) -> i32 {
    // Make sure we use the right default sysroot. The default sysroot is wrong,
    // because `get_or_default_sysroot` in `librustc_session` bases that on `current_exe`.
    //
    // Make sure we always call `compile_time_sysroot` as that also does some sanity-checks
    // of the environment we were built in.
    // FIXME: Ideally we'd turn a bad build env into a compile-time error via CTFE or so.
    if let Some(sysroot) = compile_time_sysroot() {
        let sysroot_flag = "--sysroot";
        if !args.iter().any(|e| e == sysroot_flag) {
            // We need to overwrite the default that librustc_session would compute.
            args.push(sysroot_flag.to_owned());
            args.push(sysroot);
        }
    }

    // Some options have different defaults in RustSoda than in plain rustc; apply those by making
    // them the first arguments after the binary name (but later arguments can overwrite them).
    args.splice(
        1..1,
        rustsoda::RUSTSODA_DEFAULT_ARGS.iter().map(ToString::to_string),
    );

    // Invoke compiler, and handle return code.
    let exit_code = rustc_driver::catch_with_exit_code(move || {
        //rustc_driver::RunCompiler::new(&args, callbacks).run()
        rustc_driver::run_compiler(&args, callbacks, None, None)
    });

    exit_code
}

fn main() {
    rustc_driver::install_ice_hook();

    let exit_code = {
        // initialize the report logger
        // `logger_handle` must be nested because it flushes the logs when it goes out of the scope
        let (config, mut rustc_args) = parse_config();
        //let _logger_handle = init_report_logger(default_report_logger());

        // init rustc logger
        if env::var_os("RUSTC_LOG").is_some() {
            rustc_driver::init_rustc_env_logger();
        }

        if let Some(sysroot) = compile_time_sysroot() {
            let sysroot_flag = "--sysroot";
            if !rustc_args.iter().any(|e| e == sysroot_flag) {
                // We need to overwrite the default that librustc would compute.
                rustc_args.push(sysroot_flag.to_owned());
                rustc_args.push(sysroot);
            }
        }

        // Finally, add the default flags all the way in the beginning, but after the binary name.
        rustc_args.splice(1..1, RUSTSODA_DEFAULT_ARGS.iter().map(ToString::to_string));

        debug!("rustc arguments: {:?}", &rustc_args);
        run_compiler(rustc_args, &mut RustSodaCompilerCalls::new(config))
    };

    std::process::exit(exit_code)
}
