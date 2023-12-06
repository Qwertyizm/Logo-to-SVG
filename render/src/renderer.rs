use runtime::colors::LogoColor;
use runtime::common::Pos;
use runtime::drawinglib::add_drawinglib;
use runtime::logo_interp::executor::execute_str;
use runtime::logo_interp::executor_state::EState;
use runtime::logo_interp::stdlib::add_stdlib;
use runtime::state::{Delegate, State};


use svg::node::element::Path;
use svg::node::element::path::Data;

pub struct DrawingDelegate {
    pub path_set: Vec<Path>,
    pub size: Pos,
    pub show_fn: Option<Box<dyn Fn(&str)>>,
}

impl DrawingDelegate {
    fn transform_coords(&self, pos: Pos) -> (f32, f32) {
        let width = self.size.x;
        let height = self.size.y;
        ((pos.x + width / 2f64 + 0.5) as f32, (-pos.y + height / 2f64 + 0.5) as f32)
    }
}

impl Delegate for DrawingDelegate {
    fn clear_graphics(&mut self) {
        self.path_set.clear();
    }

    fn draw_line(&mut self, from: Pos, to: Pos, pen_size: f64, color: LogoColor) {
        let upd_from = self.transform_coords(from);
        let upd_to = self.transform_coords(to);
        let mut pth=Path::new().set("stroke",color.get_name())
                                     .set("stroke_width",pen_size);
        let mut data=Data::new();
        data=data.move_to((upd_from.0,upd_from.1)).line_to((upd_to.0,upd_to.1));
        pth=pth.set("d",data);
        self.path_set.push(pth);
    } 

    //TODO fix showing text
    fn show(&mut self, message: &str) {
        if let Some(show_fn) = &self.show_fn {
            (show_fn)(message);
        }
    }
}

pub struct Context {
    pub state: EState<State<DrawingDelegate>>,
}

impl Context {
    pub fn new(width: i32, height: i32) -> Self {
        let dd = DrawingDelegate {
            size: Pos{x:width as f64, y: height as f64},
            path_set: Vec::<Path>::new(),
            show_fn: None,
        };
        let mut state = EState::new(State::new(width, height, dd));
        state.state.delegate.clear_graphics();
        add_stdlib(&mut state);
        add_drawinglib(&mut state);
        Self {state}
    }

    pub fn render(&mut self, proc_source: &str, cmd_source: &str) -> Result<Vec<Path>, String> {
        execute_str(&mut self.state, proc_source, cmd_source)?;
        Ok(self.state.state.delegate.path_set.clone())
    }
}