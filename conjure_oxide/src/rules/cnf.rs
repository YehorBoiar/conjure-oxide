use conjure_core::{ast::Expression as Expr, rule::RuleApplicationError};
use conjure_rules::register_rule;

/***********************************************************************************/
/*        This file contains rules for converting logic expressions to CNF         */
/***********************************************************************************/

/**
* Distribute `not` over `and` (De Morgan's Law):

* ```text
* not(and(a, b)) = or(not a, not b)
* ```
 */
#[register_rule]
fn distribute_not_over_and(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    match expr {
        Expr::Not(metadata, contents) => match contents.as_ref() {
            Expr::And(metadata, exprs) => {
                let mut new_exprs = Vec::new();
                for e in exprs {
                    new_exprs.push(Expr::Not(metadata.clone(), Box::new(e.clone())));
                }
                Ok(Expr::Or(metadata.clone(), new_exprs))
            }
            _ => Err(RuleApplicationError::RuleNotApplicable),
        },
        _ => Err(RuleApplicationError::RuleNotApplicable),
    }
}

/**
* Distribute `not` over `or` (De Morgan's Law):

* ```text
* not(or(a, b)) = and(not a, not b)
* ```
 */
#[register_rule]
fn distribute_not_over_or(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    match expr {
        Expr::Not(metadata, contents) => match contents.as_ref() {
            Expr::Or(metadata, exprs) => {
                let mut new_exprs = Vec::new();
                for e in exprs {
                    new_exprs.push(Expr::Not(metadata.clone(), Box::new(e.clone())));
                }
                Ok(Expr::And(metadata.clone(), new_exprs))
            }
            _ => Err(RuleApplicationError::RuleNotApplicable),
        },
        _ => Err(RuleApplicationError::RuleNotApplicable),
    }
}

/**
* Apply the Distributive Law to expressions like `Or([..., And(a, b)])`

* ```text
* or(and(a, b), c) = and(or(a, c), or(b, c))
* ```
 */
#[register_rule]
fn distribute_or_over_and(expr: &Expr) -> Result<Expr, RuleApplicationError> {
    fn find_and(exprs: &Vec<Expr>) -> Option<usize> {
        // ToDo: may be better to move this to some kind of utils module?
        for (i, e) in exprs.iter().enumerate() {
            match e {
                Expr::And(_, _) => return Some(i),
                _ => (),
            }
        }
        None
    }

    match expr {
        Expr::Or(metadata, exprs) => match find_and(exprs) {
            Some(idx) => {
                let mut rest = exprs.clone();
                let and_expr = rest.remove(idx);

                match and_expr {
                    Expr::And(metadata, and_exprs) => {
                        let mut new_and_contents = Vec::new();

                        for e in and_exprs {
                            // ToDo: Cloning everything may be a bit inefficient - discuss
                            let mut new_or_contents = rest.clone();
                            new_or_contents.push(e.clone());
                            new_and_contents.push(Expr::Or(metadata.clone(), new_or_contents))
                        }

                        Ok(Expr::And(metadata.clone(), new_and_contents))
                    }
                    _ => Err(RuleApplicationError::RuleNotApplicable),
                }
            }
            None => Err(RuleApplicationError::RuleNotApplicable),
        },
        _ => Err(RuleApplicationError::RuleNotApplicable),
    }
}
