use crate::errors::EssenceParseError;
use conjure_core::ast::{Expression, Range, SymbolTable};
use conjure_core::error::Error;
#[allow(unused)]
use uniplate::Uniplate;

use super::expression::parse_expression;
use super::util::{get_tree, query_toplevel};

pub fn parse_expr(src: &str, symbol_table: &SymbolTable) -> Result<Expression, EssenceParseError> {
    let exprs = parse_exprs(src, symbol_table)?;
    if exprs.len() != 1 {
        return Err(EssenceParseError::ParseError(Error::Parse(
            "Expected exactly one expression".into(),
        )));
    }
    Ok(exprs[0].clone())
}

pub fn parse_exprs(
    src: &str,
    symbol_table: &SymbolTable,
) -> Result<Vec<Expression>, EssenceParseError> {
    let (tree, source_code) = get_tree(src).ok_or(EssenceParseError::TreeSitterError(
        "Failed to parse Essence source code".to_string(),
    ))?;

    let root = tree.root_node();
    let mut ans = Vec::new();
    for expr in query_toplevel(&root, &|n| n.kind() == "expression") {
        ans.push(parse_expression(expr, &source_code, &root, symbol_table)?);
    }
    Ok(ans)
}

mod test {
    #[allow(unused)]
    use super::{parse_expr, parse_exprs};
    #[allow(unused)]
    use conjure_core::ast::{Declaration, Domain, Name, SymbolTable};
    #[allow(unused)]
    use conjure_core::{ast::Atom, ast::Expression, metadata::Metadata};
    #[allow(unused)]
    use std::collections::HashMap;
    #[allow(unused)]
    use std::sync::Arc;
    #[allow(unused)]
    use std::{cell::RefCell, rc::Rc};
    #[allow(unused)]
    use tree_sitter::Range;

    #[test]
    pub fn test_parse_expressions() {
        let src = "x >= 5, y = a / 2";
        let mut symbols = SymbolTable::new();
        let x: Rc<Declaration> = Rc::new(Declaration::new_var(
            Name::UserName("x".into()),
            Domain::BoolDomain,
        ));
        let x_refcell = Rc::new(RefCell::new(Declaration::new_var(
            Name::UserName("x".into()),
            Domain::BoolDomain,
        )));
        let y: Rc<Declaration> = Rc::new(Declaration::new_var(
            Name::UserName("y".into()),
            Domain::IntDomain(vec![conjure_core::ast::Range::Bounded(0, 10)]),
        ));
        let y_refcell = Rc::new(RefCell::new(Declaration::new_var(
            Name::UserName("y".into()),
            Domain::IntDomain(vec![conjure_core::ast::Range::Bounded(0, 10)]),
        )));
        let a: Rc<Declaration> = Rc::new(Declaration::new_var(
            Name::UserName("a".into()),
            Domain::IntDomain(vec![conjure_core::ast::Range::Bounded(0, 10)]),
        ));
        let a_refcell = Rc::new(RefCell::new(Declaration::new_var(
            Name::UserName("a".into()),
            Domain::IntDomain(vec![conjure_core::ast::Range::Bounded(0, 10)]),
        )));
        symbols
            .insert(x)
            .expect("x should not exist in the symbol-table yet, so we should be able to add it");

        symbols
            .insert(y)
            .expect("y should not exist in the symbol-table yet, so we should be able to add it");

        symbols
            .insert(a)
            .expect("a should not exist in the symbol-table yet, so we should be able to add it");

        let exprs = parse_exprs(src, &symbols).unwrap();
        assert_eq!(exprs.len(), 2);
        assert_eq!(
            exprs[0],
            Expression::Geq(
                Metadata::new(),
                Box::new(Expression::Atomic(
                    Metadata::new(),
                    Atom::new_uref_with_decl("x", x_refcell)
                )),
                Box::new(Expression::Atomic(Metadata::new(), 5.into()))
            )
        );
        assert_eq!(
            exprs[1],
            Expression::Eq(
                Metadata::new(),
                Box::new(Expression::Atomic(
                    Metadata::new(),
                    Atom::new_uref_with_decl("y", y_refcell)
                )),
                Box::new(Expression::UnsafeDiv(
                    Metadata::new(),
                    Box::new(Expression::Atomic(
                        Metadata::new(),
                        Atom::new_uref_with_decl("a", a_refcell)
                    )),
                    Box::new(Expression::Atomic(Metadata::new(), 2.into()))
                ))
            )
        );
    }
}
