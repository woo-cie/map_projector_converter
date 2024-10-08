use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Pointcloud {
        input_yaml: String,
        output_yaml: String,
        input_pcd: String,
        output_pcd: String,
    },
    Lanelet2 {
        input_yaml: String,
        output_yaml: String,
        input_osm: String,
        output_osm: String,
    },
    Stoppoints {
        input_yaml: String,
        output_yaml: String,
        input_csv: String,
        output_csv: String,
    },
}

#[derive(Debug, Parser)]
#[command(
    version,
    author = env!("CARGO_PKG_AUTHORS"),
    about,
    arg_required_else_help = true,
)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommands,
}

impl Cli {
    pub fn exec(&self) {
        match &self.subcommand {
            SubCommands::Pointcloud {
                input_yaml,
                output_yaml,
                input_pcd,
                output_pcd,
            } => {
                println!(
                    "map_projector_converter pointcloud {} {} {} {}",
                    input_yaml, output_yaml, input_pcd, output_pcd
                );
                unimplemented!();
            }
            SubCommands::Lanelet2 {
                input_yaml,
                output_yaml,
                input_osm,
                output_osm,
            } => {
                println!(
                    "map_projector_converter lanelet2 {} {} {} {}",
                    input_yaml, output_yaml, input_osm, output_osm
                );
                unimplemented!();
            }
            SubCommands::Stoppoints {
                input_yaml,
                output_yaml,
                input_csv,
                output_csv,
            } => {
                println!(
                    "map_projector_converter stoppoints {} {} {} {}",
                    input_yaml, output_yaml, input_csv, output_csv
                );
                unimplemented!();
            }
        }
    }
}
