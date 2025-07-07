use crate::core::ast::*;
use std::str::Chars;

pub fn parse(expr: &str) -> Result<Expr, String> {
    Parser::new(expr).parse_expr()
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

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_mul_div()?;
        while let Some(op) = self.curr {
            match op {
                '+' => {
                    self.bump();
                    node = Expr::BinaryOp(BinaryOp::Add, Box::new(node), Box::new(self.parse_mul_div()?));
                }
                '-' => {
                    self.bump();
                    node = Expr::BinaryOp(BinaryOp::Sub, Box::new(node), Box::new(self.parse_mul_div()?));
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_pow()?;
        while let Some(op) = self.curr {
            match op {
                '*' => {
                    self.bump();
                    node = Expr::BinaryOp(BinaryOp::Mul, Box::new(node), Box::new(self.parse_pow()?));
                }
                '/' => {
                    self.bump();
                    node = Expr::BinaryOp(BinaryOp::Div, Box::new(node), Box::new(self.parse_pow()?));
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_pow(&mut self) -> Result<Expr, String> {
        let base = self.parse_unary()?;
        if let Some('^') = self.curr {
            self.bump();
            let exp = self.parse_unary()?;
            Ok(Expr::BinaryOp(BinaryOp::Pow, Box::new(base), Box::new(exp)))
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Some('-') = self.curr {
            self.bump();
            Ok(Expr::UnaryOp(UnaryOp::Neg, Box::new(self.parse_unary()?)))
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();
        match self.curr {
            Some(c) if c.is_ascii_digit() => self.parse_number(),
            Some(c) if c.is_alphabetic() => self.parse_ident_or_func(),
            Some('(') => {
                self.bump();
                let inner = self.parse_expr()?;
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
        num.parse().map(Expr::Number).map_err(|_| "Invalid number".into())
    }

    fn parse_ident_or_func(&mut self) -> Result<Expr, String> {
        let mut ident = String::new();
        while let Some(c) = self.curr {
            if c.is_alphanumeric() {
                ident.push(c);
                self.bump();
            } else {
                break;
            }
        }

        if self.curr == Some('(') {
            self.bump();
            let arg = self.parse_expr()?;
            if self.curr != Some(')') {
                return Err("Expected ')' after function argument".into());
            }
            self.bump();
            Ok(Expr::Function(ident, Box::new(arg)))
        } else {
            Ok(Expr::Variable(ident))
        }
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
}
