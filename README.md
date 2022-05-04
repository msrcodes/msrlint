# ⏩ MSRLint
My Speedy Rust Linter. A (WIP) Rust-based replacement to ESLint.

Currently supported rules:
- Quotes
- Semi
- Eqeqeq
- NoEval

Currently supported config files:
- .eslintrc.json
  - Only for disabling/enabling of rules

## 📦 Installation

```
TODO, once this repository is public
```

## 🛠 Usage

```
USAGE:
    msrlint [OPTIONS] <FILES>...

ARGS:
    <FILES>...    File(s) to lint

OPTIONS:
    -c, --config <CONFIG>
            [NYI] Use this configuration, overriding .eslintrc.* config options if present

        --cache
            [NYI] Only check changed files

        --cache-location <CACHE_LOCATION>
            [NYI] Path to the cache file or directory

        --cache-strategy <CACHE_STRATEGY>
            [NYI] Strategy to use for detecting changed files in the cache [possible values:
            metadata, content]

        --color
            [NYI] Force enabling of color

        --debug
            [NYI] Output debugging information

        --env <ENV>
            [NYI] Specify environments

        --env-info
            [NYI] Output execution environment information

        --exit-on-fatal-error
            [NYI] Exit with exit code 2 in case of fatal error

        --ext <EXT>
            [NYI] Specify JavaScript file extensions

    -f, --format <FORMAT>
            [NYI] Use a specific output format

        --fix
            [NYI] Automatically fix problems

        --fix-dry-run
            [NYI] Automatically fix problems without saving the changes to the file system

        --fix-type <FIX_TYPE>
            [NYI] Specify the types of fixes to apply [possible values: directive, problem,
            suggestion, layout]

        --global <GLOBAL>
            [NYI] Define global variables

    -h, --help
            Print help information

        --ignore-path <IGNORE_PATH>
            [NYI] Specify path of ignore file

        --ignore-pattern <IGNORE_PATTERN>
            [NYI] Pattern of files to ignore (in addition to those in .eslintignore)

        --init
            [NYI] Run config initialization wizard

        --max-warnings <MAX_WARNINGS>
            [NYI] Number of warnings to trigger nonzero exit code

        --no-color
            [NYI] Force disabling of color

        --no-error-on-unmatched-pattern
            [NYI] Prevent errors when pattern is unmatched

        --no-eslintrc
            [NYI] Disable use of configuration from .eslintrc.*

        --no-ignore
            [NYI] Disable use of ignore files and patterns

        --no-inline-config
            [NYI] Prevent comments from changing config or rules

    -o, --output-file <OUTPUT_FILE>
            [NYI] Specify file to write report to

        --parser <PARSER>
            [NYI] Specify the parser to be used

        --parser-options <PARSER_OPTIONS>
            [NYI] Specify parser options

        --plugin <PLUGIN>
            [NYI] Specify plugins

        --print-config <PRINT_CONFIG>
            [NYI] Print the configuration for the given file

        --quiet
            [NYI] Report errors only

        --report-unused-disable-directives
            [NYI] Adds reported errors for unused eslint-disable directives

        --resolve-plugins-relative-to <RESOLVE_PLUGINS_RELATIVE_TO>
            [NYI] A folder where plugins should be resolved from, CWD by default

        --rule <RULE>
            [NYI] Specify rules

        --stdin
            [NYI] Lint code provided on <STDIN>

        --stdin-filename <STDIN_FILENAME>
            [NYI] Specify filename to process STDIN as

    -v, --version
            [NYI] Output the version number
```
