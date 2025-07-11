pub mod math;
pub mod ui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<ui::components::app::CliphApp>::new().render();
}
