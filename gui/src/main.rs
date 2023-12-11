mod printer;
use printer::Printer;

//mod sample_svg;



fn main() {
    let mut prtr=Printer::new(800,800,"image.svg");
    let res=prtr.print(
        " to draw :repcount
        make \"colors [\"red \"orange \"yellow \"green \"blue \"violet]
        repeat 144 [
            setc item remainder :repcount 6  :colors
            setlabelheight :repcount
            pu
            fd :repcount * :repcount / 30
            label \"Logo
            bk :repcount * :repcount / 30
            pd
            rt 10
            make \"repcount 1 + :repcount
        ]
        end
        window
        draw 1"
    );
    match res{
        Ok(_)=>(),
        Err(msg)=>println!("{}",msg)
    }
}



/* example - include in tests
" to draw :repcount
        repeat 144 [
            setlabelheight :repcount
            pu
            fd :repcount * :repcount / 30
            label \"Logo
            bk :repcount * :repcount / 30
            pd
            rt 10
            make \"repcount 1 + :repcount
        ]
        end
        draw 1"
         */