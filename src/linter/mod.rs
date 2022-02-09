use std::path::Path;

mod rules;

use swc_common::{self, input::SourceFileInput, sync::Lrc, SourceMap};

use swc_ecma_ast::{EsVersion, Program};
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax};

use rules::get_all_rules;

use self::rules::LintContext;

pub fn lint_file(path: &Path) {
    let cm: Lrc<SourceMap> = Default::default();
    let source_file = cm.load_file(path).unwrap();
    let es_version: EsVersion = Default::default();

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
        es_version,
        source_map: cm,
    };

    let rules = get_all_rules(context);

    // apply all rules
    if let Program::Module(m) = program {
        for mut rule in rules {
            rule.lint_module(&m);
        }
    }
}
