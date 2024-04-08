use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit};
use ring::{digest::SHA512_256_OUTPUT_LEN, pbkdf2};
use std::num::NonZeroU32;

static ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;

pub mod cipher {
    // This class deals with its own file handling including reading, 
    // writing and creating files while encrypting and decrypting contents.
    use chacha20poly1305::{aead::{Aead, AeadCore, OsRng},ChaCha20Poly1305, Nonce};
    use std::{fs::{self, File, OpenOptions}, io::{Read, Write}, path::Path, process::exit};
    use rand::RngCore;
    
    use crate::{cipher_mod::gen_key, file_mod};
    use super::stored_values;

    pub fn encrypt_contents(file: &Path, contents: &[u8], password: &str, index: i32) -> bool {
        fn exclude_value(random: &mut OsRng, value: &mut [u8]) {
            // prevents the byte value of "58" being generated within the salt or nonce.
            // The byte value "58" represents ":". i use ":" as a splitter when joining (concatenating) stored values,
            // allowing for splitting of values during decryption.
            let exclude = 58;
            for byte in value {
                while byte == &exclude {
                    *byte = random.next_u32() as u8;
                }
            }
        }

        let mut random = OsRng;
        let mut salt = [0u8; 16];
        random.fill_bytes(&mut salt);
        exclude_value(&mut random, &mut salt);

        let cipher = gen_key(password, &salt);
        let mut nonce = ChaCha20Poly1305::generate_nonce(&mut random);
        exclude_value(&mut random, &mut nonce);

        let encrypted = cipher.encrypt(&nonce, contents).map(|encrypted| {
            let mut out_file = file_mod::create_file(file, "lok".to_string());
            out_file.write_all(&(encrypted.len() as u64).to_le_bytes()).unwrap();
            out_file.write_all(&encrypted).unwrap(); 
            
            let display = file.file_name().unwrap().to_string_lossy().to_string();
            println!("({index}) * Encrypted file - {display}");
            fs::remove_file(file).unwrap();
            true
        }).unwrap_or_else(|err| {
            println!("({index}) Error: Encrypting file contents - {err}");
            false
        });
        encrypt_values(file, &nonce, &salt);
        encrypted
    }


    fn encrypt_values(file: &Path, nonce: &[u8], salt: &[u8]) {
        let (stored_nonce, stored_salt, stored_password) = &stored_values();
        let stored_password = std::str::from_utf8(stored_password).unwrap();
        let file_extension = file.extension().unwrap().to_string_lossy();
        let extension = file_extension.as_bytes();
        
        let split = ":".as_bytes();
        let values = [nonce, split, salt, split, extension].concat();

        let cipher = gen_key(stored_password, stored_salt);
        cipher.encrypt(Nonce::from_slice(stored_nonce), values.as_ref()).map(|encrypt_values| {
            let open_file = file.with_extension("lok");
            let mut out_file = OpenOptions::new().append(true).open(open_file).unwrap();
            out_file.write_all(&(encrypt_values.len() as u64).to_le_bytes()).unwrap();
            out_file.write_all(&encrypt_values).unwrap();
        }).unwrap_or_else(|err| {
            println!("Error: Encrypting salt, nonce, and file extension - {err}");
            exit(1)
        });
    }

    
    fn decrypt_values(values: Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let (stored_nonce, stored_salt, stored_password) = &stored_values();
        let stored_password = std::str::from_utf8(stored_password).unwrap();

        let cipher = gen_key(stored_password, stored_salt);
        let decrypt_values = cipher.decrypt(Nonce::from_slice(stored_nonce), values.as_ref()).map(|decrypt_values| {
            let values: Vec<&[u8]> = decrypt_values.split(|&s| s == "::".as_bytes()[0]).collect();

            if values.len() != 3 {
                println!("Error: File is missing important values");
                println!("Error: File can not be decrypted");
                exit(1)
            }

            let nonce = values[0].to_vec();
            let salt = values[1].to_vec();
            let file_extension = values[2].to_vec();
            (nonce, salt, file_extension)
        }).unwrap_or_else(|err| {
            println!("Error: Decrypting salt, nonce, and file extension - {err}");
            exit(1)
        });
        decrypt_values
    }
    

    pub fn decrypt_contents(file: &Path, password: &str, index: i32) -> bool {
        let mut input_file = File::open(file).unwrap();

        // find encrypted contents block for decryption
        let mut ciphertext_len = [0u8; 8];
        input_file.read_exact(&mut ciphertext_len).unwrap();
        let bytes_ciphertext = u64::from_le_bytes(ciphertext_len) as usize;
        let mut ciphertext = vec![0u8; bytes_ciphertext];
        input_file.read_exact(&mut ciphertext).unwrap();
        
        // find encrypted values block containing (nonce, salt and file extension) for decryption
        let mut values_len = [0u8; 8];
        input_file.read_exact(&mut values_len).unwrap();
        let bytes_values = u64::from_le_bytes(values_len) as usize;
        let mut values = vec![0u8; bytes_values];
        input_file.read_exact(&mut values).unwrap();

        let (nonce, salt, file_extension) = &decrypt_values(values);
        let cipher = gen_key(password, salt);
        let decrypted = cipher.decrypt(Nonce::from_slice(nonce), ciphertext.as_ref()).map(|decrypted| {
            let file_extension = String::from_utf8_lossy(file_extension).into_owned();
            let display = file.with_extension(&file_extension).file_name().unwrap().to_string_lossy().to_string();
           
            let mut create_file = file_mod::create_file(file, file_extension);
            create_file.write_all(&decrypted).unwrap();
            println!("({index}) + Decrypted File - {display}");
            fs::remove_file(file).unwrap();
            true
        }).unwrap_or_else(|_| {
            println!("({index}) Error: Wrong password file");
            encrypt_values(file, nonce, salt);
            false
        });
        decrypted
    }
}


fn gen_key(password: &str, salt: &[u8]) -> ChaCha20Poly1305 {
    let key = &derive_key(password, salt);
    let key = Key::from_slice(key);
    ChaCha20Poly1305::new(key)
}


fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    // chacha20poly1305 accepts a 256-bit key size (32 bytes)
    // using sha512 hashing converted to 32 bytes in length for the key 
    let mut key = [0u8; SHA512_256_OUTPUT_LEN];
    let iter = NonZeroU32::new(100_000).unwrap();
    let password = password.as_bytes();
    pbkdf2::derive(ALG, iter, salt, password, &mut key);
    key
}


fn stored_values() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    // hard coded values used for encrypting and decrypting the
    // randomly generated nonce, salt and file extension before writing to the file.
    
    // most likely a better way to do this but i wanted the executable to be as portable as possible, 
    // and having config files limits that.
    // values were generated using randomness from the operating system.
    let hex1 = "9e567349ffcd0b5b65bae05e";
    let hex2 = "e974a91f794823f5117cda422a018a22";
    let hex3 = "70617373776f7264";
    let stored_nonce = hex::decode(hex1).unwrap();
    let stored_salt = hex::decode(hex2).unwrap();
    let stored_password = hex::decode(hex3).unwrap();
    (stored_nonce, stored_salt, stored_password)
}

