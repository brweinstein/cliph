use cliph::math::algebra::simplify;
use cliph::math::ast::BinaryOp::*;
use cliph::math::ast::Expr::*;
use cliph::math::ast::UnaryOp::*;
use cliph::BinaryOp;
use cliph::Expr;
use ordered_float::OrderedFloat;

fn num(n: f64) -> Expr {
    Number(OrderedFloat(n))
}

fn var(name: &str) -> Expr {
    Variable(name.to_string())
}

fn func(name: &str, arg: Expr) -> Expr {
    Function(name.to_string(), Box::new(arg))
}

fn fold_binary_ops(op: BinaryOp, mut exprs: Vec<Expr>) -> Expr {
    if exprs.len() == 1 {
        exprs.pop().unwrap()
    } else {
        let right = exprs.pop().unwrap();
        let left = fold_binary_ops(op.clone(), exprs);
        BinaryOp(op, Box::new(left), Box::new(right))
    }
}

#[test]
fn test_number_identity() {
    assert_eq!(simplify(&num(3.14)), num(3.14));
}

#[test]
fn test_variable_identity() {
    assert_eq!(simplify(&var("x")), var("x"));
}

#[test]
fn test_add_zero() {
    assert_eq!(
        simplify(&BinaryOp(Add, Box::new(var("x")), Box::new(num(0.0)))),
        var("x")
    );
}

#[test]
fn test_mul_one() {
    assert_eq!(
        simplify(&BinaryOp(Mul, Box::new(var("x")), Box::new(num(1.0)))),
        var("x")
    );
}

#[test]
fn test_mul_zero() {
    assert_eq!(
        simplify(&BinaryOp(Mul, Box::new(var("x")), Box::new(num(0.0)))),
        num(0.0)
    );
}

#[test]
fn test_pow_zero() {
    assert_eq!(
        simplify(&BinaryOp(Pow, Box::new(var("x")), Box::new(num(0.0)))),
        num(1.0)
    );
}

#[test]
fn test_pow_one() {
    assert_eq!(
        simplify(&BinaryOp(Pow, Box::new(var("x")), Box::new(num(1.0)))),
        var("x")
    );
}

#[test]
fn test_addition_constants() {
    assert_eq!(
        simplify(&BinaryOp(Add, Box::new(num(3.0)), Box::new(num(2.0)))),
        num(5.0)
    );
}

#[test]
fn test_combine_like_terms() {
    let expr = BinaryOp(
        Add,
        Box::new(BinaryOp(Mul, Box::new(num(2.0)), Box::new(var("x")))),
        Box::new(var("x")),
    );
    assert_eq!(
        simplify(&expr),
        BinaryOp(Mul, Box::new(num(3.0)), Box::new(var("x")))
    );
}

#[test]
fn test_combine_constants_and_vars() {
    let expr = BinaryOp(
        Add,
        Box::new(num(5.0)),
        Box::new(BinaryOp(Add, Box::new(num(2.0)), Box::new(var("x")))),
    );
    assert_eq!(
        simplify(&expr),
        BinaryOp(Add, Box::new(num(7.0)), Box::new(var("x")))
    );
}

#[test]
fn test_add_double_neg() {
    let expr = BinaryOp(
        Add,
        Box::new(UnaryOp(Neg, Box::new(UnaryOp(Neg, Box::new(var("x")))))),
        Box::new(num(2.0)),
    );

    let simplified = simplify(&expr);

    let expected1 = BinaryOp(Add, Box::new(var("x")), Box::new(num(2.0)));

    let expected2 = BinaryOp(Add, Box::new(num(2.0)), Box::new(var("x")));

    assert!(
        simplified == expected1 || simplified == expected2,
        "simplified = {:?}, expected either {:?} or {:?}",
        simplified,
        expected1,
        expected2
    );
}

#[test]
fn test_subtract_same_var() {
    let expr = BinaryOp(Sub, Box::new(var("x")), Box::new(var("x")));
    assert_eq!(simplify(&expr), num(0.0));
}

