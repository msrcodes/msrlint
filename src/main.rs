use std::time::Instant;

use clap::Parser;

use crate::{cli::Cli, files::get_all_files_to_lint, linter::lint_file};

extern crate swc_common;
use colored::*;

mod cli;
mod files;
mod linter;

fn main() {
    let cli = Cli::parse();

    let start_time = Instant::now();

    let input = get_all_files_to_lint(cli.files);

    let mut num_errors = 0;

    for file in input {
        num_errors += lint_file(file.as_path());
    }

    if num_errors > 0 {
        let err_msg = format!("Found {} errors.", num_errors);
        println!("❌ {}", err_msg.bright_red().bold());
    }

    let elapsed_time = start_time.elapsed().as_secs_f32();
    println!("\n✨ Done in {:.2}s.", elapsed_time);
}
