use render::renderer::Context;
use svg::Document;
use svg::node::element::Path;

pub struct Printer{
    context: Context,
    img_path: String,
}

impl Default for Printer{
    fn default() -> Self {
        Self{
            context: Context::new(800,450),
            img_path: "".to_owned(),
        }
    }
}

impl Printer{
    pub fn new(width:i32,height:i32,img_path:&str)->Self{
        Self { context: Context::new(width,height), img_path: img_path.to_string() }
    }

    fn to_file(&self,width:i32,height:i32,nicely_done: Vec<Path>){
        let mut document= Document::new()
            .set("height",height)
            .set("width",width)
            .set("viewBox",format!("-{},-{},{},{}",width/2,height/2,width,height));
        for path in nicely_done{
            document=document.add(path);
        }
        svg::save(self.img_path.as_str(),&document).unwrap();
    }

    fn parse(cmd:&str)->(String,String){
        let pos=cmd.rfind("end");
        match pos {
            Some(x) => ( cmd[..x+3].to_string() , cmd[x+3..].to_string() ),
            None => ( "".to_string() , cmd.to_string() )
        }
    }

    pub fn print(&mut self,cmd:&str)->Result<(),String>{
        let (proc_source,cmd_source)=Self::parse(cmd);
        let image=self.context.render(proc_source.as_str(), cmd_source.as_str())?;
        self.to_file(self.context.state.state.data.canvas_width,self.context.state.state.data.canvas_height,image);
        Ok(())

    }
}
