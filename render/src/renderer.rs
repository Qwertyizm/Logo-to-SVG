use runtime::colors::LogoColor;
use runtime::common::Pos;
use runtime::drawinglib::add_drawinglib;
use runtime::logo_interp::executor::execute_str;
use runtime::logo_interp::executor_state::EState;
use runtime::logo_interp::stdlib::add_stdlib;
use runtime::state::{Delegate, State};


use svg::node::element::Path;
use svg::node::element::path::Data;

use rusttype::{Point, Font};
use text_svg::Text;


pub struct DrawingDelegate {
    pub path_set: Vec<Path>,
    pub size: Pos,
    pub show_fn: Option<Box<dyn Fn(&str)>>,
}

impl Delegate for DrawingDelegate {
    fn clear_graphics(&mut self) {
        self.path_set.clear();
    }

    fn draw_line(&mut self, from: Pos, to: Pos, pen_size: f64, color: LogoColor) {
        let mut pth=Path::new().set("stroke",color.get_name())
                                     .set("stroke_width",pen_size);
        let mut data=Data::new();
        data=data.move_to((from.x as f32,-from.y as f32)).line_to((to.x as f32,-to.y as f32));
        pth=pth.set("d",data);
        self.path_set.push(pth);
    } 
    
    fn make_text(&mut self, pos: Pos, angle: f64, labelh: f64, color: LogoColor, word: String){
        let font_data: &[u8] = include_bytes!("impact.ttf");
        let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
        let text=Text::builder()
                        .size(labelh as f32)
                        .start(Point { 
                                x:0.0,
                                y:0.0
                                })
                        .build(&font,word.as_str());
        let path=text.path
                                .set("transform",format!("translate({},{}) rotate({})",pos.x,-pos.y,angle-90.0).as_str())
                                .set("text-anchor", "middle")
                                .set("fill",color.get_name());
        self.path_set.push(path);
    }

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