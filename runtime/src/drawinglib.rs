use logo_interp::core::LogoValue;
use logo_interp::executor_state::*;
//use crate::state;

use super::colors::get_color_from_str;
use super::state::{Delegate, PenState, State};
use super::common::Pos;

pub fn add_drawinglib<D: Delegate + 'static>(es: &mut EState<State<D>>) {
    es.functions.insert("show".to_string(), Function::from_proc1(show));

    es.functions.insert("cg".to_string(), Function::from_proc(cg));
    es.functions.insert("clean".to_string(), Function::from_proc(clean));
    es.functions.insert("clearscreen".to_string(), Function::from_proc(clean));

    es.functions.insert("pu".to_string(), Function::from_proc(pu));
    es.functions.insert("pd".to_string(), Function::from_proc(pd));
    es.functions.insert("pe".to_string(), Function::from_proc(pe));

    es.functions.insert("rt".to_string(), Function::from_proc1(rt));
    es.functions.insert("right".to_string(), Function::from_proc1(rt));
    es.functions.insert("lt".to_string(), Function::from_proc1(lt));
    es.functions.insert("left".to_string(), Function::from_proc1(lt));
    es.functions.insert("fd".to_string(), Function::from_proc1(fd));
    es.functions.insert("forward".to_string(),Function::from_proc1(fd));
    es.functions.insert("bk".to_string(), Function::from_proc1(bk));
    es.functions.insert("back".to_string(), Function::from_proc1(bk));

    es.functions.insert("heading".to_string(), Function::from_fn(heading));
    es.functions.insert("seth".to_string(), Function::from_proc1(seth));
    es.functions.insert("setheading".to_string(), Function::from_proc1(seth));
    es.functions.insert("setpos".to_string(), Function::from_proc1(setpos));
    es.functions.insert("setx".to_string(), Function::from_proc1(setx));
    es.functions.insert("sety".to_string(), Function::from_proc1(sety));
    es.functions.insert("pos".to_string(), Function::from_fn(pos));
    es.functions.insert("xcoor".to_string(), Function::from_fn(xcoor));
    es.functions.insert("ycoor".to_string(), Function::from_fn(ycoor));
    es.functions.insert("home".to_string(), Function::from_proc(home));

    es.functions.insert("pensize".to_string(), Function::from_fn(pensize));
    es.functions.insert("setpensize".to_string(), Function::from_proc1(setpensize));

    es.functions.insert("ht".to_string(), Function::from_proc(ht));
    es.functions.insert("st".to_string(), Function::from_proc(st));

    es.functions.insert("setc".to_string(), Function::from_proc1(setc));
    es.functions.insert("setcolor".to_string(), Function::from_proc1(setc));
    es.functions.insert("color".to_string(), Function::from_fn(color));

    es.functions.insert("setlabelheight".to_string(),Function::from_proc1(setlabelh));
    es.functions.insert("label".to_string(), Function::from_proc1(label));

    es.functions.insert("window".to_string(), Function::from_proc(window));
}

fn show<D: Delegate>(state: &mut EState<State<D>>, val: LogoValue) -> Result<(), String> {
    state.state.delegate.show(format!("{}", val).as_str());
    Ok(())
}

fn cg<D: Delegate>(state: &mut EState<State<D>>) -> Result<(), String> {
    let state = &mut state.state;
    state.data.turtle_pos = Pos{x: 0f64, y: 0f64};
    state.data.turtle_angle = 0f64;
    state.delegate.clear_graphics();
    Ok(())
}

fn clean<D: Delegate>(state: &mut EState<State<D>>) -> Result<(), String> {
    state.state.delegate.clear_graphics();
    Ok(())
}

fn pu<D: Delegate>(state: &mut EState<State<D>>) -> Result<(), String> {
    state.state.data.pen_state = PenState::Up;
    Ok(())
}

fn pd<D: Delegate>(state: &mut EState<State<D>>) -> Result<(), String> {
    state.state.data.pen_state = PenState::Down;
    Ok(())
}

fn pe<D: Delegate>(state: &mut EState<State<D>>) -> Result<(), String> {
    state.state.data.pen_state = PenState::Erase;
    Ok(())
}

