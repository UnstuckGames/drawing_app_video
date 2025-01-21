use crate::app_state::AppState;
use crate::drawing::*;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::*;
use dioxus_free_icons::Icon;

#[allow(non_snake_case)]
pub fn MenuBar() -> Element {
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
