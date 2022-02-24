use serde::{Deserialize, Serialize};
use swc_common::{input::SourceFileInput, sync::Lrc, SourceMap};
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax};

use super::rules::lints::quotes::QuotesConfig;
use std::{fmt::Debug, path::PathBuf};

#[derive(Default, Clone)]
pub struct LintConfig {
    pub quotes: RuleConfig<QuotesConfig>,
}

impl From<PathBuf> for LintConfig {
    fn from(buf: PathBuf) -> Self {
        // If buf is default, the user did not specify a config file to load
        if buf == PathBuf::default() {
            return LintConfig::default();
        }

        // Get file extension
        let valid_extensions = ["json", "js", "cjs", "yml", "yaml"];
        let ext = buf.extension().unwrap().to_str().unwrap();

        // Check for invalid file extensions
        if !valid_extensions.contains(&ext) {
            panic!(
                "Extension {:?} is not a valid file extension. Please use one of {:?} instead.",
                ext, valid_extensions
            )
        }

        // Parse .eslintrc.js files
        if ext == "js" {
            let cm: Lrc<SourceMap> = Default::default();
            let source_file = cm.load_file(buf.as_path()).unwrap();

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

            // TODO: fix this hacky mess
            let src = &cm.lookup_byte_offset(module.span.lo).sf.src;
            let split = src.as_str().split("module.exports");

            if split.count() <= 1 {
                panic!("No module.exports detected")
            } else {
            }
        } else {
            // TODO: parse other file types
            todo!()
        }

        LintConfig::default()
    }
}

// Adapted from
// https://github.com/swc-project/swc/blob/e9c1b229262c07d114e4b75bbc9f104b45fbedf3/crates/swc_ecma_lints/src/config.rs#L53-L67
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleConfig<T: Debug + Clone + Serialize + Default>(#[serde(default)] T);

impl<T: Debug + Clone + Serialize + Default> RuleConfig<T> {
    pub(crate) fn get_rule_config(&self) -> &T {
        &self.0
    }
}
// end
