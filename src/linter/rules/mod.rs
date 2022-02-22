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
        lint_config: _,
        es_version: _,
        source_map,
    } = context;

    let rules = vec![quotes::quotes(&source_map)];

    rules
}