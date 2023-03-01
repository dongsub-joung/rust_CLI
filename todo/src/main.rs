mod tools;

use tools;
use std::io::{self, stdin};

fn main() {
    loop{
        let mut list: Vec<String>= Vec::new();
    
        println!("Press a button\n");
        println!("1. Add a todo\n");
        println!("2. Delete a todo\n");
       
        let number= inputing();
       
        match number{
            0 => break,
            1 => {
                let inputed= inputing_string();
                
                let list= list.clone();
                
                let added_list= tools::Add::adding(inputed, list);
                
                tools::List::list_up(&added_list);
            },
            2 => {
                let index= inputing();
                
                let deleted_list= tools::Delete::deleting(index, list_backup);
                
                tools::List::list_up(&deleted_list);
            },
            _ => tools::List::list_up(&list),
        }
    }


    fn inputing() -> i32{ 
        let mut std_number= String::new();   
        stdin().read_line(&mut std_number).unwrap();
        let result :usize= std_number.trim_end().parse().unwrap();
        
        result as i32 
    }

    fn inputing_string() -> String{
        let mut inputed= String::new();
        stdin().read_line(&mut inputed).unwrap();
        
        let result= inputed.trim_end().to_string();

        result
    }
}

