mod cli;

use std::str::FromStr;

use anoma_apps::logging;
use color_eyre::eyre::Result;
use tracing_subscriber::filter::Directive;
use dhat::{Dhat, DhatAlloc};

#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() -> Result<()> {
    // https://docs.rs/dhat/0.2.2/dhat/#usage-heap-profiling
    let _dhat = Dhat::start_heap_profiling();

    // init error reporting
    color_eyre::install()?;

    // init logging
    let default_directive = Directive::from_str("anoma=info")?;
    logging::init_from_env_or(default_directive)?;

    // run the CLI
    cli::main()
}
