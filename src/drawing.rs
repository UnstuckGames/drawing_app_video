// drawing.rs

use derive_getters::Dissolve;
#[allow(unused_imports)]
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_elements::geometry::ElementPoint;
use hsv;
use std::sync::{Arc, Mutex};

use enum_map::Enum;

use std::f64::consts::PI;
use web_sys::{
    wasm_bindgen::JsCast, CanvasRenderingContext2d, HtmlAnchorElement, HtmlCanvasElement,
};

// PUBLIC
#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    pub points: Vec<ElementPoint>,
    pub color: Color,
    pub line_width: f64,
}

#[derive(Clone, Debug, Dissolve, Copy, PartialEq)]
pub struct Color(pub f64, pub f64, pub f64);

#[derive(Debug, Clone, Copy, Enum)]
pub enum ToolMode {
    Pen,
    Eraser,
    Line,
    Circle,
    Rectangle,
    Polygon,
}

type Handler = Arc<Mutex<dyn FnMut() -> ()>>;
#[derive(Clone)]
pub struct CanvasToolHandler {
    pub onmousedown: Handler,
    pub onmousemove: Handler,
    pub onmouseup: Handler,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapeProperties {
    pub start_point: ElementPoint,
    pub end_point: ElementPoint,
    pub color: Color,
    pub line_width: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    DrawPath(Path),
    ErasePath(Path),
    DrawLine(ShapeProperties),
    DrawCircle(ShapeProperties),
    DrawRectangle(ShapeProperties),
    EraseCanvas,
}

impl Command {
    pub fn execute(&self, drawing_canvas: &DrawingCanvas) {
        match self {
            Self::DrawPath(path) => {
                drawing_canvas.begin_path();
                path.points.iter().for_each(|point| {
                    drawing_canvas.draw_point(point, path.color, path.line_width)
                });
            }
            Self::DrawRectangle(shape_properties) => {
                drawing_canvas.draw_rect(
                    &shape_properties.start_point,
                    &shape_properties.end_point,
                    &shape_properties.color,
                    shape_properties.line_width,
                );
            }
            Self::DrawCircle(shape_properties) => {
                drawing_canvas.draw_circle(
                    &shape_properties.start_point,
                    &shape_properties.end_point,
                    &shape_properties.color,
                    shape_properties.line_width,
                );
            }

            Self::DrawLine(shape_properties) => {
                drawing_canvas.draw_line(
                    &shape_properties.start_point,
                    &shape_properties.end_point,
                    &shape_properties.color,
                    shape_properties.line_width,
                );
            }
            Self::ErasePath(path) => {
                path.points
                    .iter()
                    .for_each(|point| drawing_canvas.erase(*point, path.line_width));
            }

            Self::EraseCanvas => drawing_canvas.clear_canvas(),
            // _ => {} // good placeholder
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct CanvasProperties {
    pub name: &'static str,
    pub width: f64,
    pub height: f64,
}

pub trait Canvas {
    // to implement
    fn properties(&self) -> &CanvasProperties;

    // shared methods
    fn get_canvas(&self) -> HtmlCanvasElement {
        let document = get_document();
        let canvas = document
            .get_element_by_id(&self.properties().name)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas
    }

    fn get_context(&self) -> CanvasRenderingContext2d {
        let canvas = self.get_canvas();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context
    }

    fn begin_path(&self) {
        self.get_context().begin_path();
    }
    fn draw_point(&self, point: &ElementPoint, color: Color, line_width: f64) {
        let ctx = self.get_context();
        let (r, g, b) = color.dissolve();
        ctx.set_stroke_style_str(&format!("rgb({},{},{})", r, g, b)[..]);
        ctx.set_line_width(line_width);
        ctx.line_to(point.x, point.y);
        ctx.stroke();
    }
    fn clear_canvas(&self) {
        let c = self.get_canvas();
        self.get_context()
            .clear_rect(0.0, 0.0, c.width() as f64, c.height() as f64);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DrawingCanvas {
    pub properties: CanvasProperties,
}

impl DrawingCanvas {
    pub fn erase(&self, point: ElementPoint, line_width: f64) {
        let ctx = self.get_context();
        let eraser_size = line_width;
        ctx.clear_rect(
            point.x - eraser_size / 2.0,
            point.y - eraser_size / 2.0,
            line_width,
            line_width,
        );
    }

    pub fn draw_rect(
        &self,
        start: &ElementPoint,
        end: &ElementPoint,
        color: &Color,
        line_width: f64,
    ) {
        let ctx = self.get_context();
        let (r, g, b) = color.dissolve();
        ctx.begin_path();
        ctx.set_line_width(line_width);
        ctx.set_stroke_style_str(&format!("rgb({},{},{})", r, g, b)[..]);
        ctx.rect(start.x, start.y, end.x - start.x, end.y - start.y);
        ctx.stroke();
    }

    pub fn draw_circle(
        &self,
        start: &ElementPoint,
        end: &ElementPoint,
        color: &Color,
        line_width: f64,
    ) {
        let ctx = self.get_context();
        let (r, g, b) = color.dissolve();
        ctx.begin_path();
        ctx.set_line_width(line_width);
        ctx.set_stroke_style_str(&format!("rgb({},{},{})", r, g, b)[..]);
        ctx.arc(
            start.x,
            start.y,
            ((end.x - start.x).powf(2.0) + (end.y - start.y).powf(2.0)).sqrt(),
            0.0,
            2.0 * PI,
        )
        .unwrap();
        ctx.stroke();
    }
    pub fn draw_line(
        &self,
        start: &ElementPoint,
        end: &ElementPoint,
        color: &Color,
        line_width: f64,
    ) {
        let ctx = self.get_context();
        let (r, g, b) = color.dissolve();
        ctx.begin_path();
        ctx.set_line_width(line_width);
        ctx.move_to(start.x, start.y);
        ctx.set_stroke_style_str(&format!("rgb({},{},{})", r, g, b)[..]);
        ctx.line_to(end.x, end.y);
        ctx.stroke();
    }

    pub fn save_canvas(&self) {
        let document = get_document();
        let c = self.get_canvas();

        let img_url = c.to_data_url().unwrap();
        let img_link = document
            .create_element("a")
            .unwrap()
            .dyn_into::<HtmlAnchorElement>()
            .unwrap();

        img_link.set_href(&img_url[..]);
        img_link.set_download("image.png");
        img_link.click();
        img_link.remove();
    }

    pub fn execute_commands(&self, commands: Vec<Command>) {
        match commands.is_empty() {
            true => {}
            false => {
                commands.iter().for_each(|command| command.execute(&self));
            }
        }
    }
}

impl Canvas for DrawingCanvas {
    fn properties(&self) -> &CanvasProperties {
        &self.properties
    }
}

#[derive(Clone, Copy)]
pub struct SatValCanvas {
    pub properties: CanvasProperties,
}

impl SatValCanvas {
    pub fn draw_color_picker(&self, hue: f64) {
        let c = self.get_canvas();
        (0..c.width()).for_each(|px_x| {
            (0..c.height()).for_each(|px_y| {
                let (r, g, b): (u8, u8, u8) = hsv::hsv_to_rgb(
                    hue,
                    (px_x as f64) / (c.width() as f64),
                    (px_y as f64) / (c.height() as f64),
                );

                let ctx = self.get_context();
                let fill_style = &format!("rgb({},{},{})", r, g, b)[..];

                ctx.set_fill_style_str(fill_style);
                ctx.fill_rect(px_x as f64, (c.height() - 1 - px_y) as f64, 1.0, 1.0);
            });
        });
    }

    pub fn draw_sat_val_pointer(&self, sat_val_coord: (f64, f64)) {
        let ctx = self.get_context();
        ctx.begin_path();
        ctx.arc(
            sat_val_coord.0,
            sat_val_coord.1,
            3.0, // radius of circle
            0.0,
            2.0 * PI,
        )
        .unwrap();

        ctx.set_stroke_style_str(&format!("rgb(255,255,255)")[..]);
        ctx.stroke();
    }

    pub fn read_color(self, sat_val_coord: (f64, f64)) -> Color {
        let color = self
            .get_context()
            .get_image_data(sat_val_coord.0, sat_val_coord.1, 1.0, 1.0)
            .unwrap()
            .data();
        Color(color[0] as f64, color[1] as f64, color[2] as f64)
    }
}

impl Canvas for SatValCanvas {
    fn properties(&self) -> &CanvasProperties {
        &self.properties
    }
}

#[derive(Clone, Copy)]
pub struct HueCanvas {
    pub properties: CanvasProperties,
}

impl HueCanvas {
    pub fn draw_hue_pointer(&self, hue: f64) {
        let c = self.get_canvas();
        let ctx = self.get_context();

        ctx.clear_rect(0.0, 0.0, c.width() as f64, (c.height() / 2) as f64);

        ctx.set_fill_style_str("rgb(0,0,0)");
        ctx.fill_rect(
            hue * (c.width() as f64 / 360.0) as f64,
            0.0,
            2.0,
            (c.height() / 2) as f64,
        )
    }

    pub fn draw_hue_bar(&self, hue: f64) {
        let c = self.get_canvas();
        let ctx = self.get_context();

        (0..c.width()).for_each(|px_x| {
            let (r, g, b): (u8, u8, u8) =
                hsv::hsv_to_rgb(360.0 * ((px_x as f64) / (c.width() as f64)), 1.0, 1.0);
            ctx.set_fill_style_str(&format!("rgb({},{},{})", r, g, b)[..]);
            ctx.fill_rect(
                px_x as f64,
                (c.height() / 2) as f64 + 1.0,
                1.0,
                (c.height() / 2) as f64,
            );
        });
        self.draw_hue_pointer(hue);
    }
}

impl Canvas for HueCanvas {
    fn properties(&self) -> &CanvasProperties {
        &self.properties
    }
}

// PRIVATE

fn get_document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}
