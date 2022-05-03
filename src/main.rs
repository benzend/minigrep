use std::env;
use std::fs;

const FLAG: &str = "flag-";
const READ_FILE_FLAG: &str = "read_file";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut non_flags: Vec<String> = Vec::new();
    let mut flags: Vec<String> = Vec::new();
    
    for arg in args {
        if is_flag(&arg) {
            flags.push(String::from(&arg[FLAG.len()..]));
        } else {
            non_flags.push(String::from(&arg));
        }
    }

    if flags.contains(&String::from(READ_FILE_FLAG)) {
        if non_flags.len() == 1 {
            let file_to_read = &non_flags[0];
            let contents = fs
                ::read_to_string(file_to_read)
                .expect("Something went wrong reading the file");

            println!("{}", contents);
        }


    }

}

fn is_flag(string: &String) -> bool {
    if &string[0..FLAG.len()] == FLAG {
        true
    } else {
        false
    }
}
