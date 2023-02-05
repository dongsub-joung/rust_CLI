fn main() {
    loop{
        println!("Press a button\n");
        println!("1. Show up list\n");
        println!("2. Add list\n");
        println!("3. Delete list\n");
        
        // let entered_number= stdin();
        let list: Vec<String>= Vec::new();
        
        match entered_number{
            1 => showUp(),
            2 => adding(),
            3 => deleting(),
            _ => none
        }
    }

    fn showUp(){
        for( i in list){
            println!("{}. {}", i, element)
        }
    }

    fn adding(){
        let inputed= stdin();

    }
}