fn rt<D: Delegate>(state: &mut EState<State<D>>, val: f64) -> Result<(), String> {
    state.state.data.turtle_angle += val;
    Ok(())
}

fn lt<D: Delegate>(state: &mut EState<State<D>>, val: f64) -> Result<(), String> {
    state.state.data.turtle_angle -= val;
    Ok(())
}

fn fd<D: Delegate>(state: &mut EState<State<D>>, val: f64) -> Result<(), String> {
    let old_pos = state.state.data.turtle_pos;
    let angle = state.state.data.turtle_angle;
    let delta_x = angle.to_radians().sin() * val;
    let delta_y = angle.to_radians().cos() * val;
    let new_pos = Pos{x: old_pos.x + delta_x, y: old_pos.y + delta_y};
    move_turtle(&mut state.state, new_pos);
    Ok(())
}

fn bk<D: Delegate>(state: &mut EState<State<D>>, val: f64) -> Result<(), String> {
    fd(state, -val)
}

fn heading<D: Delegate>(state: &mut EState<State<D>>) -> Result<f64, String> {
    Ok(state.state.data.turtle_angle)
}

fn seth<D: Delegate>(state: &mut EState<State<D>>, h: f64) -> Result<(), String> {
    state.state.data.turtle_angle = h;
    Ok(())
}

fn pos<D: Delegate>(state: &mut EState<State<D>>) -> Result<Vec<f64>, String> {
    Ok(vec![state.state.data.turtle_pos.x, state.state.data.turtle_pos.y])
}

fn setpos<D: Delegate>(state: &mut EState<State<D>>, pos: Vec<f64>) -> Result<(), String> {
    if pos.len() != 2 {
        Err("Setpos takes exactly 2 coordinates".to_string())
    }
    else {
        move_turtle(&mut state.state, Pos{ x: pos[0], y: pos[1] });
        Ok(())
    }
}

fn xcoor<D: Delegate>(state: &mut EState<State<D>>) -> Result<f64, String> {
    Ok(state.state.data.turtle_pos.x)
}

fn ycoor<D: Delegate>(state: &mut EState<State<D>>) -> Result<f64, String> {
    Ok(state.state.data.turtle_pos.y)
}

fn setx<D: Delegate>(state: &mut EState<State<D>>, x: f64) -> Result<(), String> {
    let y = state.state.data.turtle_pos.y;
    move_turtle(&mut state.state, Pos{x, y});
    Ok(())
}

fn sety<D: Delegate>(state: &mut EState<State<D>>, y: f64) -> Result<(), String> {
    let x = state.state.data.turtle_pos.x;
    move_turtle(&mut state.state, Pos{x, y});
    Ok(())
}

fn home<D: Delegate>(state: &mut EState<State<D>>) -> Result<(), String> {
    move_turtle(&mut state.state, Pos{ x: 0f64, y: 0f64 });
    state.state.data.turtle_angle = 0.0;
    Ok(())
}

fn setpensize<D: Delegate>(state: &mut EState<State<D>>, pen_size: f64) -> Result<(), String> {
    state.state.data.pen_size = pen_size;
    Ok(())
}

fn pensize<D: Delegate>(state: &mut EState<State<D>>) -> Result<f64, String> {
    Ok(state.state.data.pen_size)
}

fn ht<D: Delegate>(state: &mut EState<State<D>>) -> Result<(), String> {
    state.state.data.turtle_visible = false;
    Ok(())
}

fn st<D: Delegate>(state: &mut EState<State<D>>) -> Result<(), String> {
    state.state.data.turtle_visible = true;
    Ok(())
}

fn window<D: Delegate>(state: &mut EState<State<D>>)->Result<(),String>{
    state.state.data.turtle_border=false;
    Ok(())
}

fn setc<D: Delegate>(state: &mut EState<State<D>>, color: String) -> Result<(), String> {
    let col = get_color_from_str(color.as_str());
    match col{
        Ok(c) => {
            state.state.data.color=c;
            Ok(())
        },
        Err(msg) => Err(msg)
    }
}

fn color<D: Delegate>(state: &mut EState<State<D>>) -> Result<String, String> {
    Ok(state.state.data.color.get_name())
}

