use std::io;

// @todo
// lock stdin buffer 
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

    fn adding(mut list: Vec<String>) -> Vec<String>{
        let inputed= inputing_string();

        list.push(inputed);

        list
    }

    fn deleting(mut list: Vec<String>) -> Vec<String>{
        let index= inputing();
        
        list.remove(index as usize);

        list
    }

    fn inputing() -> i32{ 
        let mut std_number= String::new();   
        let strs= io::stdin().read_line(&mut std_number);
        let number_string= strs.unwrap().to_string().trim_end().to_string();
        
        number_string.parse().unwrap()
    }

    fn inputing_string() -> String{
        let mut inputed= String::new();
        let strs= io::stdin().read_line(&mut inputed);
        
        strs.unwrap().to_string().trim_end().to_string()
    }
}


