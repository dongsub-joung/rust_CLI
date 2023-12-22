mod service;
mod user;
mod todo;
use std::io::{self, BufRead, Stdin, stdin};

pub fn run(){
    println!("ID");
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input)
        .expect("Failed to read line");

    println!("PW");
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input)
        .expect("Failed to read line");

    let user_id = first_input;
    let user_pw = second_input;

    let user= user::utils::login(user_id, user_pw);
    if user.status == true {
        // todo
    }else {
        println!("You are not join us");
    }
}