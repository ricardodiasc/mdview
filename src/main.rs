use std::{path::Path, fs::File, io::Read, panic};

fn open_file() -> String {
    let path = Path::new("foo.md");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open filei {} .: {}", display, why),
        Ok(file) => file,
    };
    
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Could not read {}. : {}", display, why),
        Ok(_) => println!("{}", termimad::inline(&s)),
    };
    
    return s;
}


fn main() {
    
    open_file();
    // println!("{}" , termimad::inline(&content));
    // println!("{}", termimad::inline("*some*"));
    println!("End of File!");
}
