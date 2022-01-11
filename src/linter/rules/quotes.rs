use swc_ecma_lints::rule::Rule;
use swc_ecmascript::{
    ast::{Expr, Module, Str},
    visit::{noop_visit_type, Visit, VisitWith},
};

use super::visitor_rule;

pub fn quotes() -> Box<dyn Rule> {
    visitor_rule(Quotes::default())
}

#[derive(Debug, Default)]
struct Quotes;

impl Visit for Quotes {
    noop_visit_type!();

    fn visit_str(&mut self, str: &Str) {
        println!("{}", str.value.to_string());

        // does this have the correct type of quotes?
        // foo()

        str.visit_children_with(self);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        // println!("{:?}", expr);

        expr.visit_children_with(self);
    }

    fn visit_module(&mut self, program: &Module) {
        // println!("{:?}", program.body);
        program.visit_children_with(self);
    }
}
