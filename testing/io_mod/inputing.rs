use std::io;

pub mod inputing{
    pub fn init() -> String{
       let mut buff= String::new();
       io::stdin().read_line(&mut buff).unwrap();
        
       buff
    }
}
