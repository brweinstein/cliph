use crate::math::algebra::simplify;
use crate::math::diff::differentiate;
use crate::math::format::format_expr_latex;
use crate::math::parser::{latex_to_math_expr, parse};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = renderMathInElementHelper)]
    fn render_math_in_element(elem: Element);
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub expr: String,
}

#[function_component(Output)]
pub fn output(props: &Props) -> Html {
    let node_ref_simplified = use_node_ref();
    let node_ref_derivative = use_node_ref();

    let (simplified_expr_latex, derivative_latex) = {
        let expr_str = latex_to_math_expr(&props.expr);

        match parse(&expr_str) {
            Ok(expr) => {
                let simplified = simplify(&expr);
                let simplified_latex = format!("${}$", format_expr_latex(&simplified));

                let deriv = differentiate(&simplified, "x");
                let derivative_latex = format!("${}$", format_expr_latex(&deriv));

                (simplified_latex, derivative_latex)
            }
            Err(_) => (
                "Error parsing expression".to_string(),
                "Error parsing expression".to_string(),
            ),
        }
    };

    // Use effect with setTimeout to defer renderMathInElement call safely
    {
        let node_ref_simplified = node_ref_simplified.clone();
        let simplified_expr_latex = simplified_expr_latex.clone();
        let expr = props.expr.clone();

        use_effect_with((expr, simplified_expr_latex), move |_| {
            if let Some(elem) = node_ref_simplified.cast::<Element>() {
                let elem_clone = elem.clone();
                let closure = Closure::once(move || {
                    render_math_in_element(elem_clone);
                });
                window()
                    .unwrap()
                    .set_timeout_with_callback(closure.as_ref().unchecked_ref())
                    .expect("failed to set timeout");
                closure.forget();
            }
            || ()
        });
    }

    {
        let node_ref_derivative = node_ref_derivative.clone();
        let derivative_latex = derivative_latex.clone();
        let expr = props.expr.clone();

        use_effect_with((expr, derivative_latex), move |_| {
            if let Some(elem) = node_ref_derivative.cast::<Element>() {
                let elem_clone = elem.clone();
                let closure = Closure::once(move || {
                    render_math_in_element(elem_clone);
                });
                window()
                    .unwrap()
                    .set_timeout_with_callback(closure.as_ref().unchecked_ref())
                    .expect("failed to set timeout");
                closure.forget();
            }
            || ()
        });
    }

    html! {
        <>
            <p>{ "Simplified expression:" }</p>
            <div ref={node_ref_simplified}>
                <code>{ simplified_expr_latex }</code>
            </div>

            <p>{ "Derivative w.r.t x:" }</p>
            <div ref={node_ref_derivative}>
                <code>{ derivative_latex }</code>
            </div>
        </>
    }
}
