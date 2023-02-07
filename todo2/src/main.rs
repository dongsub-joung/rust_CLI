use std::io;

fn main() {
    loop{
        let mut list: Vec<String>= Vec::new();

        println!("Press a button\n");
        println!("1. Show up list\n");
        println!("2. Add a todo\n");
        println!("3. Delete a todo\n");
       
        let number= inputing();
       
        // fix list= list.push(inputed);
        // fix list= list.remove(inputed_index)
        match number{
            1i32 => println!("{:?}", list),
            2i32 => list.push(inputing_string()),
            3i32 => list.remove(inputing() as u32),
            _ => break,
        }
    }

    fn adding(mut list: Vec<String>) -> Vec<String>{
        let inputed= inputing_string();
       
        let mut list_new= list;
        
        list_new.push(inputed);

        list_new
    }

    fn deleting(mut list: Vec<String>) -> Vec<String>{
        let index= inputing();
        let mut list_new= list;
        
        list_new.remove(index as usize);

        list_new
    }

    fn inputing() -> i32{ 
        let mut std_number= String::new();   
        let strs= io::stdin().read_line(&mut std_number);
        let number_string= strs.unwrap().to_string();
        
        number_string.parse().unwrap()
    }

    fn inputing_string() -> String{
        let mut inputed= String::new();
        let strs= io::stdin().read_line(&mut inputed);
        let result= strs.unwrap().to_string();
        
        result
    }
}


