use crate::math::ast::*;

pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::Number(_) | Expr::Variable(_) => expr.clone(),

        Expr::UnaryOp(op, e) => {
            let se = simplify(e);
            match (op, &se) {
                (UnaryOp::Neg, Expr::Number(n)) => Expr::Number(-n),
                _ => Expr::UnaryOp(op.clone(), Box::new(se)),
            }
        }

        Expr::BinaryOp(op, a, b) => {
            let sa = simplify(a);
            let sb = simplify(b);

            match op {
                BinaryOp::Add => {
                    let mut terms = flatten_add(&sa);
                    terms.extend(flatten_add(&sb));

                    let (sum_consts, mut non_consts) = partition_consts(terms);

                    let mut result = if sum_consts != 0.0 {
                        vec![Expr::Number(sum_consts)]
                    } else {
                        vec![]
                    };

                    result.append(&mut non_consts);

                    if result.is_empty() {
                        Expr::Number(0.0)
                    } else if result.len() == 1 {
                        result.into_iter().next().unwrap()
                    } else {
                        fold_binary_ops(BinaryOp::Add, result)
                    }
                }

                BinaryOp::Mul => {
                    let mut factors = flatten_mul(&sa);
                    factors.extend(flatten_mul(&sb));

                    let (prod_consts, mut non_consts) = partition_consts_mul(factors);

                    if prod_consts == 0.0 {
                        Expr::Number(0.0)
                    } else {
                        let mut result = if prod_consts != 1.0 {
                            vec![Expr::Number(prod_consts)]
                        } else {
                            vec![]
                        };

                        result.append(&mut non_consts);

                        if result.is_empty() {
                            Expr::Number(1.0)
                        } else if result.len() == 1 {
                            result.into_iter().next().unwrap()
                        } else {
                            fold_binary_ops(BinaryOp::Mul, result)
                        }
                    }
                }

                BinaryOp::Sub => {
                    let sb_neg = simplify(&Expr::UnaryOp(UnaryOp::Neg, Box::new(sb)));
                    simplify(&Expr::BinaryOp(BinaryOp::Add, Box::new(sa), Box::new(sb_neg)))
                }

                BinaryOp::Div => {
                    if sb == Expr::Number(1.0) {
                        sa
                    } else if sa == Expr::Number(0.0) {
                        Expr::Number(0.0)
                    } else {
                        Expr::BinaryOp(BinaryOp::Div, Box::new(sa), Box::new(sb))
                    }
                }

                BinaryOp::Pow => match &sb {
                    Expr::Number(0.0) => Expr::Number(1.0),
                    Expr::Number(1.0) => sa,
                    _ => Expr::BinaryOp(BinaryOp::Pow, Box::new(sa), Box::new(sb)),
                },
            }
        }

        Expr::Function(name, arg) => {
            let sarg = simplify(arg);
            if let Expr::Number(n) = &sarg {
                let val = match name.as_str() {
                    "sin" => n.sin(),
                    "cos" => n.cos(),
                    "tan" => n.tan(),
                    "log" => {
                        if *n > 0.0 {
                            n.ln()
                        } else {
                            return Expr::Function(name.clone(), Box::new(sarg));
                        }
                    }
                    "exp" => n.exp(),
                    "abs" => n.abs(),
                    _ => return Expr::Function(name.clone(), Box::new(sarg)),
                };
                Expr::Number(val)
            } else {
                Expr::Function(name.clone(), Box::new(sarg))
            }
        }
    }
}

fn flatten_add(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::BinaryOp(BinaryOp::Add, a, b) => {
            let mut v = flatten_add(a);
            v.extend(flatten_add(b));
            v
        }
        _ => vec![expr.clone()],
    }
}

fn flatten_mul(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::BinaryOp(BinaryOp::Mul, a, b) => {
            let mut v = flatten_mul(a);
            v.extend(flatten_mul(b));
            v
        }
        _ => vec![expr.clone()],
    }
}

fn partition_consts(terms: Vec<Expr>) -> (f64, Vec<Expr>) {
    let mut sum_consts = 0.0;
    let mut non_consts = Vec::new();
    for term in terms {
        if let Expr::Number(n) = term {
            sum_consts += n;
        } else {
            non_consts.push(term);
        }
    }
    (sum_consts, non_consts)
}

fn partition_consts_mul(factors: Vec<Expr>) -> (f64, Vec<Expr>) {
    let mut prod_consts = 1.0;
    let mut non_consts = Vec::new();
    for factor in factors {
        if let Expr::Number(n) = factor {
            prod_consts *= n;
        } else {
            non_consts.push(factor);
        }
    }
    (prod_consts, non_consts)
}

fn fold_binary_ops(op: BinaryOp, mut exprs: Vec<Expr>) -> Expr {
    if exprs.len() == 1 {
        exprs.pop().unwrap()
    } else {
        let right = exprs.pop().unwrap();
        let left = fold_binary_ops(op.clone(), exprs);
        Expr::BinaryOp(op, Box::new(left), Box::new(right))
    }
}
