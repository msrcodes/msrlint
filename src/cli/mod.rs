use std::path::PathBuf;

pub use clap::{ArgEnum, Parser};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum FixType {
    Directive,
    Problem,
    Suggestion,
    Layout,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum CacheStrategy {
    Metadata,
    Content,
}

/// Find and fix problems in your JavaScript code... but faster.
#[derive(Parser)]
#[clap(about, version, author)]
pub struct Cli {
    // Interpreted from https://eslint.org/docs/user-guide/command-line-interface 2021-12-27 @ 1740 GMT

    // ~~~ Basic configuration
    /// [NYI] Disable use of configuration from .eslintrc.*
    #[clap(long)]
    pub no_eslintrc: bool,

    /// [NYI] Use this configuration, overriding .eslintrc.* config options if present
    #[clap(short = 'c', long)]
    pub config: Option<String>,

    /// [NYI] Specify environments
    #[clap(long)]
    pub env: Option<String>,

    /// [NYI] Specify JavaScript file extensions
    #[clap(long)]
    pub ext: Option<String>,

    /// [NYI] Define global variables
    #[clap(long)]
    pub global: Option<String>,

    /// [NYI] Specify the parser to be used
    #[clap(long)]
    pub parser: Option<String>,

    /// [NYI] Specify parser options
    #[clap(long)]
    pub parser_options: Option<String>,

    /// [NYI] A folder where plugins should be resolved from, CWD by default
    #[clap(long)]
    pub resolve_plugins_relative_to: Option<String>,

    // ~~~ Specifying rules and plugins
    /// [NYI] Specify plugins
    #[clap(long)]
    pub plugin: Option<String>,

    /// [NYI] Specify rules
    #[clap(long)]
    pub rule: Option<String>,

    // ~~~ Fixing problems
    /// [NYI] Automatically fix problems
    #[clap(long)]
    pub fix: bool,

    /// [NYI] Automatically fix problems without saving the changes to the file system
    #[clap(long)]
    pub fix_dry_run: bool,

    /// [NYI] Specify the types of fixes to apply
    #[clap(arg_enum, long)]
    pub fix_type: Option<FixType>,

    // ~~~ Ignoring files
    /// [NYI] Specify path of ignore file
    #[clap(long)]
    pub ignore_path: Option<String>,

    /// [NYI] Disable use of ignore files and patterns
    #[clap(long)]
    pub no_ignore: bool,

    /// [NYI] Pattern of files to ignore (in addition to those in .eslintignore)
    #[clap(long)]
    pub ignore_pattern: Option<String>,

    // ~~~ Using stdin
    /// [NYI] Lint code provided on <STDIN>
    #[clap(long)]
    pub stdin: bool,

    /// [NYI] Specify filename to process STDIN as
    #[clap(long)]
    pub stdin_filename: Option<String>,

    // ~~~ Handling warnings
    /// [NYI] Report errors only
    #[clap(long)]
    pub quiet: bool,

    /// [NYI] Number of warnings to trigger nonzero exit code
    #[clap(long)]
    pub max_warnings: Option<u32>,

    // ~~~ Output
    /// [NYI] Specify file to write report to
    #[clap(long, short = 'o')]
    pub output_file: Option<String>,

    /// [NYI] Use a specific output format
    #[clap(long, short = 'f')]
    pub format: Option<String>,

    /// [NYI] Force enabling of color
    #[clap(long)]
    pub color: bool,

    /// [NYI] Force disabling of color
    #[clap(long)]
    pub no_color: bool,

    // ~~~ Inline configuration comments
    /// [NYI] Prevent comments from changing config or rules
    #[clap(long)]
    pub no_inline_config: bool,

    /// [NYI] Adds reported errors for unused eslint-disable directives
    #[clap(long)]
    pub report_unused_disable_directives: bool,

    // ~~~ Caching
    /// [NYI] Only check changed files
    #[clap(long)]
    pub cache: bool,

    /// [NYI] Path to the cache file or directory
    #[clap(long)]
    pub cache_location: Option<String>,

    /// [NYI] Strategy to use for detecting changed files in the cache
    #[clap(arg_enum, long)]
    pub cache_strategy: Option<CacheStrategy>,

    // ~~~ Miscellaneous
    /// [NYI] Run config initialization wizard
    #[clap(long)]
    pub init: bool,

    /// [NYI] Output execution environment information
    #[clap(long)]
    pub env_info: bool,

    /// [NYI] Prevent errors when pattern is unmatched
    #[clap(long)]
    pub no_error_on_unmatched_pattern: bool,

    /// [NYI] Exit with exit code 2 in case of fatal error
    #[clap(long)]
    pub exit_on_fatal_error: bool,

    /// [NYI] Output debugging information
    #[clap(long)]
    pub debug: bool,

    /// [NYI] Output the version number
    #[clap(short = 'v', long)]
    pub version: bool,

    /// [NYI] Print the configuration for the given file
    #[clap(long)]
    pub print_config: Option<String>,

    /// File(s) to lint
    #[clap(required = true, parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
