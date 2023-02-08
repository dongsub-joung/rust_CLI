use std::io::{self, stdin};

// @todo
// Need deep copy
fn main() {
    loop{
        let mut list: Vec<String>= Vec::new();
    
        println!("Press a button\n");
        println!("1. Show up list\n");
        println!("2. Add a todo\n");
        println!("3. Delete a todo\n");
       
        let number= inputing();
       
        match number{
            0 => break,
            1 => list_up(list),
            2 => list= adding(list),
            3 => list= deleting(list),
            _ => list_up(list),
        }
    }

    fn list_up(list: Vec<String>){
        println!("{:?}", list)
    }

    fn adding(list: Vec<String>) -> Vec<String>{
        let inputed= inputing_string();
        let mut list_new= list.clone();

        list_new.push(inputed);

        list_new
    }

    fn deleting(list: Vec<String>) -> Vec<String>{
        let index= inputing();
        let mut list_new= list.clone();

        list_new.remove(index as usize);

        list_new
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


