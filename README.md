# neolog
![Crates.io (latest)](https://img.shields.io/crates/dv/neolog) ![Crates.io](https://img.shields.io/crates/l/neolog) ![Crates.io](https://img.shields.io/crates/v/neolog) [![Source](https://github.com/jaevibing/neolog/actions/workflows/rust.yml/badge.svg)](https://github.com/jaevibing/neolog/actions/workflows/rust.yml)


A new, simple, and quick approach to log files in Rust.
## Version
neolog is currently in version 0.0.3, many features are yet to be implemented effectively and may not work as expected. If you encounter an error whilst using this program, open an error in Github.
## Usage
#### Main Functions
Import neolog into a project with:
```rust
extern crate neolog;
```
Initialise neolog with a variable like so:
```rust
let logger = neolog::init("path/to/file", 
                        "logging_level",
                        save_program_output: true (default: false), 
                        clear_on_open: true (default: true));
```
You will need to provide a path to where you want the log file to be saved. You can optionally provide the logging level and a boolean as to whether or not you want to save the programs output to the logging file.

There are several logging levels and if one is not provided it will default to the 'debug' level, these leves indicate what will be outputted to the log file. The accepted level inputs are: `all, verbose, debug, error, critical`.

save_program_output is a boolean value that defines whether or not the output of the program should be written to the log file. This, by default, is turned off and will default to the false value if a value is not provided.

clear_on_open defines whether the log file should be cleared when opened. If this value is not provided it will default to true, clearing the log file on each

To log you simply need to call the logging function from the logging variable you defined with `neolog::init()`. There are multiple functions to log to a file in neolog.
```rust
logger.info("message here");
logger.debug("debug info here");
logger.error("error message here");
logger.critical("critical error here");
```
Each will provide different headers to the message. Info will provide no header, debug will provide some text at the beginning reading `DEBUG:`, error will do the same with `## ERROR ##:` and critical will make the error stand out by giving it a header and footer and a whole line like so
```
### CRITICAL ERROR ###

message here

##### #####
```
#### Additional Functions
You can clear the log file at any time with:
```rust
logger.clear();
```
If needed, you can delete the file and the logger object with:
```rust
logger.delete();
```
You can change the log file with:
```rust
logger.change("path/to/new/file", 
                clear_on_open: true (default: true),
                delete_original_file: false (default: false));
```
You can add a new log file to log to two files at once with: (not working!!)
```rust
logger.add("path/to/second/file", 
            clear_on_open: true (default: true));
```
You can remove a file from the logger object with (this will return an error if there is only one file in the object): (not working!!)
```rust
logger.remove("path/to/file",
                delete_on_close: false (default: false));
```
## Roadmap
There are a number of features I'm considering adding before I can officiate a full v1.0 release. Here is a short list of features, if you have any desired features open an error with a tag that indicated you are suggesting a feature.
* Asynchronous logging
* Logging to multiple files at once
* Logging to multiple files with different logging levels
* Extension support