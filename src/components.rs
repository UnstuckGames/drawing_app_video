mod drawing;

use crate::drawing::*;

use dioxus::prelude::*;
use dioxus_elements::geometry::ElementPoint;

#[allow(non_snake_case)]
fn ToolBar() -> Element {
    // SIGNALS
    let mut undo_commands = use_context::<AppState>().undo_commands;
    let mut redo_commands = use_context::<AppState>().redo_commands;

    // HANDLERS
    let mut erase_handler = move || {
        clear_canvas(DRAWING_CANVAS_ID);
        if undo_commands().last().unwrap() != &Command::EraseCanvas {
            undo_commands.push(Command::EraseCanvas)
        };
        redo_commands.set(vec![])
    };

    let mut undo_handler = move || {
        if undo_commands.is_empty() {
        } else {
            clear_canvas(DRAWING_CANVAS_ID);
            redo_commands.push(undo_commands.pop().unwrap());
        }

        draw_paths(undo_commands());
    };

    let mut redo_handler = move || {
        if redo_commands.is_empty() {
        } else {
            clear_canvas(DRAWING_CANVAS_ID);
            undo_commands.push(redo_commands.pop().unwrap())
        }

        draw_paths(undo_commands());
    };

    rsx! {
        div { // TOOLBAR
            button { // ERASE
                onclick: move |_event| erase_handler(),
                "Erase canvas"
            }
            button { // UNDO
                onclick: move |_event| undo_handler(),
                "Undo"
            }
            button { // REDO
                onclick: move |_event| redo_handler(),
                "Redo"
            }
            button { // SAVE
                onclick: move |_event| save_canvas(),
                "Save"
            }
        }
    }
}

#[allow(non_snake_case)]
fn DrawingCanvas() -> Element {
    // SIGNALS
    let mut current_point = use_context::<AppState>().current_point;
    let mut tool_mode = use_context::<AppState>().tool_mode;
    let mut undo_commands = use_context::<AppState>().undo_commands;
    let mut redo_commands = use_context::<AppState>().redo_commands;
    let mut current_path = use_context::<AppState>().current_path;

    let rgb_color = use_context::<AppState>().rgb_color;

    // HANDLERS
    let mut end_draw_handler = move || match tool_mode() {
        ToolMode::Drawing => {
            tool_mode.set(ToolMode::Idle);
            undo_commands.push(Command::DrawPath(Path {
                points: current_path(),
                color: rgb_color(),
            }));
            current_path.set(vec![]);
        }
        ToolMode::Idle => {}
    };
    let mut begin_draw_handler = move |event: Event<MouseData>| {
        current_point.set(event.element_coordinates());
        tool_mode.set(ToolMode::Drawing);

        redo_commands.set(vec![]);
        current_path.push(current_point());

        begin_path();
        draw_point(current_point(), rgb_color());
    };

    let mut continue_draw_handler = move |event: Event<MouseData>| match tool_mode() {
        ToolMode::Drawing => {
            current_point.set(event.element_coordinates());
            current_path.push(current_point());
            draw_point(current_point(), rgb_color())
        }
        _ => {}
    };

    rsx! {

        div {
            canvas { // DRAWING CANVAS

                id: DRAWING_CANVAS_ID,
                width: 1000,
                height: 500,
                border: "1px solid black",

                onmousedown: move |event| begin_draw_handler(event),
                onmousemove: move |event| continue_draw_handler(event),
                onmouseleave: move |_event| end_draw_handler(),
                onmouseup: move |_event| end_draw_handler(),
            }
        }
    }
}

#[allow(non_snake_case)]
fn ColorPicker() -> Element {
    let cp_width = 150.0;
    let cp_height = 150.0;

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
        rgb_color.set(read_color(sat_val_coord()));

        clear_canvas(SAT_VAL_CANVAS_ID);
        draw_color_picker(hue());

        draw_sat_val_pointer(sat_val_coord());
    };
    let mut hue_click_handler = move |event: Event<MouseData>| {
        hue.set((event.element_coordinates().x / (cp_width as f64)) * 360.0);
        draw_hue_pointer(hue());
        draw_color_picker(hue());

        draw_sat_val_pointer(sat_val_coord());
        rgb_color.set(read_color(sat_val_coord()));
    };

    rsx! {
        div { // COLOR PICKER
            canvas{ // SAT VAL PICKER
                id: SAT_VAL_CANVAS_ID,
                width: cp_width,
                height: cp_height,
                onmounted: move |_event| {
                    draw_color_picker(hue());
                    draw_sat_val_pointer(sat_val_coord());
                },

                onclick: move |event| sat_val_click_handler(event),            }
        }

        div {
            canvas { // HUE BAR
                id: HUE_BAR_CANVAS_ID,
                width: cp_width,
                height: 20,
                onmounted: move |_event| draw_hue_bar(hue()),

                onclick: move |event| hue_click_handler(event),

            }
        }
    }
}
