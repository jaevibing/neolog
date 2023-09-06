use std::{fs::{self, File}, env, path::Path, io::{self, Write}};

pub fn init(file: String) -> Result<File, io::Error> {
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

    return Ok(file)
}

pub fn log(message: String, mut logfile: File) -> Result<File, io::Error> {
    match logfile.write_all(&message.as_bytes()) {
        Ok(_) => return Ok(logfile),
        Err(e) => return Err(e),
    }
}