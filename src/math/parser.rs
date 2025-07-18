use crate::math::ast::*;
use regex::Regex;
use std::str::Chars;

pub fn latex_to_math_expr(latex: &str) -> String {
    let mut s = latex.to_string();

    // Replace common LaTeX trig/log functions with simpler names
    s = s.replace(r"\sin", "sin");
    s = s.replace(r"\cos", "cos");
    s = s.replace(r"\tan", "tan");
    s = s.replace(r"\log", "log");
    s = s.replace(r"\exp", "exp");
    s = s.replace(r"\abs", "abs");

    // Replace \frac{a}{b} with (a)/(b)
    let re_frac = Regex::new(r"\\frac\s*\{([^}]*)\}\s*\{([^}]*)\}").unwrap();
    s = re_frac.replace_all(&s, "($1)/($2)").into_owned();

    // Remove $ signs commonly used in LaTeX math mode
    s = s.replace("$", "");

    s
}

pub fn parse(expr: &str) -> Result<Expr, String> {
    let mut parser = Parser::new(expr);
    parser.skip_whitespace();
    let parsed = parser.parse_expr()?;

    parser.skip_whitespace();
    if parser.curr.is_some() {
        return Err("Unexpected characters after expression".to_string());
    }

    Ok(parsed)
}

struct Parser<'a> {
    chars: Chars<'a>,
    curr: Option<char>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let curr = chars.next();
        Parser { chars, curr }
    }

    fn bump(&mut self) {
        self.curr = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.curr {
            if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_mul_div()?;
        loop {
            self.skip_whitespace();
            match self.curr {
                Some('+') => {
                    self.bump();
                    let rhs = self.parse_mul_div()?;
                    node = Expr::BinaryOp(BinaryOp::Add, Box::new(node), Box::new(rhs));
                }
                Some('-') => {
                    self.bump();
                    let rhs = self.parse_mul_div()?;
                    node = Expr::BinaryOp(BinaryOp::Sub, Box::new(node), Box::new(rhs));
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_pow()?;

        loop {
            self.skip_whitespace();

            match self.curr {
                Some('*') => {
                    self.bump();
                    let rhs = self.parse_pow()?;
                    node = Expr::BinaryOp(BinaryOp::Mul, Box::new(node), Box::new(rhs));
                }
                Some('/') => {
                    self.bump();
                    let rhs = self.parse_pow()?;
                    node = Expr::BinaryOp(BinaryOp::Div, Box::new(node), Box::new(rhs));
                }
                // Implicit multiplication detection:
                // If next token looks like the start of an atom/pow (number, letter, or '(')
                Some(c) if c.is_ascii_digit() || c.is_alphabetic() || c == '(' => {
                    // Parse next primary expression (power)
                    let rhs = self.parse_pow()?;
                    node = Expr::BinaryOp(BinaryOp::Mul, Box::new(node), Box::new(rhs));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_pow(&mut self) -> Result<Expr, String> {
        let base = self.parse_unary()?;
        self.skip_whitespace();
        if self.curr == Some('^') {
            self.bump();
            let exp = self.parse_unary()?;
            Ok(Expr::BinaryOp(BinaryOp::Pow, Box::new(base), Box::new(exp)))
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();
        if self.curr == Some('-') {
            self.bump();
            let expr = self.parse_unary()?;
            Ok(Expr::UnaryOp(UnaryOp::Neg, Box::new(expr)))
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();
        match self.curr {
            Some(c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(c) if c.is_alphabetic() => self.parse_ident_or_func(),
            Some('(') => {
                self.bump();
                let inner = self.parse_expr()?;
                self.skip_whitespace();
                if self.curr != Some(')') {
                    return Err("Expected ')'".into());
                }
                self.bump();
                Ok(inner)
            }
            Some(c) => Err(format!("Unexpected character '{}'", c)),
            None => Err("Unexpected end of input".into()),
        }
    }

    fn parse_number(&mut self) -> Result<Expr, String> {
        let mut num = String::new();
        while let Some(c) = self.curr {
            if c.is_ascii_digit() || c == '.' {
                num.push(c);
                self.bump();
            } else {
                break;
            }
        }
        num.parse()
            .map(Expr::Number)
            .map_err(|_| "Invalid number".into())
    }

    fn parse_ident_or_func(&mut self) -> Result<Expr, String> {
        let mut ident = String::new();
        while let Some(c) = self.curr {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.bump();
            } else {
                break;
            }
        }

        self.skip_whitespace();
        if self.curr == Some('(') {
            self.bump();
            let arg = self.parse_expr()?;
            self.skip_whitespace();
            if self.curr != Some(')') {
                return Err("Expected ')' after function argument".into());
            }
            self.bump();
            Ok(Expr::Function(ident, Box::new(arg)))
        } else {
            Ok(Expr::Variable(ident))
        }
    }
}
