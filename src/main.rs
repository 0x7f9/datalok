use std::{env, path::{Path, PathBuf}};

mod file_mod;
mod cipher_mod;
mod binary_mod;
mod compress_mod;
// mod sensitive;

struct Options {
    file: PathBuf,
    mode: String,
    option: String,
}

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
}


fn main() {
    let arg_file = env::args().nth(1).unwrap_or_default();
    let arg_mode = env::args().nth(2).unwrap_or_default();
    let arg_option = env::args().nth(3).unwrap_or_default();

    if arg_file.len() == 0 {
        print_options();
        return;
    }
    
    let args = Options {
        file: std::path::PathBuf::from(arg_file),
        mode: arg_mode,
        option: arg_option,
    };

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
    let mut files = Vec::new();
    if args.option == "-r" {
        files = file_mod::read_dir(&args.file);
    } else {
        if !args.file.is_file() {
            println!("Error: A directory can be encrypted only inside another directory");
            print_options();
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

