use std::io::{self, stdin};

fn main() {
    inputing();
}

fn inputing(){
    let mut buff= String::new();
    stdin().read_line(&mut buff).unwrap();
    println!("{}", buff);
}
