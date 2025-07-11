use crate::math::ast::*;

pub fn format_expr_latex(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        Expr::Variable(v) => v.clone(),
        Expr::UnaryOp(UnaryOp::Neg, e) => format!("-{}", format_expr_latex(e)),
        Expr::BinaryOp(op, a, b) => match op {
            BinaryOp::Add => format!("{} + {}", format_expr_latex(a), format_expr_latex(b)),
            BinaryOp::Sub => format!("{} - {}", format_expr_latex(a), format_expr_latex(b)),
            BinaryOp::Mul => format!("{} {}", format_expr_latex(a), format_expr_latex(b)),
            BinaryOp::Div => format!("\\frac{{{}}}{{{}}}", format_expr_latex(a), format_expr_latex(b)),
            BinaryOp::Pow => format!("{}^{{{}}}", format_expr_latex(a), format_expr_latex(b)),
        },
        Expr::Function(name, arg) => {
            let latex_name = match name.as_str() {
                "sin" => "\\sin",
                "cos" => "\\cos",
                "tan" => "\\tan",
                "log" => "\\log",
                "exp" => "\\exp",
                "abs" => "\\left|",
                _ => name,
            };
            if name == "abs" {
                format!("{}{}\\right|", latex_name, format_expr_latex(arg))
            } else {
                format!("{}\\left({}\\right)", latex_name, format_expr_latex(arg))
            }
        }
    }
}

pub fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Variable(v) => v.clone(),
        Expr::UnaryOp(UnaryOp::Neg, e) => format!("-{}", format_expr(e)),
        Expr::BinaryOp(op, a, b) => {
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                BinaryOp::Pow => "^",
            };
            format!("({} {} {})", format_expr(a), op_str, format_expr(b))
        }
        Expr::Function(name, arg) => format!("{}({})", name, format_expr(arg)),
    }
}
