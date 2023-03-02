use std::io;

fn main() {
    let file= selecteingFile();

    println!("1. Sha256, 2. sha512"); 
    let number: usize= inputing().parse().unwrap();
    
    let mut result= String::new();
    loop {
        match number{
            1usize => result= hashing::sha256(file),       
            2usize => result= hashing::sha512(file),       
            _ => continue,                            
        }
}                                     
    
    

    println!("Hello, world!");

    fn selecteingFile() -> File{
        // path 
        // selecting
        //
    }

    fn inputing()-> String{
        let mut number= String::new();                 
        io::stdin().read_line(&mut number).unwrap();
        
        number
    }
}

pub mod hashing{
    pub fn sha256(file :File) -> String{
    
    }

    pub fn sha512(file: File) -> String{
    
    }
}
