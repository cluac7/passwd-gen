// This should allow us to input arguments from the command line
// It is probably not the best way to do it since it is my first time
use std::{any::type_name_of_val, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let length_str = &args[1];
    let passwd_type = &args[2];

    let length = match length_str.parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("The length of the password should be a number between 0 and 255");
            usage();
            return;
        }
    };

    println!(
        "The type of the password should be: {}, it is a {}",
        passwd_type,
        type_name_of_val(passwd_type)
    );
}

fn usage() {
    println!(
        r#"
Usage: passwd-gen <length> <type>
    <length>    The length of the password (0-255)
    <type>      The type of the password (l: lowercase, a: all characters)
        "#
    );
}
