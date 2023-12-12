mod io_file;
mod hash;

use std::io;

fn inputing()-> String{
    let mut buff= String::new();                 
    io::stdin().read_line(&mut buff).expect("unvalid value and format");

    buff
}

fn main() {
    let mut result: Vec<u8>= Vec::new();
    let key = b"MyNameIsDongSub"; // Replace with your own key
    let iv = b"SomeOneHelpMePlz"; // Replace with your own initialization vector

    println!("1. keep going, 2. pass"); 
    let number: usize= inputing().trim().parse().expect("unvalid foramt- not int");

    println!("Set a absolute path");  
    // let pwd= inputing();
    // let pwd= "/home/kiririn/git/rust_CLI/file_hashing_cli/src/txt.txt";
    let pwd= "/home/kiririn/git/rust_CLI/file_hashing_cli/txt.txt";
    
    // let plaintext = "Hello, this is a secret message!";
    let plaintext= io_file::io_file::selecteing_file(pwd)
        .expect("can't parse the data");

    match number{
        1 => {
                result= match hash::hashing::encrypt_string(key, iv, plaintext.as_str()) {
                    Ok(ciphertext) => {
                        ciphertext
                    },
                    Err(e) => {
                        eprintln!("Encryption error: {:?}", e);
                        let mut null_vec: Vec<u8>= Vec::new();
                        null_vec
                    },
                };
        },       
        2 => {
            println!("Bye");   
        },       
        _ => println!("invalid value"),                            
    }                                      

    io_file::io_file::saving_file(&result).expect("faild a saveing work");

    println!("Done!");
}


