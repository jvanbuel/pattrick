use std::{env, fmt::Error};

use clap::CommandFactory;
use clap_complete_fig::Fig;
use pattrick_clap::Cli;

fn main() -> Result<(), Error> {
    let outdir = if let Ok(outdir) = env::var("OUT_DIR") {
        outdir
    } else {
        String::from("fig_spec")
    };
    let path = clap_complete::generate_to(Fig, &mut Cli::command(), "pattrick", outdir)
        .expect("Unable to generate Fig spec");

    println!("cargo:warning=completion file is generated: {path:?}");

    Ok(())
}
