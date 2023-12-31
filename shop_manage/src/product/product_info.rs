
pub struct Product_Info{
    manager_name: String,
    address: String,
}

impl Product_Info{
    pub fn new(manager_name: &str, address: &str) -> Self{
        let manager_name= manager_name.to_string();
        let address=address.to_string();
         
        Self{
            manager_name, address
        }
    }
}