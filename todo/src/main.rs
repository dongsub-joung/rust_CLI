use std::io;

fn main() {
    whill number==0{
        let mut list: Vec<String>= Vec::new();

        println!("Press a button\n");
        println!("1. Show up list\n");
        println!("2. Add a todo\n");
        println!("3. Delete a todo\n");
       
        let number= inputing();
       
        match number{
            0i32 => break,
            1i32 => list_up(&list),
            2i32 => list= adding(list),
            3i32 => list= deleting(list),
            _ => continue,
        }
    }

    fn list_up(list: &Vec<String>){
        // 1i32 => println!(":?", list),
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


