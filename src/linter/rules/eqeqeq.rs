use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use swc_common::Span;
use swc_ecma_ast::{
    BinExpr,
    BinaryOp::{EqEq, EqEqEq, NotEq, NotEqEq},
};
use swc_ecma_lints::rule::Rule;
use swc_ecma_utils::HANDLER;
use swc_ecma_visit::{noop_visit_type, Visit, VisitWith};

use crate::linter::{config::RuleConfig, rule::visitor_rule};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct EqeqeqConfig {
    mode: Mode,
}

pub fn eqeqeq(config: &RuleConfig<EqeqeqConfig>) -> Box<dyn Rule> {
    visitor_rule(Eqeqeq::new(config))
}

#[derive(Default)]
struct Eqeqeq {
    mode: Mode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, std::cmp::PartialEq)]
enum Mode {
    Always,
    Never,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Always
    }
}

impl Eqeqeq {
    fn new(config: &RuleConfig<EqeqeqConfig>) -> Self {
        let eqeqeq_config = config.get_rule_config();
        Self {
            mode: eqeqeq_config.mode,
        }
    }

    fn emit_error(&self, span: Span, found: &str, expected: &str) {
        let msg = format!("Expected \"{}\" and instead saw \"{}\".", expected, found);
        HANDLER.with(|handler| {
            handler.struct_span_err(span, &msg).emit();
        });
    }
}

impl Debug for Eqeqeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Eqeqeq").finish()
    }
}

impl Visit for Eqeqeq {
    noop_visit_type!();

    fn visit_bin_expr(&mut self, bin_expr: &BinExpr) {
        let op = bin_expr.op;
        let span = bin_expr.span;
        let mode = self.mode;

        match op {
            // ==
            EqEq => {
                if mode == Mode::Always {
                    self.emit_error(span, "==", "===");
                }
            }
            // \!=
            NotEq => {
                if mode == Mode::Always {
                    self.emit_error(span, "!=", "===");
                }
            }
            // ===
            EqEqEq => {
                if mode == Mode::Never {
                    self.emit_error(span, "===", "==");
                }
            }
            // \!==
            NotEqEq => {
                if mode == Mode::Never {
                    self.emit_error(span, "!==", "!=");
                }
            }
            _ => {}
        }

        bin_expr.visit_children_with(self);
    }
}
