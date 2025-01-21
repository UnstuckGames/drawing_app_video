use crate::AppState;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn LineWidthSlider() -> Element {
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
