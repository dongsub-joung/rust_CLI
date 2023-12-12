use std::fs::File;
use std::io::{self, Read, Write};


pub mod io_file{
    use super::*;
    
    pub fn selecteing_file(pwd: &str) -> io::Result<String> {
        let mut file_to_read = File::open(pwd)?;
        let mut content = String::new();
    
        file_to_read.read_to_string(&mut content)?;

        println!("File content copied successfully!");
    
        // content
        Ok(content)
    }

    pub fn saving_file(content: &[u8]) -> io::Result<()> {
        let mut file_to_write = File::create("output.txt")?;
    
        // Write the content to the new file
        file_to_write.write_all(content)?;
    
        println!("File content saved successfully!");
    
        Ok(())
    }
}
