struct Server{
    root: String,
    pw: String,
    first_volume: u128,
    max_device: u8,
}

impl Server{
    pub fn new(root: String, pw: String, first_volume: u128, max_device:u8) -> Self{
        self{
            root, pw, first_volume, max_device
        }
    }
}