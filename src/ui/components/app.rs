use crate::ui::components::{graph::Graph, input::ExpressionInput, output::Output};
use yew::prelude::*;

#[function_component(CliphApp)]
pub fn cliph_app() -> Html {
    let expr = use_state(|| "x^2".to_string());

    html! {
        <div class="container">
            <h1>{ "Cliph – Graphing Calculator" }</h1>
            <ExpressionInput expr={expr.clone()} />
            <Output expr={(*expr).clone()} />
            <Graph expr={(*expr).clone()} />
        </div>
    }
}
