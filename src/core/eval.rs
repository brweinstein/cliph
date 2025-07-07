use std::collections::HashMap;
use crate::core::ast::*;

pub fn evaluate(expr: &Expr) -> f64 {
    evaluate_with_env(expr, &HashMap::new())
}

pub fn evaluate_with_env(expr: &Expr, vars: &HashMap<String, f64>) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Variable(name) => *vars.get(name).unwrap_or(&0.0),
        Expr::UnaryOp(UnaryOp::Neg, e) => -evaluate_with_env(e, vars),
        Expr::BinaryOp(op, a, b) => {
            let left = evaluate_with_env(a, vars);
            let right = evaluate_with_env(b, vars);
            match op {
                BinaryOp::Add => left + right,
                BinaryOp::Sub => left - right,
                BinaryOp::Mul => left * right,
                BinaryOp::Div => left / right,
                BinaryOp::Pow => left.powf(right),
            }
        }
        Expr::Function(f, arg) => {
            let x = evaluate_with_env(arg, vars);
            match f.as_str() {
                "sin" => x.sin(),
                "cos" => x.cos(),
                "tan" => x.tan(),
                "log" => x.ln(),
                "exp" => x.exp(),
                "abs" => x.abs(),
                _ => panic!("Unknown function: {}", f),
            }
        }
    }
}
