use std::time::Instant;

use clap::Parser;

use crate::{cli::Cli, files::get_all_files_to_lint, linter::lint_file};

extern crate swc_common;

mod cli;
mod files;
mod linter;

fn main() {
    let cli = Cli::parse();

    let start_time = Instant::now();

    let input = get_all_files_to_lint(cli.files);

    for file in input {
        lint_file(file.as_path());
    }

    let elapsed_time = start_time.elapsed().as_secs_f32();
    println!("\n✨ Done in {:.2}s.", elapsed_time);
}
