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

    println!("1. keep going, 2. pass"); 
    let number: usize= inputing().trim().parse().expect("unvalid foramt- not int");

    println!("Set a absolute path");  
    // let pwd= inputing();
    let pwd= "/home/kiririn/git/rust_CLI/file_hashing_cli/src/txt.txt";
    let file_data= io_file::io_file::selecteing_file(pwd)
        .expect("can't parse the data");

    match number{
        1 => {
                let key = b"supersecretkey12"; // Replace with your own key
                let iv = b"initializationvec"; 
                result= hash::hashing::encrypt_string(key, iv,file_data.as_str())
                    .expect("can't encrypt");
        },       
        2 => {
            println!("Bye");   
        },       
        _ => println!("invalid value"),                            
    }                                      

    io_file::io_file::saving_file(file_data).expect("faild a saveing work");

    println!("Done!");
}


