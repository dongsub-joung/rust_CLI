#![allow(dead_code)]
#![allow(unused_variables)]
#[warn(unused_must_use)]

use self::models::*;
use todo_v03::*;

#[path ="./service/mod.rs"]
mod service;

pub mod join{
    use super::*;
    use sha2::{Digest, Sha256};

    pub fn login(user_id: String, user_pw: String) -> User{
        // get user from DB
        let user_id_clone= user_id.clone();
        let user_pw_clone= user_pw.clone();
        let mut user= service::service::get_user(&user_id).unwrap().unwrap();

        // hasing pw
        let mut hasher = Sha256::new();
        hasher.update(user_pw_clone.as_bytes());
        let result = hasher.finalize();
        let hash_str = format!("{:x}", result);
        
        // if hashed == hashing
        if user_id_clone == user.user_id && hash_str == user.hashed_user_pw {
            user.user_status= true;
        }

        user
    }
}

#[allow(dead_code)]
pub mod init{
    use super::*;
    use std::io::{self};

    fn inputing_str()->String{
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input
    }

    pub fn menu(){
        println!("1. Join");
        println!("2. Login-ADD");
        println!("3. Manage");
    }

    pub fn id_and_pw() -> (String, String){
        println!("ID");
        let user_id_str= &inputing_str();
        println!("PW");
        let user_pw= &inputing_str();

        (user_id_str.to_string(), user_pw.to_string())
    }

    pub async fn run() {
       
        menu();
        let mut number: i32= inputing_str().trim().parse().unwrap();

        loop {

            match number {
                1 =>{
                    println!("You are not join us");
                    println!("Join us!");
                    let (user_id_str, user_pw)= id_and_pw();
                    service::service::create_user(user_id_str.as_str(), user_pw.as_str());
                },
                2 =>{

                    let (user_id, user_pw) = id_and_pw();
            
                    let user= join::login(user_id, user_pw);
                    if user.user_status == true {
                        println!("OK");
                        let todos_json = service::service::get_todos().await.unwrap();
                        for todo in todos_json  {
                            println!("{:?}", todo);
                        }

                        println!("Add todo");
                        let todotext_inputed= &inputing_str();
                        let _ = service::service::create_todo(todotext_inputed).await.unwrap();
                    }
                },
                3 => {
                    let (user_id, user_pw) = id_and_pw();
            
                    let user= join::login(user_id, user_pw);
                    if user.user_status == true {
                       println!("removing id");
                       let id_inputed= &inputing_str();
                       service::service::remove_todo(id_inputed).await.unwrap();
                    }
                }
                _ => {continue;}
            }

            menu();
            number= inputing_str().trim().parse().unwrap();
        }
    }
}

#[async_std::main]
async fn main() {

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            init::run().await;
        })
}