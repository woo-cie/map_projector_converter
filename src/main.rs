use clap::Parser;
use map_projector_converter::cli::Cli;

fn main() {
    Cli::parse().exec();
}
