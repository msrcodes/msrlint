use swc_common::SourceMap;
use swc_ecma_ast::{EsVersion, Module, Program, Script};
use swc_ecma_lints::rule::Rule;
use swc_ecma_visit::{Visit, VisitWith};

mod quotes;

use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
struct VisitorRule<V>(V)
where
    V: Send + Sync + Visit;

impl<V> Rule for VisitorRule<V>
where
    V: Send + Sync + Visit + Debug,
{
    fn lint_module(&mut self, program: &Module) {
        program.visit_with(&mut self.0);
    }

    fn lint_script(&mut self, program: &Script) {
        program.visit_with(&mut self.0);
    }
}

pub(crate) fn visitor_rule<V>(v: V) -> Box<dyn Rule>
where
    V: 'static + Send + Sync + Visit + Default + Debug,
{
    Box::new(VisitorRule(v))
}

pub struct LintContext<'a> {
    pub program: &'a Program,
    pub es_version: EsVersion,
    pub source_map: Arc<SourceMap>,
}

pub fn get_all_rules(context: LintContext) -> Vec<Box<dyn Rule>> {
    let LintContext {
        program: _,
        es_version: _,
        source_map,
    } = context;

    let rules = vec![quotes::quotes(&source_map)];

    rules
}
