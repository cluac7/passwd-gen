// This should allow us to input arguments from the command line
// It is probably not the best way to do it since it is my first time
use std::{any::type_name_of_val, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = &args[1];
    let passwd_type = &args[2];

    println!(
        "The length of the password should be: {}, it is a {}",
        length,
        type_name_of_val(length)
    );
    println!(
        "The type of the password should be: {}, it is a {}",
        passwd_type,
        type_name_of_val(passwd_type)
    );
}
