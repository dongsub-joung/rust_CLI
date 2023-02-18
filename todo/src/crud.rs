pub mod Crud{

    pub fn list_up(list: Vec<String>){
        println!("{:?}", list)
    }

    pub fn adding(list: Vec<String>) -> Vec<String>{
        let inputed= inputing_string();
        let mut list_new= list.clone();

        list_new.push(inputed);

        list_new
    }

    pub fn deleting(list: Vec<String>) -> Vec<String>{
        let index= inputing();
        let mut list_new= list.clone();

        list_new.remove(index as usize);

        list_new
    }

    pub fn inputing() -> i32{ 
        let mut std_number= String::new();   
        stdin().read_line(&mut std_number).unwrap();
        let result :usize= std_number.trim_end().parse().unwrap();
        
        result as i32 
    }

    pub fn inputing_string() -> String{
        let mut inputed= String::new();
        stdin().read_line(&mut inputed).unwrap();
        
        let result= inputed.trim_end().to_string();

        result
    }
}
