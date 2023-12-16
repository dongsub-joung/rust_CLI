pub struct Day{
    pub yyyy: u32,
    pub mm: u32,
    pub dd: u32,
    pub difference: u32,
}

#[warn(non_camel_case_types)]
pub struct DayOutPut{
    pub id: u32,
    pub yyyy: u32,
    pub mm: u32,
    pub dd: u32,
    pub difference: u32,
}

impl Day {
    pub fn new(yyyy: u32, mm: u32, dd:u32, difference: u32) -> Day{
        Self { yyyy, mm, dd, difference }
    }
}