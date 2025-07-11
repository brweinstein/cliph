use yew::prelude::*;
use crate::math::parser::latex_to_math_expr;
use crate::math::parser::parse;
use crate::math::diff::differentiate;
use crate::math::format::format_expr_latex;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub expr: String,
}

#[function_component(Output)]
pub fn output(props: &Props) -> Html {
    let expr_str = latex_to_math_expr(&props.expr);

    let expr = parse(&expr_str);
    let derivative = expr
        .as_ref()
        .map(|e| differentiate(e, "x"))
        .map(|e| format_expr_latex(&e))
        .unwrap_or_else(|_| "Error parsing expression".to_string());

    html! {
        <>
            <p>{ "Derivative w.r.t x:" }</p>
            <p><code>{ derivative }</code></p>
        </>
    }
}
