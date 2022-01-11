use std::path::Path;

mod rules;

use swc_common::{
    self,
    errors::{ColorConfig, Handler},
    input::SourceFileInput,
    sync::Lrc,
    SourceMap,
};

use swc_ecmascript::parser::{lexer::Lexer, Parser};
use swc_ecmascript::{ast::Module, parser::Syntax};

use rules::get_all_rules;

fn parse_module_file(path: &Path) -> Module {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let fm = cm.load_file(path).expect("failed to load test.js");

    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Es(Default::default()),
        // EsVersion defaults to es5
        Default::default(),
        SourceFileInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parser module")
}

pub fn lint_file(path: &Path) {
    // parse file into AST
    let module = parse_module_file(path);

    // apply all rules
    for mut rule in get_all_rules() {
        rule.lint_module(&module);
    }
}
