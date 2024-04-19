# d a t a l o k

A fast and simple command line data locking tool requiring only a custom made password file as a cipher key. Capable of encrypting any data stored on disk and decrypting data to its original state.

**Encryption process**:
1. Encrypt the file contents using Chacha20poly1305 and save them to a file with an extension 'lok'.
2. Combine the nonce, salt used in the encryption, and the file extension.
3. Encrypt the combined values using stored binary data to secure them.
4. Encode everything to base64 using a custom engine, then convert to binary to increase the file size before compressing.
5. Delete the original file from the system.

**Decryption process**:
1. Decompress the data and decode it.
2. Extract the encrypted combined values, and decrypt them.
3. Extract the encrypted file contents.
4. Using the decrypted values to decrypt the extracted file contents and restore the file to its original state.

## Password File Examples
- A random stream of characters inside a file.
- The contents of a file already on your computer.
- Generate a unique cryptographic key to use inside a file.
- Append numbers or letters to an existing file to create a password file. With the ability to remove the appended characters after encryption, and only add them back before decryption.

<!-- ### Encryption 

- Chacha20poly1305 for encryption
- Password file as a cipher key
- New nonce and salt for each file processed
- Original file size and structure is obfuscated
- Encoding with custom base64 engine -->
<!-- - Encrypted text is converted to binary code and then compressed -->

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
***
[![Download](https://img.shields.io/badge/Download-v0.3-blue?style=flat)](https://github.com/0x7f9/datalok/releases/download/v0.3/datalok.exe)
