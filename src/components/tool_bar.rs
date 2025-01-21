use crate::app_state::AppState;
use crate::drawing::*;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::*;
use dioxus_free_icons::Icon;

#[allow(non_snake_case)]
pub fn ToolBar() -> Element {
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
