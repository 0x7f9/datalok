# d a t a l o k

Very fast and simple command line data locking tool requiring only a custom made password file as a cipher key. Capable of encrypting any data stored on disk and decrypting all data to its original state.

**A encrypted file contains**:
- File contents - Original file contents
- Combined values - File extension, nonce and salt
  
**How the process works**:
1. The ```File contents``` are encrypted and stored within ```file_name.lok```.
2. The nonce and salt used for encryption, and the file extension are combined.
3. The ```Combined values``` are encrypted using stored values within the binary's code, and stored in the new ```file_name.lok```.
4. The original file is removed from the system.

During decryption, the ```Combined values``` are decrypted first, then using these values to decrypt the ```File contents``` and restore the file to its orginal state.  

### Password File
> Needs to contain valid UTF-8 characters

**Examples incude**: 

- A random stream of characters inside a file.
- The contents of a file already on your computer.
- Generate a unique cryptographic key to use inside a file.
- Append numbers or letters to an existing file to create a password file. With the ability to remove the appended characters after encryption, and only add them back before decryption.

### Encryption 

- Chacha20poly1305 for encryption
- Password file as a cipher key
- New nonce and salt for each file processed
- Original file size and structure is obfuscated
- Encrypted text is converted to binary code and then compressed

## How to Use

```bash
# Build from source
# Build location: datalok/target/release/datalok
cd "download folder"/datalok
cargo build --release

# Example usage:
# datalok <file/folder> [-s] <-e | -d> [-r]

#  Flags:
#  -e Encryption 
#  -d Decryption 
#  -s Start a session
#  -r Recursively processes files in a directory

# Encrypt a file:
  datalok <file> -e
# Decrypt a folder recursively:
  datalok <folder> -d -r
# Start a session
  datalok -s
```
**Session features include**:  
- Storing the password file for the duration of the session.
- Capable of encrypting or decrypting files from any location on the system.
- Uses the system file explorer when selecting files or folders to process.
```bash
# Example session usage:
#  <e | d> [-r]

#  Commands:
#  e | encrypt - Encryption
#  d | decrypt - Decryption
#  h | help - Shows all commands
#  exit - Exits the session
  
#  Flags:
#  -r Recursively processes files in a directory

# Encrypt a folder recursively:
  e -r
# Decrypt a folder recursively:
  decrypt -r 
```

### Basic Examples
![datalock_example](https://github.com/0x7f9/datalok/assets/141240295/c39e547d-10f5-4216-8439-bb358a41d94f)  

![datalock_example2](https://github.com/0x7f9/datalok/assets/141240295/e9c82d99-b181-4804-820f-0001d3eb4d00)
***
[![Download](https://img.shields.io/badge/Download-v0.2-blue?style=flat)](https://github.com/0x7f9/datalok/releases/download/v0.2/datalok.exe)
