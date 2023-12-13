pub mod crud{
    use rusqlite::{Connection, Result};
    
    pub fn add() -> Result<()>{   
        let conn = Connection::open("database.db")?;
      
        println!("add");

        Ok(())
    }
    pub fn list_up(){
        println!("list up"); 
    }

    pub fn update(){
        println!("update");
    }

    pub fn delete(){
        println!("del");
    }
}

