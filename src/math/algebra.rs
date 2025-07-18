use crate::math::ast::*;
use ordered_float::OrderedFloat;

pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::Number(_) | Expr::Variable(_) => expr.clone(),

        Expr::UnaryOp(op, e) => {
            let se = simplify(e);
            match (op, &se) {
                (UnaryOp::Neg, Expr::Number(n)) => Expr::Number(OrderedFloat(-n.0)),

                (UnaryOp::Neg, Expr::UnaryOp(UnaryOp::Neg, inner)) => *inner.clone(), // double negation

                // Distribute negation over addition: -(a + b) = (-a) + (-b)
                (UnaryOp::Neg, Expr::BinaryOp(BinaryOp::Add, a, b)) => {
                    let neg_a = simplify(&Expr::UnaryOp(UnaryOp::Neg, a.clone()));
                    let neg_b = simplify(&Expr::UnaryOp(UnaryOp::Neg, b.clone()));
                    Expr::BinaryOp(BinaryOp::Add, Box::new(neg_a), Box::new(neg_b))
                }

                // Distribute negation over subtraction: -(a - b) = (-a) + b
                (UnaryOp::Neg, Expr::BinaryOp(BinaryOp::Sub, a, b)) => {
                    let neg_a = simplify(&Expr::UnaryOp(UnaryOp::Neg, a.clone()));
                    Expr::BinaryOp(BinaryOp::Add, Box::new(neg_a), b.clone())
                }

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

                    // Inserted: Check for sin²(x) + cos²(x)
                    if terms.len() == 2 {
                        if let (
                            Expr::BinaryOp(BinaryOp::Pow, s1, e1),
                            Expr::BinaryOp(BinaryOp::Pow, s2, e2),
                        ) = (&terms[0], &terms[1])
                        {
                            if let (
                                Expr::Function(f1, arg1),
                                Expr::Function(f2, arg2),
                                Expr::Number(n1),
                                Expr::Number(n2),
                            ) = (&**s1, &**s2, &**e1, &**e2)
                            {
                                if n1 == &OrderedFloat(2.0)
                                    && n2 == &OrderedFloat(2.0)
                                    && ((f1 == "sin" && f2 == "cos")
                                        || (f1 == "cos" && f2 == "sin"))
                                    && arg1 == arg2
                                {
                                    return Expr::Number(OrderedFloat(1.0));
                                }
                            }
                        }
                    }

                    let combined_terms = combine_like_terms(terms);
                    let (sum_consts, mut non_consts) = partition_consts(combined_terms);

                    let mut result = if sum_consts.0.abs() > 1e-12 {
                        vec![Expr::Number(sum_consts)]
                    } else {
                        vec![]
                    };

                    result.append(&mut non_consts);

                    if result.is_empty() {
                        Expr::Number(OrderedFloat(0.0))
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

                    if prod_consts == OrderedFloat(0.0) {
                        Expr::Number(OrderedFloat(0.0))
                    } else {
                        let mut result = if (prod_consts.0 - 1.0).abs() > 1e-12 {
                            vec![Expr::Number(prod_consts)]
                        } else {
                            vec![]
                        };

                        result.append(&mut non_consts);

                        if result.is_empty() {
                            Expr::Number(OrderedFloat(1.0))
                        } else if result.len() == 1 {
                            result.into_iter().next().unwrap()
                        } else {
                            fold_binary_ops(BinaryOp::Mul, result)
                        }
                    }
                }

                BinaryOp::Sub => {
                    if sa == sb {
                        Expr::Number(OrderedFloat(0.0))
                    } else {
                        // Distribute negation over addition if sb is sum
                        match sb {
                            Expr::BinaryOp(BinaryOp::Add, left, right) => {
                                let neg_left = simplify(&Expr::UnaryOp(UnaryOp::Neg, left));
                                let neg_right = simplify(&Expr::UnaryOp(UnaryOp::Neg, right));
                                simplify(&Expr::BinaryOp(
                                    BinaryOp::Add,
                                    Box::new(sa),
                                    Box::new(simplify(&Expr::BinaryOp(
                                        BinaryOp::Add,
                                        Box::new(neg_left),
                                        Box::new(neg_right),
                                    ))),
                                ))
                            }
                            _ => {
                                let sb_neg = simplify(&Expr::UnaryOp(UnaryOp::Neg, Box::new(sb)));
                                simplify(&Expr::BinaryOp(
                                    BinaryOp::Add,
                                    Box::new(sa),
                                    Box::new(sb_neg),
                                ))
                            }
                        }
                    }
                }

                BinaryOp::Div => match (&sa, &sb) {
                    (Expr::Number(n1), Expr::Number(n2)) => Expr::Number(OrderedFloat(n1.0 / n2.0)),
                    _ => {
                        if sb == Expr::Number(OrderedFloat(1.0)) {
                            sa
                        } else if sa == Expr::Number(OrderedFloat(0.0)) {
                            Expr::Number(OrderedFloat(0.0))
                        } else {
                            Expr::BinaryOp(BinaryOp::Div, Box::new(sa), Box::new(sb))
                        }
                    }
                },

                BinaryOp::Pow => match &sb {
                    Expr::Number(n) if *n == OrderedFloat(0.0) => Expr::Number(OrderedFloat(1.0)),
                    Expr::Number(n) if *n == OrderedFloat(1.0) => sa,
                    _ => Expr::BinaryOp(BinaryOp::Pow, Box::new(sa), Box::new(sb)),
                },
            }
        }

        Expr::Function(name, arg) => {
            let sarg = simplify(arg);
            if let Expr::Number(n) = &sarg {
                let val = match name.as_str() {
                    "sin" => n.0.sin(),
                    "cos" => n.0.cos(),
                    "tan" => n.0.tan(),
                    "log" => {
                        if n.0 > 0.0 {
                            n.0.ln()
                        } else {
                            return Expr::Function(name.clone(), Box::new(sarg));
                        }
                    }
                    "exp" => n.0.exp(),
                    "abs" => n.0.abs(),
                    _ => return Expr::Function(name.clone(), Box::new(sarg)),
                };
                Expr::Number(OrderedFloat(val))
            } else {
                Expr::Function(name.clone(), Box::new(sarg))
            }
        }
    }
}

