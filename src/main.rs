// main.rs

mod drawing;
use crate::drawing::*;
#[allow(unused_imports)]
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_elements::geometry::ElementPoint;
use dioxus_free_icons::icons::ld_icons::*;
use dioxus_free_icons::Icon;
use enum_map::enum_map;
use std::sync::{Arc, Mutex};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}
//
// fn main() {
//     dioxus::LaunchBuilder::new()
//         .with_cfg(server_only!(ServeConfig::builder().incremental(
//             IncrementalRendererConfig::default()
//                 .invalidate_after(std::time::Duration::from_secs(120)),
//         )))
//         .launch(app);
// }

#[derive(Clone, Debug)]
struct AppState {
    current_point: Signal<ElementPoint>,
    tool_mode: Signal<ToolMode>,
    tool_active: Signal<bool>,

    undo_commands: Signal<Vec<Command>>,
    redo_commands: Signal<Vec<Command>>,
    current_path: Signal<Vec<ElementPoint>>,

    rgb_color: Signal<Color>,
    hue: Signal<f64>,
    sat_val_coord: Signal<(f64, f64)>,

    line_width: Signal<f64>,

    canvas_cursor: Signal<String>,
    point_down: Signal<ElementPoint>,

    drawing_canvas: Signal<DrawingCanvas>,

    shape_properties: Signal<ShapeProperties>,
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
#[allow(non_snake_case)]
fn MenuBar() -> Element {
    // PROPERTIES
    let drawing_canvas = use_context::<AppState>().drawing_canvas;
    let drawing_canvas = drawing_canvas();

    // SIGNALS
    let mut undo_commands = use_context::<AppState>().undo_commands;
    let mut redo_commands = use_context::<AppState>().redo_commands;

    // HANDLERS
    let mut clear_canvas_handler = move || {
        drawing_canvas.clear_canvas();
        if undo_commands().last().unwrap() != &Command::EraseCanvas {
            undo_commands.push(Command::EraseCanvas)
        };
        redo_commands.set(vec![])
    };

    let mut undo_handler = move || {
        if undo_commands.is_empty() {
        } else {
            drawing_canvas.clear_canvas();
            redo_commands.push(undo_commands.pop().unwrap());
        }

        drawing_canvas.execute_commands(undo_commands());
    };

    let mut redo_handler = move || {
        if redo_commands.is_empty() {
        } else {
            drawing_canvas.clear_canvas();
            undo_commands.push(redo_commands.pop().unwrap())
        }

        drawing_canvas.execute_commands(undo_commands());
    };

    rsx! {
        div { // MENUBAR
            button { // CLEAR CANVAS
                onclick: move |_event| clear_canvas_handler(),
                Icon {
                    icon: LdTrash,
                }
            }
            button { // UNDO
                onclick: move |_event| undo_handler(),
                Icon {
                    icon: LdUndo,
                }
            }
            button { // REDO
                onclick: move |_event| redo_handler(),
                Icon {
                    icon: LdRedo,
                }
            }
            button { // SAVE
                onclick: move |_event| drawing_canvas.save_canvas(),
                Icon {
                    icon: LdSave,
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn ToolBar() -> Element {
    let mut tool_mode = use_context::<AppState>().tool_mode;
    let mut canvas_cursor = use_context::<AppState>().canvas_cursor;

    rsx! {
        div{
            button { // Pen
                onclick: move |_event| {
                    tool_mode.set(ToolMode::Pen);
                    canvas_cursor.set("crosshair".to_string())
                    },
                Icon {
                    icon: LdPencil,
                }
            }
            button {
                onclick: move |_event| tool_mode.set(ToolMode::Eraser),
                Icon {
                    icon: LdEraser,
                }
            }
            button {// Line
                onclick: move |_event| tool_mode.set(ToolMode::Line),
                Icon {
                    icon: LdMinus,
                }
            }
            button {// Circle
                onclick: move |_event| tool_mode.set(ToolMode::Circle),
                Icon {
                    icon: LdCircle,
                }
            }
            button {// Rectangle
                onclick: move |_event| tool_mode.set(ToolMode::Rectangle),
                Icon {
                    icon: LdRectangleHorizontal,
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn DrawCanvas() -> Element {
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
                drawing_canvas.erase(current_point());
            })),
            onmousemove: Arc::new(Mutex::new(move || {
                current_path.push(current_point());
                drawing_canvas.erase(current_point());
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

                // onmouseleave: move |_event| mouse_up_handler(), //leave and up have the same
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

#[allow(non_snake_case)]
fn ColorPicker() -> Element {
    // PROPERTIES
    let sat_val_canvas = SatValCanvas {
        properties: CanvasProperties {
            name: "sat-val-canvas",
            width: 150.0,
            height: 150.0,
        },
    };

    let hue_canvas = HueCanvas {
        properties: CanvasProperties {
            name: "hue-canvas",
            width: sat_val_canvas.properties.width,
            height: 20.0,
        },
    };

    // SIGNALS
    let mut rgb_color = use_context::<AppState>().rgb_color;
    let mut hue = use_context::<AppState>().hue;
    let mut sat_val_coord = use_context::<AppState>().sat_val_coord;

    // HANDLERS
    let mut sat_val_click_handler = move |event: Event<MouseData>| {
        sat_val_coord.set((
            event.element_coordinates().x as f64,
            event.element_coordinates().y as f64,
        ));
        rgb_color.set(sat_val_canvas.read_color(sat_val_coord()));

        sat_val_canvas.clear_canvas();
        sat_val_canvas.draw_color_picker(hue());

        sat_val_canvas.draw_sat_val_pointer(sat_val_coord());
    };
    let mut hue_click_handler = move |event: Event<MouseData>| {
        hue.set((event.element_coordinates().x / (sat_val_canvas.properties.width as f64)) * 360.0);
        hue_canvas.draw_hue_pointer(hue());
        sat_val_canvas.draw_color_picker(hue());

        sat_val_canvas.draw_sat_val_pointer(sat_val_coord());
        rgb_color.set(sat_val_canvas.read_color(sat_val_coord()));
    };

    rsx! {
        div { // COLOR PICKER
            position: "relative",
            canvas{ // SAT VAL PICKER
                id: sat_val_canvas.properties.name,
                width: sat_val_canvas.properties.width,
                height: sat_val_canvas.properties.height,
                onmounted: move |_event| {
                    sat_val_canvas.draw_color_picker(hue());
                    sat_val_canvas.draw_sat_val_pointer(sat_val_coord());
                },

                onclick: move |event| sat_val_click_handler(event),            }
        }

        div {
            canvas { // HUE BAR
                id: hue_canvas.properties.name,
                width: hue_canvas.properties.width,
                height: hue_canvas.properties.height,
                onmounted: move |_event| hue_canvas.draw_hue_bar(hue()),

                onclick: move |event| hue_click_handler(event),

            }
        }
    }
}
#[allow(non_snake_case)]
fn LineWidthSlider() -> Element {
    let mut line_width = use_context::<AppState>().line_width;

    rsx! {
        div {
            class: "stroke-div",
            div{
                class: "stroke-text",
                "Stroke width: {line_width()} px"
            }
            input {
                type: "range",
                min: 1.0,
                max: 10.0,
                value: line_width(),
                oninput: move |event| {
                    line_width.set(event.value().parse::<f64>().unwrap());
                },
            }
        }
    }
}
