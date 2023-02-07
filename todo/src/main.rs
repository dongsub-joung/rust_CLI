use std::io;

fn main() {
    loop{
        let mut list: Vec<String>= Vec::new();
        
        println!("Press a button\n");
        println!("1. Show up list\n");
        println!("2. Add a todo\n");
        println!("3. Delete a todo\n");
       
        let number= inputing();
        
        match number{
            1i32 => listUp(&list),
            2i32 => adding(&list),
            3i32 => deleting(&list),
            _ => continue,
        }
    }

    fn listUp(mut list: &Vec<String>){
        println!("{:?}", list)
    }

    fn adding(mut list: &Vec<String>){
        let mut inputed= String::new();
        let stdin= io::stdin();

        stdin.read_line(&mut inputed);
       
        list.push(inputed);
    }

    fn deleting(mut list: &Vec<String>){
        let index= inputing();
        list.remove(index as usize);
    }

    fn inputing() -> i32{ 
        let mut std_number= String::new();   
        let strs= io::stdin().read_line(&mut std_number);
        let number_string= strs.unwrap().to_string();
        
        number_string.parse().unwrap()
    }
}


