<<<<<<< HEAD
use std::{env, path::{Path, PathBuf}};
=======
use std::{env, io::{self, Write}, path::{Path, PathBuf}, process::exit};
>>>>>>> 4a5048c (Added sessions)

mod file_mod;
mod cipher_mod;
mod binary_mod;
mod compress_mod;
<<<<<<< HEAD
// mod sensitive;

struct Options {
    file: PathBuf,
=======

struct Options {
    file: PathBuf,
    session: String,
>>>>>>> 4a5048c (Added sessions)
    mode: String,
    option: String,
}

<<<<<<< HEAD
fn print_options() {
    println!("
    Options:
    -e Encryption
    -d Decryption
    -r All files in a directory
    
    Usage:
    datalok <file or folder> <-e -d> [-r]

    Example:
    datalok.exe folder -e -r
    datalok.exe test.txt -e 
    datalok.exe test.lok -d
    ")
=======
fn print_options(error: &str) {
    println!("
    {error}
    
    Options:
    -e Encryption
    -d Decryption
    -s Start a session
    -r Recursively processes files in a directory
    
    Usage:
    datalok <file or folder> [-s] <-e -d> [-r]")
>>>>>>> 4a5048c (Added sessions)
}


fn main() {
    let arg_file = env::args().nth(1).unwrap_or_default();
<<<<<<< HEAD
    let arg_mode = env::args().nth(2).unwrap_or_default();
    let arg_option = env::args().nth(3).unwrap_or_default();

    if arg_file.len() == 0 {
        print_options();
        return;
    }
    
    let args = Options {
        file: std::path::PathBuf::from(arg_file),
=======
    let arg_session = env::args().nth(1).unwrap_or_default();
    let arg_mode = env::args().nth(2).unwrap_or_default();
    let arg_option = env::args().nth(3).unwrap_or_default();

    let args = Options {
        file: std::path::PathBuf::from(arg_file),
        session: arg_session,
>>>>>>> 4a5048c (Added sessions)
        mode: arg_mode,
        option: arg_option,
    };

<<<<<<< HEAD
    if !args.file.exists() {
        println!("Error: Can not find file or folder");
        print_options();
        return;
    }

    println!("Input password file for session");
    let password = &file_mod::get_password();
    println!("Password file loaded\n");

    // testing 
    // if args.mode == "" || args.mode == "" {
    //     unsafe {
    //     sensitive::SAFE_MODE = true
    //     }    
    // }

    // println!("{0}", sensitive::Sensitive(password.clone()));
    check_options(args, password);
}


fn check_options(args: Options, password: &str) {
=======
    if args.session == "-s" {
        // start a constant session 
        handle_session();
    } else {
        if !args.file.exists() {
            print_options("Error: Can not find file or folder");
            return;
        }
    
        if args.mode.len() == 0 {
            print_options("Error: No encryption mode chosen");
            return;
        }
        check_options(args);
    }
}


fn check_options(args: Options) {
    println!("Input password file");
    let password = &file_mod::get_password();
    
>>>>>>> 4a5048c (Added sessions)
    let mut files = Vec::new();
    if args.option == "-r" {
        files = file_mod::read_dir(&args.file);
    } else {
        if !args.file.is_file() {
<<<<<<< HEAD
            println!("Error: A directory can be encrypted only inside another directory");
            print_options();
=======
            print_options("Error: A directory can be encrypted only inside another directory");
>>>>>>> 4a5048c (Added sessions)
            return;
        }
        files.push(args.file);
    }

    let mut index = 0;
    for file in &files {
        index += 1;
        if args.mode == "-e" {
            encrypt(file, password, index);
        } else {
            decrypt(file, password, index);
        }
    }
}


<<<<<<< HEAD
=======
fn handle_session() {
    fn print_commands() {
    // internal function
    println!("
    Commands:   
        e | encrypt - Encryption
        d | decrypt - Decryption
        h | help - Shows all commands
        cls - Clear the console
        exit - Exits the session
        
        Flags:
        -r Recursively processes files in a directory")
    }

    fn get_input() -> Vec<String> {
    // internal function
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // split the reponse on every space map, each input to a string and collect in a vec 
    let input = input.trim().split(" ").map(|x |x.to_string()).collect::<Vec<String>>();
    input
    }

    println!("Session started - Type help or h");
    println!("Input password file for session");
    let password = &file_mod::get_password();
    loop {
        print!(">>:: "); 
        io::stdout().flush().unwrap(); 

        let input = get_input();
        let (input, args) = (&input[0], &input[1..]);
        if input == "exit" {
            println!("Exiting session");
            exit(1);
        } else if input == "e" || input == "encrypt" {
            let mut index = 0;
            if args.len() == 0 {
                println!("Input a file to encrypt");
                let file = &file_mod::get_file();
                if !file.exists() {
                    continue;
                }
                println!("");
                encrypt(file, password, index);

            } else if args.get(0).unwrap() == "-r" {
                println!("Input a folder to encrypt");
                let dir = file_mod::get_folder();
                if !dir.exists() {
                    continue;
                }
                println!("");
                let files = file_mod::read_dir(&dir);
                for file in &files {
                    index += 1;
                    encrypt(file, password, index);
                }
            
            } else {
                println!("Error: Unknown command - Type help or h");
            }

        } else if input == "d" || input == "decrypt" {
            let mut index = 0;
            if args.len() == 0 {
                println!("Input a file to decrypt");
                let file = &file_mod::get_file();
                if !file.exists() {
                    continue;
                }
                println!("");
                decrypt(file, password, index);

            } else if args.get(0).unwrap() == "-r" {
                println!("Input a folder to decrypt");
                let dir = file_mod::get_folder();
                if !dir.exists() {
                    continue;
                }
                println!("");
                let files = file_mod::read_dir(&dir);
                for file in &files {
                    index += 1;
                    decrypt(file, password, index);
                }

            } else {
                println!("Error: Unknown command - Type help or h");
            }

        } else if input == "h" || input == "help" {
            print_commands();
        } else if input == "cls" {
            for _ in 0..150 {
                // i guess
                println!();
            }
        } else {
            if input.len() == 0 {
                continue;
            }
            println!("Error: Unknown command - Type help or h");
        }
        println!("");
    }
}


>>>>>>> 4a5048c (Added sessions)
fn encrypt(file: &Path, password: &str, index: i32) {
    let check_file = file.to_string_lossy();
    if check_file.ends_with("lok") {
        println!("({index}) Error: File is already encrypted");
        return;
    }

    let contents = &file_mod::read_file(file);
    if contents.len() == 0 {
        println!("({index}) Error: No file contents found");
        return;
    }

    if !cipher_mod::cipher::encrypt_contents(file, contents, password, index){
        file_mod::write_vector(file, contents);
        // println!("DEBUG: Writing file contents back...");
        return;
    }

    binary_mod::to_binary(file);
    compress_mod::compress(file);
}


fn decrypt(file: &Path, password: &str, index: i32) {
    let check_file = file.to_string_lossy();
    if !check_file.ends_with("lok") {
        println!("({index}) Error: File is not encrypted");
        return;
    }

    compress_mod::decompress(file);
    binary_mod::from_binary(file);

    if !cipher_mod::cipher::decrypt_contents(file, password, index) {
        // println!("DEBUG: Compressing file again...");
        binary_mod::to_binary(file);
        compress_mod::compress(file);
    }
}

