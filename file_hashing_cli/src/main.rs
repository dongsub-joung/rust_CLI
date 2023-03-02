use std::io;

fn main() {
    let file= selecteingFile();

    println!("1. Sha256, 2. sha512"); 
    let mut result= String::new();
    let number: usize= inputing().parse().unwrap();
    
    loop {
        match number{
            1usize => {
                    result= hashing::sha256(file);
                    break
            },       
            2usize => {
                    result= hashing::sha512(file);
                    break
            },       
            _ => continue,                            
        }
    }                                      

    println!("{}", result);

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
