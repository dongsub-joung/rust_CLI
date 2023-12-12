mod crud;

use std::io::{self, stdin, Error};

fn intro() {
    println!("  _   _      _ _        __        __         _     _ _ _");
    println!(" | | | | ___| | | ___   \\ \\      / /__  _ __| | __| | | |");
    println!(" | |_| |/ _ \\ | |/ _ \\   \\ \\ /\\ / / _ \\| '__| |/ _` | | |");
    println!(" |  _  |  __/ | | (_) |   \\ V  V / (_) | |  | | (_| |_|_|");
    println!(" |_| |_|\\___|_|_|\\___/     \\_/\\_/ \\___/|_|  |_|\\__,_(_|_)");
}

fn inputing() -> Result<i32, Error>{ 
    let mut std_number= String::new();   
    stdin().read_line(&mut std_number).unwrap();
    let result :usize= std_number.trim_end().parse().unwrap();
    
    Ok(result as i32) 
}

fn inputing_string() -> Result<String, Error>{
    let mut inputed= String::new();
    stdin().read_line(&mut inputed).unwrap();
    
    let result= inputed.trim_end().to_string();

    Ok(result)
}

fn menu(){
    println!("\n");
    println!("\n");
    println!("Press a button\n");
    println!("1. Add a todo\n");
    println!("2. Update a todo\n");
    println!("3. Delete a todo\n");
}

fn main() {
    const IO_ERR:& str= "invalied input";
    const NOT_INT: &str= "invalied input: not int";

    loop{
        let mut list: Vec<String>= Vec::new();
        
        intro();
        menu();
       
        let number= inputing().expect(NOT_INT);
       
        match number{
            0 => break,
            1 => {
                let inputed= inputing_string().expect(IO_ERR);
                
                crud::crud::add();
                crud::crud::list_up();
            },
            2 => {
                let index= inputing().expect(NOT_INT);
                
                crud::crud::update();
                crud::crud::list_up();
            },
            3 => {
                let index= inputing().expect(NOT_INT);
                
                crud::crud::delete();
                crud::crud::list_up();
            },
            _ => crud::List::list_up(),
        }
    }
}

