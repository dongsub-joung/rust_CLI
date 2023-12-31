use uuid::Uuid;

mod date;
mod product_info;

pub struct Product{
    product_id: String,
    volume: i128,
    per_cost: i128,
    managers: product_info::Product_Info,
    received_time: date::Date,
}


impl Product{    
    pub fn new(volume: &str, per_cost: &str, managers: product_info::Product_Info, time: date::Date)->Self{
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

    pub fn new_product_iofo(manager_name: String, address: String)-> product_info::Product_Info{
        let manager= product_info::Product_Info::new(&manager_name, &address);
        manager
    }

    pub fn new_date(day: u32, month: u32, year: i32)-> date::Date{
        let time= date::Date::new(day, month, year);
        time
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