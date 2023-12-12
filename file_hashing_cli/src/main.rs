mod io_file;
mod get_os;
mod hash;

use std::io;

fn inputing()-> String{
    let mut buff= String::new();                 
    io::stdin().read_line(&mut buff).expect("unvalid value and format");
    let str_data= buff;
    buff.clear();
    
    str_data
}

fn main() {
    let mut result= String::new();

    println!("1. Sha256, 2. sha512"); 
    let number: usize= inputing().parse().expect("unvalid foramt- not int");
    
    println!("Set a absolute path");  
    let pwd= inputing();
    let file_data= io_file::io_file::selecteing_file(pwd.as_str()).expect("can't parse the data");

    match number{
        1usize => {
                result= hash::hashing::sha256(file_data);
        },       
        2usize => {
                result= hash::hashing::sha512(file_data);
        },       
        _ => println!("invalid value"),                            
    }                                      

    io_file::io_file::saving_file(file_data).expect("faild a saveing work");

    println!("Done!");
}


