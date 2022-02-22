use std::path::Path;

pub mod config;
mod rule;
mod rules;

use swc_common::{
    self,
    errors::{ColorConfig, Handler},
    input::SourceFileInput,
    sync::Lrc,
    SourceMap,
};

use swc_ecma_ast::{EsVersion, Program};
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax};

use rules::get_all_rules;
use swc_ecma_utils::HANDLER;

use self::{config::LintConfig, rules::LintContext};

/// Lint file, returning the number of errors found
pub fn lint_file(path: &Path, lint_config: &LintConfig) -> usize {
    let cm: Lrc<SourceMap> = Default::default();
    let source_file = cm.load_file(path).unwrap();
    let es_version: EsVersion = Default::default();

    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Es(Default::default()),
        // EsVersion defaults to es5
        es_version,
        SourceFileInput::from(&*source_file),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();
    let program = Program::Module(module);

    let context = LintContext {
        program: &program,
        lint_config: &lint_config,
        es_version,
        source_map: cm,
    };

    let rules = get_all_rules(context);

    let mut num_errors = 0;

    HANDLER.set(&handler, || {
        // apply all rules
        if let Program::Module(m) = program {
            for mut rule in rules {
                rule.lint_module(&m);
            }
        }

        num_errors += handler.err_count()
    });

    num_errors
}
