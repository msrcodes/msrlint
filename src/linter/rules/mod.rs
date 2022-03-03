#[path = ""]
pub(crate) mod lints {
    pub mod quotes;
}

use std::{collections::HashMap, sync::Arc};

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

fn get_all_rules_raw<'a>(context: &'a LintContext) -> HashMap<&'a str, Box<dyn Rule>> {
    let LintContext {
        program: _,
        lint_config,
        es_version: _,
        source_map,
    } = context;

    let mut rule_map = HashMap::new();
    rule_map.insert("quotes", quotes::quotes(source_map, &lint_config.quotes));

    rule_map
}

pub fn get_all_rules(context: LintContext) -> Vec<Box<dyn Rule>> {
    let mut rules = get_all_rules_raw(&context);

    let LintContext {
        program: _,
        lint_config,
        es_version: _,
        source_map: _,
    } = context;

    if lint_config.rules.contains(&String::from("eslint:all")) {
        // If assuming eslint:all, start with all rules and disable the rules specified
        for rule in &lint_config.disabled_rules {
            rules.remove(rule.as_str());
        }

        rules.into_values().collect()
    } else {
        todo!()
    }
}
