use crate::math::algebra::simplify;
use crate::math::ast::*;
use ordered_float::OrderedFloat; // Adjust path as needed

pub fn differentiate(expr: &Expr, var: &str) -> Expr {
    simplify(&expr);
    let diffed = match expr {
        Expr::Number(_) => Expr::Number(OrderedFloat(0.0)),

        Expr::Variable(name) => {
            if name == var {
                Expr::Number(OrderedFloat(1.0))
            } else {
                Expr::Number(OrderedFloat(0.0))
            }
        }

        Expr::UnaryOp(UnaryOp::Neg, e) => {
            Expr::UnaryOp(UnaryOp::Neg, Box::new(differentiate(e, var)))
        }

        Expr::BinaryOp(op, a, b) => match op {
            BinaryOp::Add => Expr::BinaryOp(
                BinaryOp::Add,
                Box::new(differentiate(a, var)),
                Box::new(differentiate(b, var)),
            ),

            BinaryOp::Sub => Expr::BinaryOp(
                BinaryOp::Sub,
                Box::new(differentiate(a, var)),
                Box::new(differentiate(b, var)),
            ),

            BinaryOp::Mul => Expr::BinaryOp(
                BinaryOp::Add,
                Box::new(Expr::BinaryOp(
                    BinaryOp::Mul,
                    Box::new(differentiate(a, var)),
                    b.clone(),
                )),
                Box::new(Expr::BinaryOp(
                    BinaryOp::Mul,
                    a.clone(),
                    Box::new(differentiate(b, var)),
                )),
            ),

            BinaryOp::Div => {
                let u = a.clone();
                let v = b.clone();
                Expr::BinaryOp(
                    BinaryOp::Div,
                    Box::new(Expr::BinaryOp(
                        BinaryOp::Sub,
                        Box::new(Expr::BinaryOp(
                            BinaryOp::Mul,
                            Box::new(differentiate(&u, var)),
                            v.clone(),
                        )),
                        Box::new(Expr::BinaryOp(
                            BinaryOp::Mul,
                            u.clone(),
                            Box::new(differentiate(&v, var)),
                        )),
                    )),
                    Box::new(Expr::BinaryOp(
                        BinaryOp::Pow,
                        v.clone(),
                        Box::new(Expr::Number(OrderedFloat(2.0))),
                    )),
                )
            }

            BinaryOp::Pow => match (&**a, &**b) {
                (_, Expr::Number(n)) => Expr::BinaryOp(
                    BinaryOp::Mul,
                    Box::new(Expr::BinaryOp(
                        BinaryOp::Mul,
                        Box::new(Expr::Number(*n)),
                        Box::new(Expr::BinaryOp(
                            BinaryOp::Pow,
                            a.clone(),
                            Box::new(Expr::Number(OrderedFloat(n.0 - 1.0))),
                        )),
                    )),
                    Box::new(differentiate(a, var)),
                ),

                _ => Expr::Function("diff_not_supported".into(), Box::new(expr.clone())),
            },
        },

        Expr::Function(name, arg) => {
            let d_arg = differentiate(arg, var);
            let inner = arg.clone();
            match name.as_str() {
                "sin" => Expr::BinaryOp(
                    BinaryOp::Mul,
                    Box::new(Expr::Function("cos".into(), inner.clone())),
                    Box::new(d_arg),
                ),

                "cos" => Expr::BinaryOp(
                    BinaryOp::Mul,
                    Box::new(Expr::UnaryOp(
                        UnaryOp::Neg,
                        Box::new(Expr::Function("sin".into(), inner.clone())),
                    )),
                    Box::new(d_arg),
                ),

                "exp" => Expr::BinaryOp(
                    BinaryOp::Mul,
                    Box::new(Expr::Function("exp".into(), inner.clone())),
                    Box::new(d_arg),
                ),

                "log" => Expr::BinaryOp(BinaryOp::Div, Box::new(d_arg), inner),

                _ => Expr::Function("diff_not_supported".into(), Box::new(expr.clone())),
            }
        }
    };
    simplify(&diffed)
}
