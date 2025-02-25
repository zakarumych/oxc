use oxc_ast::{
    ast::{BinaryOperator, Expression, UnaryOperator},
    AstKind, Span,
};
use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::Error,
};
use oxc_macros::declare_oxc_lint;

use crate::{autofix::Fix, context::LintContext, rule::Rule, AstNode};

#[derive(Debug, Error, Diagnostic)]
#[error("eslint(eqeqeq): Expected {1:?} and instead saw {0:?}")]
#[diagnostic(severity(warning), help("Prefer strict {1} operator"))]
struct EqEqEqDiagnostic(&'static str, &'static str, #[label] pub Span);

#[derive(Debug, Default, Clone)]
pub struct EqEqEq;

declare_oxc_lint!(
    /// ### What it does
    /// Requires the use of the === and !== operators
    ///
    /// ### Why is this bad?
    /// Using non-strict equality operators leads to hard to track bugs due to type coercion.
    ///
    /// ### Example
    /// ```javascript
    /// let a = []
    /// let b = false
    /// a == b
    /// ```
    EqEqEq,
    nursery
);

fn to_strict_operator(operator: BinaryOperator) -> BinaryOperator {
    match operator {
        BinaryOperator::Equality => BinaryOperator::StrictEquality,
        BinaryOperator::Inequality => BinaryOperator::StrictInequality,
        _ => unreachable!(),
    }
}

impl Rule for EqEqEq {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::BinaryExpression(binary_expr) = node.get().kind() else { return };
        if !matches!(binary_expr.operator, BinaryOperator::Equality | BinaryOperator::Inequality) {
            return;
        }

        let is_valid_comparison = match (&binary_expr.left, &binary_expr.right) {
            (Expression::UnaryExpression(unary_expr), _)
            | (_, Expression::UnaryExpression(unary_expr)) => {
                matches!(unary_expr.operator, UnaryOperator::Typeof)
            }
            (lhs, rhs) => {
                (lhs.is_null() || rhs.is_null())
                    || lhs.is_literal_expression() && rhs.is_literal_expression()
            }
        };

        if !is_valid_comparison {
            let operator = binary_expr.operator.as_str();
            let prefered_operator = to_strict_operator(binary_expr.operator).as_str();
            ctx.diagnostic(EqEqEqDiagnostic(operator, prefered_operator, binary_expr.span));
            ctx.fix(Fix::new(prefered_operator, binary_expr.span));
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        ("typeof foo == 'undefined'", None),
        ("'hello' != 'world'", None),
        ("0 == 0", None),
        ("true == true", None),
        ("foo == null", None),
    ];

    let fail = vec![
        ("a == b", None),
        ("foo == true", None),
        ("bananas != 1", None),
        ("value == undefined", None),
    ];

    Tester::new(EqEqEq::NAME, pass, fail).test_and_snapshot();
}
