use std::{fmt::Debug, sync::Arc};

use serde::{Deserialize, Serialize};
use swc_common::SourceMap;
use swc_ecma_ast::FnDecl;
use swc_ecma_lints::rule::Rule;
use swc_ecma_visit::{noop_visit_type, Visit};

use crate::linter::{config::RuleConfig, rule::visitor_rule};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndentConfig {
    #[serde(default)]
    num: u32,
    indent_type: IndentType,
}

impl Default for IndentConfig {
    fn default() -> Self {
        Self {
            num: 4,
            indent_type: IndentType::Space,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, std::cmp::PartialEq)]
enum IndentType {
    Space,
    Tab,
}

impl Default for IndentType {
    fn default() -> Self {
        Self::Space
    }
}

pub fn indent(source_map: &Arc<SourceMap>, config: &RuleConfig<IndentConfig>) -> Box<dyn Rule> {
    visitor_rule(Indent::new(source_map.clone(), config))
}

#[derive(Default)]
struct Indent {
    source_map: Arc<SourceMap>,
    num: u32,
    indent_type: IndentType,
}

impl Indent {
    fn new(source_map: Arc<SourceMap>, config: &RuleConfig<IndentConfig>) -> Self {
        let indent_config = config.get_rule_config();
        Self {
            source_map,
            num: indent_config.num,
            indent_type: indent_config.indent_type,
        }
    }
}

impl Debug for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Indent").finish()
    }
}

impl Visit for Indent {
    noop_visit_type!();

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        println!("{:#?}", fn_decl.function);

        // TODO
    }
}
