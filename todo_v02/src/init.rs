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

    let user= user::utils::login(user_id, user_pw);
    if user.status == true {
        let todos_json= service::service::get_todos();
        let todos= service::service::handle_json(todos_json);
        for todo in todos {
            todo::Todo::show_info(todo);
        }
    }else {
        println!("You are not join us");
    }
}