#[test]
fn test_subtract_constants() {
    let expr = BinaryOp(Sub, Box::new(num(5.0)), Box::new(num(3.0)));
    assert_eq!(simplify(&expr), num(2.0));
}

#[test]
fn test_subtract_distributes_negation() {
    let expr = BinaryOp(
        Sub,
        Box::new(num(2.0)),
        Box::new(BinaryOp(Add, Box::new(var("x")), Box::new(num(3.0)))),
    );
    let simplified = simplify(&expr);

    // Expected simplified form: (-1) + (-x)
    let expected1 = BinaryOp(
        Add,
        Box::new(num(-1.0)),
        Box::new(UnaryOp(Neg, Box::new(var("x")))),
    );

    let expected2 = BinaryOp(
        Add,
        Box::new(UnaryOp(Neg, Box::new(var("x")))),
        Box::new(num(-1.0)),
    );

    assert!(
        simplified == expected1 || simplified == expected2,
        "simplified = {:?}, expected either {:?} or {:?}",
        simplified,
        expected1,
        expected2
    );
}

#[test]
fn test_multiply_constants() {
    assert_eq!(
        simplify(&BinaryOp(Mul, Box::new(num(3.0)), Box::new(num(4.0)))),
        num(12.0)
    );
}

#[test]
fn test_multiply_and_combine_like_terms() {
    let expr = BinaryOp(
        Add,
        Box::new(BinaryOp(Mul, Box::new(num(2.0)), Box::new(var("x")))),
        Box::new(BinaryOp(Mul, Box::new(num(3.0)), Box::new(var("x")))),
    );
    assert_eq!(
        simplify(&expr),
        BinaryOp(Mul, Box::new(num(5.0)), Box::new(var("x")))
    );
}

#[test]
fn test_div_by_one() {
    assert_eq!(
        simplify(&BinaryOp(Div, Box::new(var("x")), Box::new(num(1.0)))),
        var("x")
    );
}

#[test]
fn test_zero_div() {
    assert_eq!(
        simplify(&BinaryOp(Div, Box::new(num(0.0)), Box::new(var("x")))),
        num(0.0)
    );
}

#[test]
fn test_div_constants() {
    assert_eq!(
        simplify(&BinaryOp(Div, Box::new(num(6.0)), Box::new(num(2.0)))),
        num(3.0)
    );
}

#[test]
fn test_nested_add_flattened() {
    let expr = BinaryOp(
        Add,
        Box::new(BinaryOp(Add, Box::new(var("x")), Box::new(var("y")))),
        Box::new(var("z")),
    );
    let expected = fold_binary_ops(Add, vec![var("x"), var("y"), var("z")]);
    assert_eq!(simplify(&expr), expected);
}

#[test]
fn test_nested_mul_flattened() {
    let expr = BinaryOp(
        Mul,
        Box::new(BinaryOp(Mul, Box::new(num(2.0)), Box::new(var("x")))),
        Box::new(num(3.0)),
    );
    assert_eq!(
        simplify(&expr),
        BinaryOp(Mul, Box::new(num(6.0)), Box::new(var("x")))
    );
}

#[test]
fn test_double_negation() {
    let expr = UnaryOp(Neg, Box::new(UnaryOp(Neg, Box::new(var("x")))));
    assert_eq!(simplify(&expr), var("x"));
}

#[test]
fn test_negate_number() {
    assert_eq!(simplify(&UnaryOp(Neg, Box::new(num(3.0)))), num(-3.0));
}

#[test]
fn test_sin_number() {
    assert_eq!(simplify(&func("sin", num(0.0))), num(0.0));
}

#[test]
fn test_cos_zero() {
    assert_eq!(simplify(&func("cos", num(0.0))), num(1.0));
}

#[test]
fn test_exp_zero() {
    assert_eq!(simplify(&func("exp", num(0.0))), num(1.0));
}

#[test]
fn test_log_one() {
    assert_eq!(simplify(&func("log", num(1.0))), num(0.0));
}

