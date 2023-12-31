
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
    pub fn display(&self) {
        println!("{:02}/{:02}/{:04}", self.month, self.day, self.year);
    }
}