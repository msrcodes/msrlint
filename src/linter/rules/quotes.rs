use std::{fmt::Debug, sync::Arc};

use swc_common::SourceMap;
use swc_ecma_ast::{Expr, Module, Str};
use swc_ecma_visit::{noop_visit_type, Visit, VisitWith};

use swc_ecma_lints::rule::Rule;

use super::visitor_rule;

pub fn quotes(source_map: &Arc<SourceMap>) -> Box<dyn Rule> {
    visitor_rule(Quotes::new(source_map.clone()))
}

#[derive(Default)]
struct Quotes {
    source_map: Arc<SourceMap>,
}

impl Quotes {
    fn new(source_map: Arc<SourceMap>) -> Self {
        Self { source_map }
    }
}

impl Debug for Quotes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Quotes").finish()
    }
}

#[derive(std::cmp::PartialEq, Debug)]
enum QuotesType {
    Single,
    Double,
    Backtick,
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

impl Visit for Quotes {
    noop_visit_type!();

    fn visit_str(&mut self, str: &Str) {
        // Get quote type as bytes, for comparison later
        let quote = self.source_map.lookup_byte_offset(str.span.lo);
        let quote_index = quote.pos.0;
        let src = &quote.sf.src;
        let bytes = src.as_bytes()[quote_index as usize];

        // TODO: Get this from configuration
        let expected_type = QuotesType::Double;

        // Output error if quotes type is not the same as expected
        if let Some(quotes_type) = QuotesType::from_bytes(bytes) {
            // If quotes type is as expected, ignore
            if quotes_type == expected_type {
                return;
            }

            // TODO: test for template literals - if it contains a variable, we allow it

            // If quotes type is not as expected, output warning
            println!(
                "Unexpected quotes type in str \"{}\", found \"{:?}\", expected \"{:?}\"",
                str.value, quotes_type, expected_type
            )
        }

        str.visit_children_with(self);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        expr.visit_children_with(self);
    }

    fn visit_module(&mut self, program: &Module) {
        program.visit_children_with(self);
    }
}
