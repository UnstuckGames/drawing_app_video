// draw_canvas

use crate::app_state::AppState;
use crate::drawing::*;
use dioxus::prelude::*;
use enum_map::enum_map;
use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
pub fn DrawCanvas() -> Element {
    // SETTING UP PROPERTIES
    let drawing_canvas = use_context::<AppState>().drawing_canvas;
    let drawing_canvas = drawing_canvas();

    let preview_canvas = DrawingCanvas {
        properties: CanvasProperties {
            name: "preview-canvas",
            width: drawing_canvas.properties.width,
            height: drawing_canvas.properties.height,
        },
    };

    // SIGNALS
    let mut current_point = use_context::<AppState>().current_point;
    let tool_mode = use_context::<AppState>().tool_mode;
    let mut tool_active = use_context::<AppState>().tool_active;
    let mut undo_commands = use_context::<AppState>().undo_commands;
    let mut redo_commands = use_context::<AppState>().redo_commands;
    let mut current_path = use_context::<AppState>().current_path;
    let rgb_color = use_context::<AppState>().rgb_color;
    let canvas_cursor = use_context::<AppState>().canvas_cursor;

    let mut point_down = use_context::<AppState>().point_down;

    let line_width = use_context::<AppState>().line_width;

    let mut shape_properties = use_context::<AppState>().shape_properties;

    // HANDLERS
    let tool_handlers = enum_map![
        ToolMode::Pen => CanvasToolHandler {
            // PEN
            onmousedown: Arc::new(Mutex::new(move || {
                current_path.push(current_point());
                drawing_canvas.begin_path();
                drawing_canvas.draw_point(&current_point(), rgb_color(), line_width());
            })),
            onmousemove: Arc::new(Mutex::new(move || {
                current_path.push(current_point());
                drawing_canvas.draw_point(&current_point(), rgb_color(), line_width());
            })),
            onmouseup: Arc::new(Mutex::new(move || {
                undo_commands.push(Command::DrawPath(Path {
                    points: current_path(),
                    color: rgb_color(),
                    line_width: line_width(),
                }));
                current_path.set(vec![]);
            })),
        },

        ToolMode::Eraser => CanvasToolHandler {
            //ERASER
            onmousedown: Arc::new(Mutex::new(move || {
                current_path.push(current_point());
                drawing_canvas.begin_path();
                drawing_canvas.erase(current_point(), line_width());
            })),
            onmousemove: Arc::new(Mutex::new(move || {
                current_path.push(current_point());
                drawing_canvas.erase(current_point(), line_width());
            })),
            onmouseup: Arc::new(Mutex::new(move || {
                undo_commands.push(Command::ErasePath(Path {
                    points: current_path(),
                    color: Color(0.0, 0.0, 0.0),
                    line_width: line_width(),
                }));
                current_path.set(vec![]);
            })),
        },

        ToolMode::Line => CanvasToolHandler {
            onmousedown: Arc::new(Mutex::new(move || {
                point_down.set(current_point());
            })),
            onmousemove: Arc::new(Mutex::new(move || {
                preview_canvas.clear_canvas();
                preview_canvas.draw_line(&point_down(), &current_point(), &rgb_color(), line_width());
            })),
            onmouseup: Arc::new(Mutex::new(move || {
                    preview_canvas.clear_canvas();

                    shape_properties.set(ShapeProperties {
                        start_point: point_down(),
                        end_point: current_point(),
                        color: rgb_color(),
                        line_width: line_width(),
                    });
                    undo_commands.push(Command::DrawLine(
                            shape_properties()
                    ));
                    drawing_canvas.draw_line(
                        &point_down(),
                        &current_point(),
                        &rgb_color(),
                        line_width(),
                    );

            })),
        },
        ToolMode::Rectangle => CanvasToolHandler {
            onmousedown: Arc::new(Mutex::new(move || {
                point_down.set(current_point());
            })),
            onmousemove: Arc::new(Mutex::new(move || {
                preview_canvas.clear_canvas();
                preview_canvas.draw_rect(&point_down(), &current_point(), &rgb_color(), line_width());
            })),
            onmouseup: Arc::new(Mutex::new(move || {
                    preview_canvas.clear_canvas();

                    shape_properties.set(ShapeProperties {
                        start_point: point_down(),
                        end_point: current_point(),
                        color: rgb_color(),
                        line_width: line_width(),
                    });
                    undo_commands.push(Command::DrawRectangle(
                            shape_properties()
                    ));
                    drawing_canvas.draw_rect(
                        &point_down(),
                        &current_point(),
                        &rgb_color(),
                        line_width(),
                    );

            })),
        },
        ToolMode::Circle => CanvasToolHandler {
            onmousedown: Arc::new(Mutex::new(move || {
                point_down.set(current_point());
            })),
            onmousemove: Arc::new(Mutex::new(move || {
                preview_canvas.clear_canvas();
                preview_canvas.draw_circle(&point_down(), &current_point(), &rgb_color(), line_width());
            })),
            onmouseup: Arc::new(Mutex::new(move || {
                    preview_canvas.clear_canvas();

                    shape_properties.set(ShapeProperties {
                        start_point: point_down(),
                        end_point: current_point(),
                        color: rgb_color(),
                        line_width: line_width(),
                    });
                    undo_commands.push(Command::DrawCircle(
                            shape_properties()
                    ));
                    drawing_canvas.draw_circle(
                        &point_down(),
                        &current_point(),
                        &rgb_color(),
                        line_width(),
                    );

            })),
        },

        _ => CanvasToolHandler {
            onmousedown: Arc::new(Mutex::new(move || {})),
            onmousemove: Arc::new(Mutex::new(move || {})),
            onmouseup: Arc::new(Mutex::new(move || {})),
        }
    ];

    let CanvasToolHandler {
        onmousedown,
        onmousemove,
        onmouseup,
    } = tool_handlers[tool_mode()].clone();

    let mouse_down_handler = move |event: Event<MouseData>| {
        tool_active.set(true);

        current_point.set(event.element_coordinates());
        redo_commands.set(vec![]);

        let mut handle = onmousedown.lock().unwrap();
        handle();
    };

    let mouse_move_handler = move |event: Event<MouseData>| {
        if tool_active() {
            current_point.set(event.element_coordinates());

            let mut handle = onmousemove.lock().unwrap();
            handle();
        };
    };

    let mouse_up_handler = move || {
        if tool_active() {
            tool_active.set(false);

            let mut handle = onmouseup.lock().unwrap();
            handle();
        }
    };

    rsx! {

        div {
            canvas { // DRAWING CANVAS

                id: drawing_canvas.properties.name,
                width: drawing_canvas.properties.width,
                height: drawing_canvas.properties.height,
                // border: "2px solid black",
                z_index: 0,
                position: "absolute",
                cursor: canvas_cursor(),
                background: "white",
                class: "drawing-canvas",

                onmousedown: move |event| mouse_down_handler.clone()(event),
                onmousemove: move |event| mouse_move_handler.clone()(event),

                // onmouseleave: move |_event| mouse_up_handler.clone()(), //leave and up have the same
                onmouseup: move |_event| mouse_up_handler.clone()(),
            }

            canvas {
                id: preview_canvas.properties.name,
                width: preview_canvas.properties.width,
                height: preview_canvas.properties.height,
                // border: "2px solid black",
                z_index: 1,
                position: "relative",
                pointer_events: "none",
                class: "drawing-canvas",
            }
        }
    }
}
