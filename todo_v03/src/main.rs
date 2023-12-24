#![allow(dead_code)]
#![allow(unused_variables)]
#[warn(unused_must_use)]

use self::models::*;
use diesel::prelude::*;
use todo_v03::*;

pub mod join{
    use super::*;
    use sha2::{Digest, Sha256};

    pub fn login(user_id: String, user_pw: String) -> User{
        // get user from DB
        let user_id_clone= user_id.clone();
        let user_pw_clone= user_pw.clone();
        let mut user= service::get_user(&user_id).unwrap().unwrap();

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

pub mod service {
    use self::models::{NewUser, User};
    use self::schema::users::dsl::*;
    use sha2::{Digest, Sha256};
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    pub struct Todo {
        id: u64,
        todotext: String,
    }

    use reqwest::Response;

    pub async fn get_todos() -> Result<Vec<Todo>, ()> {
        let url = "https://todo.ngrok.app/todos"; // Replace with your API endpoint
    
        let response = reqwest::get(url).await.unwrap(); // Perform GET request
    
        if response.status().is_success() {
            let todos: Vec<Todo> = response.json().await.unwrap(); // Deserialize JSON into Vec<Todo>
            Ok(todos)
        } else {
            Err(())
        }
    }

    pub fn get_user(user_name_id: &str) -> Result<Option<todo_v03::models::User>, diesel::result::Error>{
        // use self::schema::users::dsl::users;
        use self::schema::users::dsl::user_id;

        let connection = &mut establish_connection();
    
        let a_user = users
            .filter(user_id.eq(user_name_id))
            .select(User::as_select())
            .first(connection)
            .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error
        
        a_user
    }

    pub fn show_users() -> Vec<User> {
        let connection = &mut establish_connection();
        let results = users
            .filter(user_status.eq(true))
            .limit(5)
            .select(User::as_select())
            .load(connection)
            .expect("Error loading posts");

        results
    }

    fn hashing(txt: &str) -> String{
        let mut hasher = Sha256::new();
        hasher.update(txt.as_bytes());
        let result = hasher.finalize();
        let hash_str = format!("{:x}", result);
        hash_str
    }
    
    pub fn create_user(user_id_str: &str, user_pw: &str){
        use crate::schema::users;

        let connection = &mut establish_connection();

        let hashed_user_pw_str= hashing(user_pw);
        let new_user = NewUser { 
            user_id: user_id_str
            , hashed_user_pw: &hashed_user_pw_str
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(connection)
            .expect("Error saving new post");
        
        println!("creating successed");
    }

    use reqwest::Client;
    use std::collections::HashMap;

    pub async fn create_todo(todotext_inputed: &str) -> Result<(), Box<dyn std::error::Error>>{
            // Create a reqwest Client
        let client = Client::new();

        // URL endpoint where you want to send the POST request
        let url = "https://todo.ngrok.app/todo/add";

        // Create a HashMap with the data to be sent in the request body
        let mut data = HashMap::new();
        data.insert("body_text", todotext_inputed);
        // Add more data as needed

        // Send the POST request asynchronously
        let response = client.post(url)
            .json(&data)
            .send()
            .await.unwrap();
        
        // Check if the request was successful (status code 200)
        if response.status().is_success() {
            println!("Request successful!");
            // Handle the response data if needed
            let body = response.text().await.unwrap();
            println!("Response body: {}", body);
        } else {
            println!("Request failed with status code: {}", response.status());
        }


        println!("creating successed");

        Ok(())
    }
}


#[allow(dead_code)]
pub mod init{
    use super::*;
    use std::io::{self};
    use reqwest::Response;
    use service::get_todos; 

    fn inputing_str()->String{
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input
    }

    pub async fn run() {
        println!("1. Join");
        println!("2. Login");
        println!("3. Manage");
        let mut number: i32= inputing_str().trim().parse().unwrap();

        loop {

            match number {
                1 =>{
                    println!("You are not join us");
                    println!("Join us!");
                    println!("ID");
                    let user_id_str= &inputing_str();
                    println!("PW");
                    let user_pw= &inputing_str();
                    service::create_user(user_id_str, user_pw);
                },
                2 =>{
                    println!("ID");
                    let first_input= inputing_str();
                
                    println!("PW");
                    let second_input = inputing_str();
                
                    let user_id = first_input;
                    let user_pw = second_input;
                
                    let user= join::login(user_id, user_pw);
                    if user.user_status == true {
                        println!("OK");
                        let todos_json = service::get_todos().await.unwrap();
                        for todo in todos_json  {
                            println!("{:?}", todo);
                        }

                        println!("Add todo");
                        let todotext_inputed= &inputing_str();
                        let _ = service::create_todo(todotext_inputed).await.unwrap();
                    }
                },
                3 => {
                    println!("ID");
                    let first_input= inputing_str();
                
                    println!("PW");
                    let second_input = inputing_str();
                
                    let user_id = first_input;
                    let user_pw = second_input;
                
                    let user= join::login(user_id, user_pw);
                    if user.user_status == true {
                       println!("remove");
                    }
                }
                _ => {continue;}
            }

            println!("1. Join");
            println!("2. Loin");
            println!("3. Manage");
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