#[path = ""]
pub(crate) mod lints {
    pub mod quotes;
}

use std::sync::Arc;

use lints::*;

use swc_common::SourceMap;
use swc_ecma_ast::{EsVersion, Program};
use swc_ecma_lints::rule::Rule;

use super::config::LintConfig;

pub struct LintContext<'a> {
    pub program: &'a Program,
    pub lint_config: &'a LintConfig,
    pub es_version: EsVersion,
    pub source_map: Arc<SourceMap>,
}

pub fn get_all_rules(context: LintContext) -> Vec<Box<dyn Rule>> {
    let LintContext {
        program: _,
        lint_config,
        es_version: _,
        source_map,
    } = context;

    // TODO: use rules from other extends, not just "all"
    if !lint_config.rules.contains(&String::from("eslint:all")) {
        return vec![];
    }

    // Initiate rules, providing necessary context properties
    let rules = vec![quotes::quotes(&source_map, &lint_config.quotes)];

    rules
}
