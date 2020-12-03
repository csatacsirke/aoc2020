use std::path::Path;
use std::fs;
use thiserror::Error;

//extern crate regex;
use regex::Regex;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
   
    #[error(transparent)]
    String(#[from] std::string::FromUtf8Error),
}

pub fn read_input<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, Error> {
    let file_contents = fs::read(file_path)?;
    let file_contents = String::from_utf8(file_contents)?;

    //let input_elements = file_contents.split(",");
    let regex = Regex::new(r"\w+").unwrap();
    let strings = regex.find_iter(&file_contents)
        .map(|m| String::from(m.as_str()))
        .collect::<Vec<String>>();
    //let matches = regex.split(&file_contents).into_iter().collect();
    //let input_elements = file_contents.split()
    //let strings = input_elements.map(String::from).collect::<Vec<String>>();
    
    Ok(strings)
}
