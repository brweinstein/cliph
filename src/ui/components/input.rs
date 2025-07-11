use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub expr: UseStateHandle<String>,
}

#[function_component(ExpressionInput)]
pub fn expression_input(props: &Props) -> Html {
    let expr = props.expr.clone();

    let oninput = {
        let expr = expr.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                expr.set(input.value());
            }
        })
    };

    html! {
        <input
            type="text"
            value={(*expr).clone()}
            {oninput}
            placeholder="Enter expression (e.g., x^2 + 3*x)"
        />
    }
}