// rest unchanged

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

fn partition_consts(terms: Vec<Expr>) -> (OrderedFloat<f64>, Vec<Expr>) {
    let mut sum_consts = OrderedFloat(0.0);
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

fn partition_consts_mul(factors: Vec<Expr>) -> (OrderedFloat<f64>, Vec<Expr>) {
    let mut prod_consts = OrderedFloat(1.0);
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

fn normalize_double_neg(expr: &Expr) -> Expr {
    match expr {
        Expr::UnaryOp(UnaryOp::Neg, inner) => {
            if let Expr::UnaryOp(UnaryOp::Neg, inner_inner) = &**inner {
                normalize_double_neg(inner_inner)
            } else {
                Expr::UnaryOp(UnaryOp::Neg, Box::new(normalize_double_neg(inner)))
            }
        }
        Expr::BinaryOp(op, a, b) => Expr::BinaryOp(
            op.clone(),
            Box::new(normalize_double_neg(a)),
            Box::new(normalize_double_neg(b)),
        ),
        _ => expr.clone(),
    }
}

fn extract_coefficient(expr: &Expr) -> Option<(OrderedFloat<f64>, Expr)> {
    let expr = normalize_double_neg(expr);
    match expr {
        Expr::Number(n) => Some((n, Expr::Number(OrderedFloat(1.0)))),

        Expr::UnaryOp(UnaryOp::Neg, inner) => {
            extract_coefficient(&inner).map(|(coef, base)| (-coef, base))
        }

        Expr::BinaryOp(BinaryOp::Mul, left, right) => {
            let left_coef = extract_coefficient(&left);
            let right_coef = extract_coefficient(&right);

            match (left_coef, right_coef) {
                (Some((lc, lb)), Some((rc, rb))) => {
                    let combined_coef = lc * rc;
                    let combined_base = if lb == Expr::Number(OrderedFloat(1.0)) {
                        rb
                    } else if rb == Expr::Number(OrderedFloat(1.0)) {
                        lb
                    } else {
                        Expr::BinaryOp(BinaryOp::Mul, Box::new(lb), Box::new(rb))
                    };
                    Some((combined_coef, combined_base))
                }
                (Some((lc, lb)), None) => {
                    let combined_base = if lb == Expr::Number(OrderedFloat(1.0)) {
                        (*right).clone()
                    } else {
                        Expr::BinaryOp(BinaryOp::Mul, Box::new(lb), Box::new((*right).clone()))
                    };
                    Some((lc, combined_base))
                }
                (None, Some((rc, rb))) => {
                    let combined_base = if rb == Expr::Number(OrderedFloat(1.0)) {
                        (*left).clone()
                    } else {
                        Expr::BinaryOp(BinaryOp::Mul, Box::new((*left).clone()), Box::new(rb))
                    };
                    Some((rc, combined_base))
                }
                (None, None) => None,
            }
        }

        _ => None,
    }
}

fn combine_like_terms(terms: Vec<Expr>) -> Vec<Expr> {
    use std::collections::HashMap;

    let mut counts: HashMap<Expr, OrderedFloat<f64>> = HashMap::new();
    let mut order: Vec<Expr> = Vec::new();

    for term in &terms {
        if let Some((coef, base)) = extract_coefficient(term) {
            if !counts.contains_key(&base) {
                order.push(base.clone());
            }
            *counts.entry(base).or_insert(OrderedFloat(0.0)) += coef;
        } else {
            if !counts.contains_key(term) {
                order.push(term.clone());
            }
            *counts.entry(term.clone()).or_insert(OrderedFloat(0.0)) += OrderedFloat(1.0);
        }
    }

    let mut combined = Vec::new();
    let mut const_terms = Vec::new();

    for base in order {
        let coef = counts.get(&base).unwrap();
        if coef.0.abs() < 1e-12 {
            continue;
        }
        if base == Expr::Number(OrderedFloat(1.0)) {
            const_terms.push(Expr::Number(*coef));
        } else if *coef == OrderedFloat(1.0) {
            combined.push(base);
        } else {
            combined.push(Expr::BinaryOp(
                BinaryOp::Mul,
                Box::new(Expr::Number(*coef)),
                Box::new(base),
            ));
        }
    }

    combined.append(&mut const_terms);
    combined
}
