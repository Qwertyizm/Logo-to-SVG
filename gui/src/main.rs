mod printer;
use printer::Printer;

//mod sample_svg;



fn main() {
    let mut prtr=Printer::new(800,800,"image.svg");
    let res=prtr.print(
    "to square :length
        repeat 4 [fd :length rt 90]
        end
        
        TO randomcolor
        setcolor pick [ 1 20 15 0 ]
        end
    
        repeat 36 [ randomcolor square random 200 rt 10 ]"
    );
    match res{
        Ok(_)=>(),
        Err(msg)=>println!("{}",msg)
    }
}
