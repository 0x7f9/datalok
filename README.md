# d a t a l o k

A fast and simple CLI (command line interface) data locking tool requiring only a custom made password file as a cipher key. Capable of encrypting any data stored on disk and restoring all data to original state.

### Overview

Datalok generates random values (a nonce and salt) for each file encrypted. These values along with the file extension, are combined and encrypted using stored values within the binary's code.

**The encrypted file contains**:
- File contents
- Combined values
  
**How the encryption process works**:
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

### Encryption

- Chacha20poly1305 for encryption
- Password file as a cipher key
- New nonce and salt for each file processed
- Original file size and structure is obfuscated
- Encrypted text is converted to binary code and then compressed

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
### CLI Examples
![datalock_example](https://github.com/0x7f9/datalok/assets/141240295/c39e547d-10f5-4216-8439-bb358a41d94f)  

![datalock_example2](https://github.com/0x7f9/datalok/assets/141240295/e9c82d99-b181-4804-820f-0001d3eb4d00)
***
### Future ideas 

- Customer hex encoder for converting the stored values inside the binary (Try using the password file as encoder key).
- Custom binary representation encoder (Maybe)
