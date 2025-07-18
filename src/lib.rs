pub mod math;
pub mod ui;

pub use math::algebra::simplify;
pub use math::ast::{BinaryOp, Expr, UnaryOp};

// Only compile this on wasm32 for Yew
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<ui::components::app::CliphApp>::new().render();
}
