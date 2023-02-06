fn main() {
    loop{
        println!("Press a button\n");
        println!("1. Show up list\n");
        println!("2. Add a todo\n");
        println!("3. Delete a todo\n");
        
        let entered_number= stdin();
        let mut list: Vec<String>= Vec::new();
        
        match entered_number{
            1 => showUp(&list),
            2 => adding(&list),
            3 => deleting(&list),
            _ => none
        }
    }

    fn listUp(&mut list: Vec<String>){
        for( i in list){
            println!("{}. {}", i, element)
        }
    }

    fn adding(mut list: Vec<String>) -> Vec<String>{
        let inputed= stdin();
        
        list.append(inputed);
        
        list
    }

    fn deleting(mut list: Vec<String>) -> Vec<String> {
        // get index

        list.remove(index);
        
        list
    }
}


