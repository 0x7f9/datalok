use std::{env, io::{self, Write}, path::{Path, PathBuf}, process::exit};

mod file_mod;
mod cipher_mod;
mod encode_mod;
mod compress_mod;
mod engine_mod;

struct Options {
    file: PathBuf,
    session: String,
    mode: String,
    option: String,
}

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
}


fn main() {
    let arg_file = env::args().nth(1).unwrap_or_default();
    let arg_session = env::args().nth(1).unwrap_or_default();
    let arg_mode = env::args().nth(2).unwrap_or_default();
    let arg_option = env::args().nth(3).unwrap_or_default();

    let args = Options {
        file: std::path::PathBuf::from(arg_file),
        session: arg_session,
        mode: arg_mode,
        option: arg_option,
    };

    // start and store a cache of the base64 engine
    engine_mod::get_engine(true);

    if args.session == "-s" {
        // start a constant session 
        handle_session();
    } else {
        if !args.file.exists() {
            print_options("Error: Can not find file, folder or session");
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
    
    let mut files = Vec::new();
    if args.option == "-r" {
        files = file_mod::read_dir(&args.file);
    } else {
        if !args.file.is_file() {
            print_options("Error: A directory can be encrypted only inside another directory");
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


fn handle_session() {
    fn print_commands() {
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
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().split(' ').map(|x |x.to_string()).collect::<Vec<String>>()
    }

    println!("Session started - Type help or h");
    println!("Input password file for session");
    let password = file_mod::get_password().clone();
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
                println!("Input a file to encrypt\n");
                let file = &file_mod::get_file();
                if !file.exists() {
                    continue;
                }
                encrypt(file, &password, index);

            } else if args.get(0).unwrap() == "-r" {
                println!("Input a folder to encrypt\n");
                let dir = file_mod::get_folder();
                if !dir.exists() {
                    continue;
                }

                let files = file_mod::read_dir(&dir);
                let send_password = password.clone();
                for file in files {
                    index += 1;
                    encrypt(&file, &send_password, index)
                } 
            
            } else {
                println!("Error: Unknown command - Type help or h");
            }

        } else if input == "d" || input == "decrypt" {
            let mut index = 0;
            if args.len() == 0 {
                println!("Input a file to decrypt\n");
                let file = &file_mod::get_file();
                if !file.exists() {
                    continue;
                }
                decrypt(file, &password, index);

            } else if args.get(0).unwrap() == "-r" {
                println!("Input a folder to decrypt\n");
                let dir = file_mod::get_folder();
                if !dir.exists() {
                    continue;
                }
                let files = file_mod::read_dir(&dir);
                let send_password = password.clone();
                for file in files {
                    index += 1;
                    decrypt(&file, &send_password, index)
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
            println!("Error: Unknown command - Type help or h\n");
        }
        println!("")
    }
}


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

    encode_mod::to_base64(file);
    encode_mod::to_binary(file);
    compress_mod::compress(file);
}


fn decrypt(file: &Path, password: &str, index: i32) {
    let check_file = file.to_string_lossy();
    if !check_file.ends_with("lok") {
        println!("({index}) Error: File is not encrypted");
        return;
    }

    compress_mod::decompress(file);
    encode_mod::from_binary(file);
    encode_mod::from_base64(file);

    if !cipher_mod::cipher::decrypt_contents(file, password, index) {
        // println!("DEBUG: Compressing file again...");
        encode_mod::to_base64(file);
        encode_mod::to_binary(file);
        compress_mod::compress(file);
    }
}

