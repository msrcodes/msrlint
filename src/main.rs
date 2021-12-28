use std::time::Instant;

use clap::Parser;

use crate::{cli::Cli, files::get_all_files_to_lint};

mod cli;
mod files;

fn main() {
    let cli = Cli::parse();

    let start_time = Instant::now();

    let input = get_all_files_to_lint(cli.files);

    println!("Using input {:?}", input);

    let elapsed_time = start_time.elapsed().as_secs_f32();
    println!("\n✨ Done in {:.2}s.", elapsed_time);
}
