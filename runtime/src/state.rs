use svg::node::element::Path;
use super::colors::{LogoColor, get_color_from_str};
use super::common::Pos;



pub trait Delegate {
    fn clear_graphics(&mut self);
    fn draw_line(&mut self, from: Pos, to: Pos, pen_size: f64, color: LogoColor);
    fn show(&mut self, message: &str);
}


#[derive(Copy, Clone, PartialEq)]
pub enum PenState {
    Up,
    Down,
    Erase
}


#[derive(Clone)]
pub struct StateData {
    pub canvas_width: i32,
    pub canvas_height: i32,
    pub turtle_pos: Pos,
    pub turtle_angle: f64,
    pub turtle_visible: bool,
    pub pen_state: PenState,
    pub pen_size: f64,
    pub color: LogoColor,
    pub objects: Vec<Path>,
}

pub struct State<D: Delegate> {
    pub data: StateData,
    pub delegate: D,
}

impl<D: Delegate> State<D> {
    pub fn new(canvas_width: i32, canvas_height: i32, delegate: D) -> Self {
        State {
            data: StateData {
                canvas_width,
                canvas_height,
                turtle_pos: Pos { x: 0f64, y: 0f64 },
                turtle_angle: 0f64,
                turtle_visible: true,
                pen_state: PenState::Down,
                pen_size: 4f64,
                color: get_color_from_str("black").expect("Don't know black color"),
                objects: vec![]
            },
            delegate
        }
    }
}