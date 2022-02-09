use std::path::Path;

mod rules;

use swc_common::{
    self,
    // errors::{ColorConfig, Handler},
    input::SourceFileInput,
    sync::Lrc,
    SourceFile,
    SourceMap,
};

use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax};

use rules::get_all_rules;

pub struct ParsedModule {
    pub module: Module,
    pub source_file: Lrc<SourceFile>,
}

fn parse_module_file(path: &Path) -> ParsedModule {
    let cm: Lrc<SourceMap> = Default::default();
    let source_file = cm.load_file(path).unwrap();
    // let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // let source_file = cm.load_file(path).expect("failed to load test.js");

    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Es(Default::default()),
        // EsVersion defaults to es5
        Default::default(),
        SourceFileInput::from(&*source_file),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap();

    ParsedModule {
        module,
        source_file,
    }

    // for e in parser.take_errors() {
    //     e.into_diagnostic(&handler).emit();
    // }

    // let module = parser
    //     .parse_module()
    //     .map_err(|e| {
    //         // Unrecoverable fatal error occurred
    //         e.into_diagnostic(&handler).emit()
    //     })
    //     .expect("failed to parser module");

    // ParsedModule {
    //     module,
    //     source_file,
    // }
}

pub fn lint_file(path: &Path) {
    // parse file into AST
    let parsed_module = parse_module_file(path);

    // apply all rules
    for mut rule in get_all_rules() {
        rule.lint_module(&parsed_module.module);
    }
}
