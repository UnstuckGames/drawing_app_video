// main.rs

mod app_state;
mod drawing;
use crate::app_state::AppState;

use crate::drawing::*;

mod components;
use components::{ColorPicker, DrawCanvas, LineWidthSlider, MenuBar, ToolBar};
#[allow(unused_imports)]
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_elements::geometry::ElementPoint;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    // INITIALIZE STATE
    let _app_state = use_context_provider(|| AppState {
        current_point: Signal::new(ElementPoint::zero()),
        tool_mode: Signal::new(ToolMode::Pen),
        tool_active: Signal::new(false),

        undo_commands: Signal::new(vec![]),
        redo_commands: Signal::new(vec![]),
        current_path: Signal::new(vec![]),

        rgb_color: Signal::new(Color(0.0, 0.0, 0.0)),
        hue: Signal::new(0.0),
        sat_val_coord: Signal::new((0.0, 0.0)),

        line_width: Signal::new(1.0),

        canvas_cursor: Signal::new("default".to_string()),
        point_down: Signal::new(ElementPoint::zero()),

        drawing_canvas: Signal::new(DrawingCanvas {
            properties: CanvasProperties {
                name: "drawing-canvas",
                width: 1500.0,
                height: 800.0,
            },
        }),
        shape_properties: Signal::new(ShapeProperties {
            start_point: ElementPoint::zero(),
            end_point: ElementPoint::zero(),
            color: Color(0.0, 0.0, 0.0),
            line_width: 1.0,
        }),
    });
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div{
            class: "main-div",
            div {
                class: "tool-div",
                h1 {
                    "Lets Draw!"
                }
                MenuBar {}
                ToolBar {}
                LineWidthSlider {}
                ColorPicker {}
            }

            DrawCanvas {}
        }
    }
}
