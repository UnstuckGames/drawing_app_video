use crate::drawing::{Color, Command, DrawingCanvas, ShapeProperties, ToolMode};
use dioxus::prelude::*;
use dioxus_elements::geometry::ElementPoint;

#[derive(Clone, Debug)]
pub struct AppState {
    pub current_point: Signal<ElementPoint>,
    pub tool_mode: Signal<ToolMode>,
    pub tool_active: Signal<bool>,

    pub undo_commands: Signal<Vec<Command>>,
    pub redo_commands: Signal<Vec<Command>>,
    pub current_path: Signal<Vec<ElementPoint>>,

    pub rgb_color: Signal<Color>,
    pub hue: Signal<f64>,
    pub sat_val_coord: Signal<(f64, f64)>,

    pub line_width: Signal<f64>,

    pub canvas_cursor: Signal<String>,
    pub point_down: Signal<ElementPoint>,

    pub drawing_canvas: Signal<DrawingCanvas>,

    pub shape_properties: Signal<ShapeProperties>,
}