fn setlabelh<D: Delegate>(state: &mut EState<State<D>>, h: f64)->Result<(),String>{
    if h>0.0{
        state.state.data.labelh=h;
        return Ok(());
    }
    Err(format!("Incorrect label height value: {}",h))
}

fn label<D: Delegate>(state: &mut EState<State<D>>, word: String)->Result<(),String>{
    make_text(&mut state.state, word);
    Ok(())
}

fn make_text<D:Delegate>(state: &mut State<D>, word: String){
    let color = state.data.color.clone();
    let height = state.data.labelh;
    let angle = state.data.turtle_angle;
    let pos = state.data.turtle_pos;
    state.delegate.make_text(pos,angle, height, color,word);
}

fn move_turtle<D: Delegate>(state: &mut State<D>, pos: Pos) {
    let old_pos = state.data.turtle_pos;
    if state.data.turtle_border{
        let w2 = state.data.canvas_width as f64 / 2f64;
        let h2 = state.data.canvas_height as f64 / 2f64;
        if pos.y > old_pos.y + f64::EPSILON {
            let xp = intersect_horizontal(old_pos, pos, h2, -w2, w2);
            if let Some(xpos)=xp {
                draw_line(state, old_pos, Pos{x: xpos, y: h2});
                state.data.turtle_pos = Pos{x: xpos, y: -h2};
                move_turtle(state, Pos{x: pos.x, y: pos.y - state.data.canvas_height as f64});
                return;
            }
        }
        if pos.y + f64::EPSILON < old_pos.y {
            let xp = intersect_horizontal(old_pos, pos, -h2, -w2, w2);
            if let Some(xpos)=xp {
                draw_line(state, old_pos, Pos{x: xpos, y: -h2});
                state.data.turtle_pos = Pos{x: xpos, y: h2};
                move_turtle(state, Pos{x: pos.x, y: pos.y + state.data.canvas_height as f64});
                return;
            }
        }
        if pos.x > old_pos.x + f64::EPSILON {
            let yp = intersect_vertical(old_pos, pos, w2, -h2, h2);
            if let Some(ypos)=yp {
                draw_line(state, old_pos, Pos{x: w2, y: ypos});
                state.data.turtle_pos = Pos{x: -w2, y: ypos};
                move_turtle(state, Pos{x: pos.x - state.data.canvas_width as f64, y: pos.y});
                return;
            }
        }
        if pos.x + f64::EPSILON < old_pos.x {
            let yp = intersect_vertical(old_pos, pos, -w2, -h2, h2);
            if let Some(ypos)=yp {
                draw_line(state, old_pos, Pos{x: -w2, y: ypos});
                state.data.turtle_pos = Pos{x: w2, y: ypos};
                move_turtle(state, Pos{x: pos.x + state.data.canvas_width as f64, y: pos.y});
                return;
            }
        }
        state.data.turtle_pos = pos;
        draw_line(state, old_pos, pos);
    }
    else{
        state.data.turtle_pos = pos;
        draw_line(state, old_pos, pos);
    }
}

fn draw_line<D: Delegate>(state: &mut State<D>, p1: Pos, p2: Pos) {

    let mut color = state.data.color.clone();
    if state.data.pen_state == PenState::Erase {
        color=get_color_from_str("white").expect("Don't know white color");
    }
    if state.data.pen_state != PenState::Up {
        state.delegate.draw_line(p1, p2, state.data.pen_size, color);
    }
}

fn intersect_horizontal(p1: Pos, p2: Pos, y: f64, x1: f64, x2: f64) -> Option<f64> {
    if p1.y.min(p2.y) > y || p1.y.max(p2.y) < y {
        return None;
    }
    let xp = p1.x - (p1.y - y) / (p1.y - p2.y) * (p1.x - p2.x);
    if xp >= x1 && xp <= x2 {
        Some(xp)
    }
    else {
        None
    }
}

fn intersect_vertical(p1: Pos, p2: Pos, x: f64, y1: f64, y2: f64) -> Option<f64> {
    if p1.x.min(p2.x) > x || p1.x.max(p2.x) < x {
        return None;
    }
    let yp = p1.y - (p1.x - x) / (p1.x - p2.x) * (p1.y - p2.y);
    if yp >= y1 && yp <= y2 {
        Some(yp)
    }
    else {
        None
    }
}
