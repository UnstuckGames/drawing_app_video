use crate::app_state::AppState;
use crate::drawing::*;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn ColorPicker() -> Element {
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
