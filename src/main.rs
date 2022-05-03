use std::env;

const FLAG: &str = "--";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    for arg in args {
        if is_flag(&arg) {
            println!("{} is a flag", &arg);
        } else {
            println!("{} is not a flag", &arg);
        }
    }
    
}

fn is_flag(string: &String) -> bool {
    if &string[0..2] == FLAG {
        true
    } else {
        false
    }
}
