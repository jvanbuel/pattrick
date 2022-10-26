use clap::CommandFactory;
use clap_complete;
use clap_complete_fig::Fig;
use pattrick_clap::Cli;

fn main() {
    clap_complete::generate_to(Fig, &mut Cli::command(), "pattrick", "fig_spec")
        .expect("Unable to generate Fig spec");
}
