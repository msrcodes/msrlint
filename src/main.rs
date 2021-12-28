use std::time::Instant;

use clap::Parser;

use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();

    let start_time = Instant::now();

    println!("Using input {:?}", cli.files);

    let elapsed_time = start_time.elapsed().as_secs_f32();
    println!("\n✨ Done in {:.2}s.", elapsed_time);
}
