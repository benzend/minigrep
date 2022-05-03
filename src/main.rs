use std::env;

const FLAG: &str = "--";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut flags: Vec<String> = Vec::new();
    
    for arg in args {
        if is_flag(&arg) {
            flags.push(String::from(&arg));
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
