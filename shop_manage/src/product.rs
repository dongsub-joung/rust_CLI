use uuid::Uuid;

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


pub struct Date {
    day: u32,
    month: u32,
    year: i32,
}

// Implementation of methods for the Date struct
impl Date {
    // Constructor function to create a new Date instance
    pub fn new(day: u32, month: u32, year: i32) -> Date {
        Date {
            day,
            month,
            year,
        }
    }

    // Function to display the date
    fn display(&self) {
        println!("{:02}/{:02}/{:04}", self.month, self.day, self.year);
    }
}

pub struct Product{
    product_id: String,
    volume: i128,
    per_cost: i128,
    managers: Product_Info,
    received_time: Date,
}


impl Product{
    
    pub fn new(volume: &str, per_cost: &str, managers: Product_Info, time: Date)->Self{
        let uuid= Uuid::new_v4().to_string();
        
        let volume: i128= volume.parse().unwrap();
        let per_cost: i128= per_cost.parse().unwrap();
        
        Self{
            product_id : uuid
            , volume
            , per_cost
            , managers 
            , received_time : time
        }
    }

    pub fn get_product_id(self) -> String{
        self.product_id
    }

    pub fn accounting(mut self, amount: i128){
        const STOCK: i128= 10;

        self.volume-= amount;
        if &self.volume < &0 {
            println!("Can't sell it all");
            println!("just {}", &self.volume);
            self.volume+=STOCK;
        }else{
            println!("Thx, i can sell it {}, {}", &amount, (&self.per_cost * &amount));
        }
    }

}