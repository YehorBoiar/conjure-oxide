//! Normalising rules for `Neq` and `Eq`.

use conjure_core::ast::{Expression as Expr, SymbolTable};
use conjure_core::rule_engine::{
    register_rule, ApplicationError::RuleNotApplicable, ApplicationResult, Reduction,
};

use conjure_essence_macros::essence_expr;
use Expr::*;

/// Converts a negated `Neq` to an `Eq`
///
/// ```text
/// not(neq(x)) ~> eq(x)
/// ```
#[register_rule(("Base", 8800))]
fn negated_neq_to_eq(expr: &Expr, _: &SymbolTable) -> ApplicationResult {
    match expr {
        Not(_, a) => match a.as_ref() {
            Neq(_, b, c) if (b.is_safe() && c.is_safe()) => {
                Ok(Reduction::pure(essence_expr!(&b = &c)))
            }
            _ => Err(RuleNotApplicable),
        },
        _ => Err(RuleNotApplicable),
    }
}

/// Converts a negated `Eq` to an `Neq`
///
/// ```text
/// not(eq(x)) ~> neq(x)
/// ```
#[register_rule(("Base", 8800))]
fn negated_eq_to_neq(expr: &Expr, _: &SymbolTable) -> ApplicationResult {
    match expr {
        Not(_, a) => match a.as_ref() {
            Eq(_, b, c) if (b.is_safe() && c.is_safe()) => {
                Ok(Reduction::pure(essence_expr!(&b != &c)))
            }
            _ => Err(RuleNotApplicable),
        },
        _ => Err(RuleNotApplicable),
    }
}
