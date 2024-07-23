// This should allow us to input arguments from the command line
// It is probably not the best way to do it since it is my first time
use rand::Rng;
use std::env;

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

    gen_pwd(passwd_type, length);
}

fn gen_pwd(passwd_type: &str, length: u8) {
    match passwd_type {
        "l" => {
            let mut passwd = String::new();
            for _ in 0..length {
                let mut buf = [0; 4];
                passwd += char::from_u32(rand::thread_rng().gen_range(97..=122))
                    .unwrap()
                    .encode_utf8(&mut buf);
            }
            println!("{}", passwd);
        }
        "a" => {
            let mut passwd = String::new();
            println!("{}", passwd);
        }
        _ => {
            println!("The type of the password should be either 'l' or 'a'");
            usage();
        }
    }
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
