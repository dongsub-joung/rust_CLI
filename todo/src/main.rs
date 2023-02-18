mod crud;

use std::io::{self, stdin};
use crud::Crud::*;

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
                let list_backup= list.clone();
                let added_list= adding(list_backup);
                list= added_list.clone();
                list_up(added_list);
                
            },
            2 => {
                let list_backup= list.clone();
                let deleted_list= deleting(list_backup);
                list= deleted_list.clone();
                list_up(deleted_list);
            },
            _ => list_up(list),
        }
    }
}
