# d a t a l o k

<<<<<<< HEAD
A fast and simple CLI (command line interface) data locking tool requiring only a custom made password file as a cipher key. Capable of encrypting any data stored on disk and decrypting all data to its original state.
=======
Very fast and simple command line data locking tool requiring only a custom made password file as a cipher key. Capable of encrypting any data stored on disk and decrypting all data to its original state.
>>>>>>> 4a5048c (Added sessions)

**A encrypted file contains**:
- File contents - Original file contents
- Combined values - File extension, nonce and salt
  
**How the process works**:
1. The ```File contents``` are encrypted and stored within ```file_name.lok```.
2. The nonce and salt used for encryption, and the file extension are combined.
3. The ```Combined values``` are encrypted using stored values within the binary's code and then stored within the new ```file_name.lok```.
4. The original file is removed from the system.

During decryption, the ```Combined values``` are decrypted first, then using these values to decrypt the ```File contents``` and restore the file to its orginal state.  

### Password File
> Needs to contain valid UTF-8 characters

**Examples incude**: 

- A random stream of characters inside a file.
- The contents of a file already on your computer.
- Generate a unique cryptographic key to use inside a file.
- Append numbers or letters to an existing file to create a password file. With the ability to remove the appended characters after encryption, and only add them back before decryption.

<<<<<<< HEAD
### Encryption
=======
### Encryption 
>>>>>>> 4a5048c (Added sessions)

- Chacha20poly1305 for encryption
- Password file as a cipher key
- New nonce and salt for each file processed
- Original file size and structure is obfuscated
- Encrypted text is converted to binary code and then compressed

<<<<<<< HEAD
### How to use

```bash
# build the executable file
cd "download folder"/datalok
cargo build
cd datalok/target/debug
datalok.exe

# Options:
  -e Encryption
  -d Decryption
  -r All files in a directory

# Usage:
  datalok <file or folder> <-e -d> [-r]
```
[![Download](https://img.shields.io/badge/Download-v1.0.0-blue?style=flat)](https://github.com/0x7f9/datalok/releases/download/v1.0.0/datalok.exe)  

### CLI Examples
=======
## How to Use
**Session features include**:  
- Stores the password file for the duration of the session.
- Capable of encrypting or decrypting files from any location on the system.
- Uses system file explorer for selecting files or folders to process.
```bash
# Build from source
# Build location: datalok/target/release/datalok.exe
cd "download folder"/datalok
cargo build --release

# Example usage:
  datalok <file/folder> [-s] <-e | -d> [-r]

  Flags:
  -e Encryption 
  -d Decryption 
  -s Start a session
  -r Recursively processes files in a directory

# Encrypt a file:
  datalok <file> -e
# Decrypt a folder recursively:
  datalok <folder> -d -r

# How to use sessions
# Start a session
datalok.exe -s

# Example session usage:
  <e | d> [-f]

  Commands:
  e | encrypt - Encryption
  d | decrypt - Decryption
  h | help - Shows all commands
  exit - Exits the session
  
  Flags:
  -r Recursively processes files in a directory

# Encrypt all files in a directory:
  e -r
```

### Basic Examples
>>>>>>> 4a5048c (Added sessions)
![datalock_example](https://github.com/0x7f9/datalok/assets/141240295/c39e547d-10f5-4216-8439-bb358a41d94f)  

![datalock_example2](https://github.com/0x7f9/datalok/assets/141240295/e9c82d99-b181-4804-820f-0001d3eb4d00)
***
<<<<<<< HEAD
### Future ideas 

- Customer hex encoder for converting the stored values inside the binary (Try using the password file as encoder key).
- Custom binary representation encoder (Maybe)
=======
[![Download](https://img.shields.io/badge/Download-v1.0.0-blue?style=flat)](https://github.com/0x7f9/datalok/releases/download/v1.0.0/datalok.exe)
>>>>>>> 4a5048c (Added sessions)
