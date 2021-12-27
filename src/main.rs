use clap::{ArgEnum, Parser};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum FixType {
    Directive,
    Problem,
    Suggestion,
    Layout,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum CacheStrategy {
    Metadata,
    Content,
}

/// Find and fix problems in your JavaScript code... but faster.
#[derive(Parser)]
#[clap(about, version, author)]
struct Cli {
    // Interpreted from https://eslint.org/docs/user-guide/command-line-interface 2021-12-27 @ 1740 GMT

    // ~~~ Basic configuration
    /// [NYI] Disable use of configuration from .eslintrc.*
    #[clap(long)]
    no_eslintrc: bool,

    /// [NYI] Use this configuration, overriding .eslintrc.* config options if present
    #[clap(short = 'c', long)]
    config: Option<String>,

    /// [NYI] Specify environments
    #[clap(long)]
    env: Option<String>,

    /// [NYI] Specify JavaScript file extensions
    #[clap(long)]
    ext: Option<String>,

    /// [NYI] Define global variables
    #[clap(long)]
    global: Option<String>,

    /// [NYI] Specify the parser to be used
    #[clap(long)]
    parser: Option<String>,

    /// [NYI] Specify parser options
    #[clap(long)]
    parser_options: Option<String>,

    /// [NYI] A folder where plugins should be resolved from, CWD by default
    #[clap(long)]
    resolve_plugins_relative_to: Option<String>,

    // ~~~ Specifying rules and plugins
    /// [NYI] Specify plugins
    #[clap(long)]
    plugin: Option<String>,

    /// [NYI] Specify rules
    #[clap(long)]
    rule: Option<String>,

    // ~~~ Fixing problems
    /// [NYI] Automatically fix problems
    #[clap(long)]
    fix: bool,

    /// [NYI] Automatically fix problems without saving the changes to the file system
    #[clap(long)]
    fix_dry_run: bool,

    /// [NYI] Specify the types of fixes to apply
    #[clap(arg_enum, long)]
    fix_type: Option<FixType>,

    // ~~~ Ignoring files
    /// [NYI] Specify path of ignore file
    #[clap(long)]
    ignore_path: Option<String>,

    /// [NYI] Disable use of ignore files and patterns
    #[clap(long)]
    no_ignore: bool,

    /// [NYI] Pattern of files to ignore (in addition to those in .eslintignore)
    #[clap(long)]
    ignore_pattern: Option<String>,

    // ~~~ Using stdin
    /// [NYI] Lint code provided on <STDIN>
    #[clap(long)]
    stdin: bool,

    /// [NYI] Specify filename to process STDIN as
    #[clap(long)]
    stdin_filename: Option<String>,

    // ~~~ Handling warnings
    /// [NYI] Report errors only
    #[clap(long)]
    quiet: bool,

    /// [NYI] Number of warnings to trigger nonzero exit code
    #[clap(long)]
    max_warnings: Option<u32>,

    // ~~~ Output
    /// [NYI] Specify file to write report to
    #[clap(long, short = 'o')]
    output_file: Option<String>,

    /// [NYI] Use a specific output format
    #[clap(long, short = 'f')]
    format: Option<String>,

    /// [NYI] Force enabling of color
    #[clap(long)]
    color: bool,

    /// [NYI] Force disabling of color
    #[clap(long)]
    no_color: bool,

    // ~~~ Inline configuration comments
    /// [NYI] Prevent comments from changing config or rules
    #[clap(long)]
    no_inline_config: bool,

    /// [NYI] Adds reported errors for unused eslint-disable directives
    #[clap(long)]
    report_unused_disable_directives: bool,

    // ~~~ Caching
    /// [NYI] Only check changed files
    #[clap(long)]
    cache: bool,

    /// [NYI] Path to the cache file or directory
    #[clap(long)]
    cache_location: Option<String>,

    /// [NYI] Strategy to use for detecting changed files in the cache
    #[clap(arg_enum, long)]
    cache_strategy: Option<CacheStrategy>,

    // ~~~ Miscellaneous
    /// [NYI] Run config initialization wizard
    #[clap(long)]
    init: bool,

    /// [NYI] Output execution environment information
    #[clap(long)]
    env_info: bool,

    /// [NYI] Prevent errors when pattern is unmatched
    #[clap(long)]
    no_error_on_unmatched_pattern: bool,

    /// [NYI] Exit with exit code 2 in case of fatal error
    #[clap(long)]
    exit_on_fatal_error: bool,

    /// [NYI] Output debugging information
    #[clap(long)]
    debug: bool,

    /// [NYI] Output the version number
    #[clap(short = 'v', long)]
    version: bool,

    /// [NYI] Print the configuration for the given file
    #[clap(long)]
    print_config: Option<String>,
}

fn main() {
    let args = Cli::parse();

    println!("Hello {}!", args.exit_on_fatal_error)
}
