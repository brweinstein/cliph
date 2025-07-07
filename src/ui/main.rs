use cliph::core::{parser, eval, algebra};

fn main() {
    let expr_str = "x^2 + 2*x + 1";
    let x_val = 3.0;

    // Parse the expression
    let parsed = match parser::parse(expr_str) {
        Ok(ast) => {
            println!("Parsed AST: {:?}", ast);
            ast
        }
        Err(e) => {
            eprintln!("Parse error: {e}");
            return;
        }
    };

    // Evaluate the expression at x = 3
    let mut vars = std::collections::HashMap::new();
    vars.insert("x".to_string(), x_val);

    let value = eval::evaluate_with_env(&parsed, &vars);
    println!("f({x_val}) = {value}");

    // Simplify the expression (basic rules)
    let simplified = algebra::simplify(&parsed);
    println!("Simplified: {:?}", simplified);
}
