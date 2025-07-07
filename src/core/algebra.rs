use crate::core::ast::*;

pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::BinaryOp(BinaryOp::Add, a, b) => {
            let sa = simplify(a);
            let sb = simplify(b);
            match (&sa, &sb) {
                (Expr::Number(0.0), _) => sb,
                (_, Expr::Number(0.0)) => sa,
                _ => Expr::BinaryOp(BinaryOp::Add, Box::new(sa), Box::new(sb)),
            }
        }
        Expr::BinaryOp(BinaryOp::Mul, a, b) => {
            let sa = simplify(a);
            let sb = simplify(b);
            match (&sa, &sb) {
                (Expr::Number(1.0), _) => sb,
                (_, Expr::Number(1.0)) => sa,
                (Expr::Number(0.0), _) | (_, Expr::Number(0.0)) => Expr::Number(0.0),
                _ => Expr::BinaryOp(BinaryOp::Mul, Box::new(sa), Box::new(sb)),
            }
        }
        _ => expr.clone(),
    }
}
