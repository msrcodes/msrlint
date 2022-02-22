use std::{fmt::Debug, sync::Arc};

use serde::{Deserialize, Serialize};

use swc_common::{errors::HANDLER, SourceMap, Span};
use swc_ecma_ast::{Expr, Lit, Str, Tpl};
use swc_ecma_visit::{noop_visit_type, Visit, VisitWith};

use swc_ecma_lints::rule::Rule;

use crate::linter::{config::RuleConfig, rule::visitor_rule};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuotesConfig {
    #[serde(default)]
    prefer: QuotesType,
}

pub fn quotes(source_map: &Arc<SourceMap>, config: &RuleConfig<QuotesConfig>) -> Box<dyn Rule> {
    visitor_rule(Quotes::new(source_map.clone(), config))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, std::cmp::PartialEq)]
enum QuotesType {
    Single,
    Double,
    Backtick,
}

impl Default for QuotesType {
    fn default() -> Self {
        Self::Double
    }
}

impl QuotesType {
    pub fn from_bytes(bytes: u8) -> Option<QuotesType> {
        match bytes {
            b'\'' => Some(QuotesType::Single),
            b'"' => Some(QuotesType::Double),
            b'`' => Some(QuotesType::Backtick),
            _ => None,
        }
    }
}

#[derive(Default)]
struct Quotes {
    source_map: Arc<SourceMap>,
    prefer: QuotesType,
}

impl Quotes {
    fn new(source_map: Arc<SourceMap>, config: &RuleConfig<QuotesConfig>) -> Self {
        let quotes_config = config.get_rule_config();
        Self {
            source_map,
            prefer: quotes_config.prefer,
        }
    }

    // TODO: Get this from configuration
    fn get_preferred_type(&self) -> QuotesType {
        self.prefer
    }

    fn emit_error(&self, span: Span) {
        HANDLER.with(|handler| {
            handler
                .struct_span_err(
                    span,
                    format!(
                        "Incorrect quotes type. Expected {:?}.",
                        self.get_preferred_type()
                    )
                    .as_str(),
                )
                .emit();
        });
    }

    // Implementation for 'normal' strings - single and double quotes
    fn check_str(&self, str: &Str) {
        // Get quote type as bytes, for comparison later
        let quote = self.source_map.lookup_byte_offset(str.span.lo);
        let quote_index = quote.pos.0;
        let src = &quote.sf.src;
        let bytes = src.as_bytes()[quote_index as usize];

        // TODO: Get this from configuration
        let expected_type = self.get_preferred_type();

        // Output error if quotes type is not the same as expected
        if let Some(quotes_type) = QuotesType::from_bytes(bytes) {
            // If quotes type is as expected, ignore
            if quotes_type == expected_type {
                return;
            }

            // If quotes type is not as expected, output warning
            self.emit_error(str.span)
        }
    }

    // Implementation for template literal strings - backticks
    fn check_tpl(&self, tpl: &Tpl) {
        // If backticks are the preferred type, ignore
        if let QuotesType::Backtick = self.get_preferred_type() {
            return;
        }

        // If the template literal contains a variable reference, allow it
        if !tpl.exprs.is_empty() {
            return;
        }

        // Else, output error
        self.emit_error(tpl.span)
    }
}

impl Debug for Quotes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Quotes").finish()
    }
}

impl Visit for Quotes {
    noop_visit_type!();

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Tpl(tpl) => self.check_tpl(tpl),
            Expr::Lit(Lit::Str(str)) => self.check_str(str),
            _ => {}
        }

        expr.visit_children_with(self);
    }
}
