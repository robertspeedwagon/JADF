use std::fs;
use std::path::Path;
use std::env;
use std::process;
use regex::Regex;
mod jadf;
use crate::jadf::jadf::Field;
use crate::jadf::jadf::read;
struct Field {
    variable_name: String,
    value_string: String,
}

fn main() {
    println!("JADF (Just Another Data Format) test program!");
    let program_args: Vec<String> = env::args().collect();
    let path_to_file = Path::new(&program_args[1]);
    let file_extension = path_to_file.extension();
    let assignment_operator = Regex::new(
        r#"[[:space:]]*[[:print:]]+[[:space:]]*<-[[:space:]]*"[[:print:]]+"[[:space:]]*"#
    ).unwrap();
    match file_extension {
        None => {
            eprintln!("no extension for the file; aborting.");
            process::exit(1);
        },
        Some(os_str) => {
            match os_str.to_str() {
                Some("jadf") => { //if the extension of the file is .jadf
                    //the entire program lives here
                    //opens an existing file for reading and writing:r
                    let file: Result<fs::File, std::io::Error>  = fs::OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(false)
                                    .open(&path_to_file);
                    let _file = match file {
                        Ok(fl) => fl,
                        Err(_) => {
                            eprintln!("error while reading the file; aborting.");
                            process::exit(1);
                        },
                    };
					let file_contents: String = match fs::read_to_string(&path_to_file) {
						Ok(s) => s, 
						Err(_) => {
							eprintln!("error while reading the file; aborting.");
							process::exit(1);
						}
					};
                   let raw_fields: Vec<String> = read::extract_lines(&file_contents);
                   let mut fields: Vec<Field> = Vec::new();
                    for i in 0..raw_fields.len() {
                        let temp_var_name = match read::extract_var_name(&raw_fields[i], &assignment_operator) {
                            Some(s) => s,
                            None => {
                                eprintln!("error while parsing the file at line {}:\n\tincorrect syntax; aborting.", i);
                                process::exit(1);
                            }
                        };
                        let temp_var_value = match read::extract_var_value(&raw_fields[i], &assignment_operator) {
                            Some(s) => s,
                            None => {
                                eprintln!("error while parsing the file at line {}:\n\tincorrect syntax; abortng.", i);
                                process::exit(1);
                            }
                        };
                        fields.push(Field {
                            variable_name: temp_var_name,
                            value_string: temp_var_value,
                        });
                    }
                    //now you can do what you want with the contents of the
                    //fields vector. i'm just going to print them to show that
                    //it works:
                    for i in 0..fields.len() {
                        println!("{} = {}", fields[i].variable_name, fields[i].value_string);
                    }
                },
                _ => {
                    eprintln!("incorrect file extension; aborting.");
                    process::exit(1);
                }
            }
        }
    }
}