#[test]
fn test_abs_negative() {
    assert_eq!(simplify(&func("abs", num(-5.0))), num(5.0));
}

#[test]
fn test_function_not_evaluable() {
    let expr = func("sin", var("x"));
    assert_eq!(simplify(&expr), expr);
}

#[test]
fn test_log_negative_should_not_simplify() {
    let expr = func("log", num(-1.0));
    assert_eq!(simplify(&expr), expr);
}

#[test]
fn test_add_like_functions() {
    let expr = BinaryOp(
        Add,
        Box::new(func("cos", var("x"))),
        Box::new(func("cos", var("x"))),
    );
    let simplified = simplify(&expr);

    let expected = BinaryOp(Mul, Box::new(num(2.0)), Box::new(func("cos", var("x"))));

    assert_eq!(simplified, expected);
}

#[test]
fn test_pythagorean_identity() {
    use Expr::*;

    let x = var("x");
    let sin2 = BinaryOp(Pow, Box::new(func("sin", x.clone())), Box::new(num(2.0)));
    let cos2 = BinaryOp(Pow, Box::new(func("cos", x.clone())), Box::new(num(2.0)));

    let expr = BinaryOp(Add, Box::new(sin2), Box::new(cos2));
    let simplified = simplify(&expr);
    assert_eq!(simplified, num(1.0));
}

#[test]
fn test_nested_add_constants_flattened() {
    let expr = BinaryOp(
        Add,
        Box::new(BinaryOp(Add, Box::new(num(1.0)), Box::new(num(2.0)))),
        Box::new(num(3.0)),
    );
    assert_eq!(simplify(&expr), num(6.0));
}

#[test]
fn test_nested_mul_constants_flattened() {
    let expr = BinaryOp(
        Mul,
        Box::new(BinaryOp(Mul, Box::new(num(2.0)), Box::new(num(3.0)))),
        Box::new(num(4.0)),
    );
    assert_eq!(simplify(&expr), num(24.0));
}

#[test]
fn test_pythagorean_identity_cos2_plus_sin2_reversed() {
    let x = var("x");
    let cos2 = BinaryOp(Pow, Box::new(func("cos", x.clone())), Box::new(num(2.0)));
    let sin2 = BinaryOp(Pow, Box::new(func("sin", x.clone())), Box::new(num(2.0)));

    let expr = BinaryOp(Add, Box::new(cos2), Box::new(sin2));
    assert_eq!(simplify(&expr), num(1.0));
}

#[test]
fn test_subtract_identical_functions() {
    let expr = BinaryOp(
        Sub,
        Box::new(func("cos", var("x"))),
        Box::new(func("cos", var("x"))),
    );
    assert_eq!(simplify(&expr), num(0.0));
}

#[test]
fn test_negation_of_sum() {
    let expr = UnaryOp(
        Neg,
        Box::new(BinaryOp(Add, Box::new(var("x")), Box::new(var("y")))),
    );

    let simplified = simplify(&expr);

    let expected = BinaryOp(
        Add,
        Box::new(UnaryOp(Neg, Box::new(var("x")))),
        Box::new(UnaryOp(Neg, Box::new(var("y")))),
    );

    assert_eq!(simplified, simplify(&expected));
}

#[test]
fn test_combine_trig_like_terms() {
    let expr = BinaryOp(
        Add,
        Box::new(func("sin", var("x"))),
        Box::new(func("sin", var("x"))),
    );

    let simplified = simplify(&expr);
    let expected = BinaryOp(Mul, Box::new(num(2.0)), Box::new(func("sin", var("x"))));

    assert_eq!(simplified, expected);
}

#[test]
fn test_power_zero_nested() {
    let expr = BinaryOp(
        Pow,
        Box::new(BinaryOp(Add, Box::new(var("x")), Box::new(num(1.0)))),
        Box::new(num(0.0)),
    );
    assert_eq!(simplify(&expr), num(1.0));
}
