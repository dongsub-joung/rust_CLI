use std::fs::File;
use std::io::{self, Read, Write};
use std::fs;
use std::path::Path;

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

    pub fn visit_dirs(dir: &Path) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
    
                if path.is_dir() {
                    // If it's a directory, recurse into it
                    visit_dirs(&path)?;
                } else {
                    // If it's a file, perform the desired operation
                    println!("{}", path.display());
                    // Do something with the file path here
                }
            }
        }
        Ok(())
    }
}
