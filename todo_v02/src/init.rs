mod service;
mod user;
mod todo;

use std::io::{self};


fn inputing_str()->String{
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input
}
pub fn run(){
    println!("ID");
    let first_input= inputing_str();

    println!("PW");
    let second_input = inputing_str();

    let user_id = first_input;
    let user_pw = second_input;


    println!("Todo");
    let todo= inputing_str();
    service::service::add_todos(todo);


    // let user= user::utils::login(user_id, user_pw);
    // if user.status == true {
    //     // let todos= service::service::get_todos();
    //     // print
        
    // }else {
    //     println!("You are not join us");
    // }
}