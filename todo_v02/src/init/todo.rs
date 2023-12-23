pub struct Todo{
    id: String,
    pub textbody: String,
    pub user_id: String,
}

impl Todo {
    pub fn show_info(self){
        println!(self.textbody);
        println!(self.user_id);
    }
}