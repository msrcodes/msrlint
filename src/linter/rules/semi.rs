use serde::{Deserialize, Serialize};
use swc_common::{SourceMap, Span};
use swc_ecma_ast::{ExprStmt, ReturnStmt};
use swc_ecma_lints::rule::Rule;
use swc_ecma_utils::HANDLER;
use swc_ecma_visit::{noop_visit_type, Visit};

use std::{fmt::Debug, sync::Arc};

use crate::linter::{config::RuleConfig, rule::visitor_rule};

use std::str;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SemiConfig {
    /// A value of "true" means semis should be used instead of ASI.
    /// A value of "false" means semis should not be used, preferring ASI.
    prefer: bool,
}

pub fn semi(source_map: &Arc<SourceMap>, config: &RuleConfig<SemiConfig>) -> Box<dyn Rule> {
    visitor_rule(Semi::new(source_map.clone(), config))
}

#[derive(Default)]
struct Semi {
    source_map: Arc<SourceMap>,
    prefer: bool,
}

impl Semi {
    fn new(source_map: Arc<SourceMap>, config: &RuleConfig<SemiConfig>) -> Self {
        let semi_config = config.get_rule_config();
        Self {
            source_map,
            prefer: semi_config.prefer,
        }
    }

    fn emit_error(&self, span: Span, is_missing: bool) {
        let msg = if is_missing {
            "Missing semicolon."
        } else {
            "Extra semicolon."
        };

        HANDLER.with(|handler| {
            handler.struct_span_err(span, msg).emit();
        })
    }

    fn check_span(&self, span: Span) {
        let x = self.source_map.lookup_byte_offset(span.hi);
        // - 1 as pos.0 is the newline character
        let i = x.pos.0 - 1;
        let src = &x.sf.src;
        let final_char = src.as_bytes()[i as usize];
        let is_semi = final_char == b';';

        if self.prefer && !is_semi {
            self.emit_error(span, true);
        } else if !self.prefer && is_semi {
            self.emit_error(span, false);
        }
    }
}

impl Debug for Semi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Semi").finish()
    }
}

impl Visit for Semi {
    noop_visit_type!();

    fn visit_expr_stmt(&mut self, expr_stmt: &ExprStmt) {
        self.check_span(expr_stmt.span);
    }

    fn visit_return_stmt(&mut self, return_stmt: &ReturnStmt) {
        self.check_span(return_stmt.span);
    }
}
