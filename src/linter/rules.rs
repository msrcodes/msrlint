use swc_ecma_lints::rule::Rule;
use swc_ecmascript::{
    ast::{Module, Script},
    visit::{Visit, VisitWith},
};

mod quotes;

use std::fmt::Debug;

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

pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    vec![quotes::quotes()]
}
