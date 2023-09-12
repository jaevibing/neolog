use std::{fs::{self, File}, env, path::Path, io::{self, Write}};

#[allow(non_camel_case_types)]
pub struct neologFile {
    file: File,
    logging_level: i32
}

pub fn init(file: String, level: Option<&str>) -> Result<neologFile, io::Error> {
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

    return Ok(neologFile{file, logging_level})
}

pub fn log(message: String, mut fileref: neologFile) -> Result<neologFile, io::Error> {
    match fileref.file.write_all(&message.as_bytes()) {
        Ok(_) => return Ok(fileref),
        Err(e) => return Err(e),
    }
}

pub fn debug(mut message: String, logfile: neologFile) -> Result<neologFile, io::Error> {
    let line_number = line!();
    message = format!("DEBUG (line {}): {}", line_number, message);
    if logfile.logging_level <= 2 {
        match log(message, logfile){
            Ok(o) => Ok(o),
            Err(e) => Err(e)
        }
    }
    else {
        Ok(logfile)
    }
}

pub fn critical(mut message: String, logfile: neologFile) -> Result<neologFile, io::Error> {
    let line_number = line!();
    message = format!("## CRITICAL ERROR at line {} ##\n{}\n###   ###", line_number, message);
    if logfile.logging_level <= 2 {
        match log(message, logfile){
            Ok(o) => Ok(o),
            Err(e) => Err(e)
        }
    }
    else {
        Ok(logfile)
    }
}

pub fn error(mut message: String, logfile: neologFile) -> Result<neologFile, io::Error> {
    let line_number = line!();
    message = format!("## ERROR at line {} ##: {}", line_number, message);
    if logfile.logging_level <= 2 {
        match log(message, logfile){
            Ok(o) => Ok(o),
            Err(e) => Err(e)
        }
    }
    else {
        Ok(logfile)
    }
}