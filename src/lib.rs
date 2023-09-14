use std::{fs::{self, File}, env, path::Path, io::{self, Write}};

#[allow(non_camel_case_types)]
pub struct neologObject {
    file: File,
    logging_level: i32
}

impl neologObject {
    fn log(&mut self, message: String){
        let _ = self.file.write_all(&message.as_bytes());
    }
    
    pub fn debug(&mut self, mut message: String){
        message = format!("DEBUG: {}", message);
        if self.logging_level <= 2 {
            self.log(message);
        }
    }
    
    pub fn critical(&mut self, mut message: String){
        message = format!("## CRITICAL ERROR ##\n{}\n###   ###", message);
        if self.logging_level <= 2 {
            self.log(message);
        }
    }
    
    pub fn error(&mut self, mut message: String) {
        message = format!("## ERROR ##: {}", message);
        if self.logging_level <= 2 {
            self.log(message);
        }
    }

    pub fn info(&mut self, message: String) {
        if self.logging_level <= 2 {
            self.log(message);
        }
    }
}

pub fn init(file: String, level: Option<&str>) -> Result<neologObject, io::Error> {
    let logfile = Path::new(&env::current_dir().unwrap()).join(file);
    match fs::metadata(logfile.clone()){
        Ok(_) => (),
        Err(_) => {
            match fs::File::create(logfile.clone()){
                Ok(_) => (),
                Err(e) => {
                    eprintln!("neolog error: {}", e);
                    return Err(e)
                }
            }
        },
    };

    let file = match fs::OpenOptions::new().append(true).open(logfile.clone()){
        Ok(f) => f,
        Err(e) => {
            eprintln!("neolog error: {}", e);
            return Err(e)
        }
    };

    let logging_level: i32 = match level.unwrap_or("debug").to_lowercase().as_str() {
        "debug" => 2, // only debugging messages called by debug() and any errors
        "critical" => 4, // only errors called by critical()
        "error" => 3, // only errors called by error() or critical()
        "all" => 1, // all messages specified by log() or any other function
        "verbose" => 1,
        _ => 2, // default to debug if any other value is presented
    };

    return Ok(neologObject{file, logging_level})
}