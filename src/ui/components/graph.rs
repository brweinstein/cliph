use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::math::{eval, parser};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub expr: String,
}

#[function_component(Graph)]
pub fn graph(props: &Props) -> Html {
    let canvas_ref = use_node_ref();
    let expr_str = props.expr.clone();

    use_effect_with((canvas_ref.clone(), expr_str.clone()), move |(canvas_ref, expr_str)| {
        let cleanup = Box::new(|| ()) as Box<dyn Fn()>; // Cleanup no-op closure

        let canvas = match canvas_ref.cast::<HtmlCanvasElement>() {
            Some(c) => c,
            None => return cleanup,
        };

        let backend = match CanvasBackend::with_canvas_object(canvas) {
            Some(b) => b,
            None => return cleanup,
        };

        let root = backend.into_drawing_area();
        if root.fill(&WHITE).is_err() {
            return cleanup;
        }

        let parsed = match parser::parse(&expr_str) {
            Ok(e) => e,
            Err(_) => return cleanup,
        };

        let vars = |x: f64| {
            let mut env = std::collections::HashMap::new();
            env.insert("x".to_string(), x);
            eval::evaluate_with_env(&parsed, &env)
        };

        let x_range = -10.0..10.0;
        let y_range = -10.0..10.0;

        let mut chart = match ChartBuilder::on(&root)
            .margin(10)
            .caption("f(x)", ("sans-serif", 20))
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_range.clone(), y_range.clone())
        {
            Ok(c) => c,
            Err(_) => return cleanup,
        };

        if chart.configure_mesh().draw().is_err() {
            return cleanup;
        }

        // Draw axes explicitly
        if chart
            .draw_series([
                PathElement::new(vec![(-10.0, 0.0), (10.0, 0.0)], &BLACK),
                PathElement::new(vec![(0.0, -10.0), (0.0, 10.0)], &BLACK),
            ])
            .is_err()
        {
            return cleanup;
        }

        // Generate points on the curve
        let points: Vec<(f64, f64)> = (-100..=100)
            .map(|i| {
                let x = i as f64 / 10.0;
                (x, vars(x))
            })
            .collect();

        // Filter points inside visible y range into segments
        let is_in_range = |y: f64| y_range.start <= y && y <= y_range.end;
        let mut segments = Vec::new();
        let mut current_segment = Vec::new();

        for p in points {
            if is_in_range(p.1) {
                current_segment.push(p);
            } else {
                if current_segment.len() > 1 {
                    segments.push(current_segment);
                }
                current_segment = Vec::new();
            }
        }
        if current_segment.len() > 1 {
            segments.push(current_segment);
        }

        // Draw each segment separately
        for segment in segments {
            if chart.draw_series(LineSeries::new(segment.clone(), &RED)).is_err() {
                return cleanup;
            }
        }

        cleanup
    });

    html! {
        <canvas width=600 height=400 ref={canvas_ref}></canvas>
    }
}
