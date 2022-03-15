use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use swc_common::Span;
use swc_ecma_ast::Ident;
use swc_ecma_lints::rule::Rule;
use swc_ecma_utils::HANDLER;
use swc_ecma_visit::{noop_visit_type, Visit, VisitWith};

use crate::linter::{config::RuleConfig, rule::visitor_rule};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct NoEvalConfig {}

pub fn noeval(config: &RuleConfig<NoEvalConfig>) -> Box<dyn Rule> {
    visitor_rule(NoEval::new(config))
}

#[derive(Default)]
struct NoEval {}

impl NoEval {
    fn new(config: &RuleConfig<NoEvalConfig>) -> Self {
        let _noeval_config = config.get_rule_config();
        Self {}
    }

    fn emit_error(&self, span: Span) {
        let msg = "The use of the eval function is forbidden.";
        HANDLER.with(|handler| {
            handler.struct_span_err(span, msg).emit();
        });
    }
}

impl Debug for NoEval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoEval").finish()
    }
}

impl Visit for NoEval {
    noop_visit_type!();

    fn visit_ident(&mut self, ident: &Ident) {
        if ident.sym.to_string().as_str() == "eval" {
            self.emit_error(ident.span);
        }

        ident.visit_children_with(self);
    }
}
