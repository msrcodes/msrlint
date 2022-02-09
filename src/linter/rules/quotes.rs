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

impl Visit for Quotes {
    noop_visit_type!();

    fn visit_str(&mut self, str: &Str) {
        let quote = self.source_map.lookup_byte_offset(str.span.lo);
        let quote_index = quote.pos.0;
        let src = &quote.sf.src;

        let x = src.as_str().chars().nth(quote_index as usize);
        println!("{}", x.unwrap());

        // does this have the correct type of quotes?
        // foo()

        str.visit_children_with(self);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        expr.visit_children_with(self);
    }

    fn visit_module(&mut self, program: &Module) {
        program.visit_children_with(self);
    }
}
