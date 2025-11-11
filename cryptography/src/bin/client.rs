use std::net::TcpStream;
use std::io::{Read, Write};

use aes_gcm::aes::cipher;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};
use std::fs;



fn encrypt_file(cipher: &Aes256Gcm, file_path: &str) {

    

}





fn main() -> std::io::Result<()> {

    let mut stream = match TcpStream::connect("127.0.0.1:8080") {

        Ok(stream) => {
            println!("Connected to the server!");
            stream
        } 
        Err(_) => {
            println!("Couldn't connect to server...");
            return Ok(())
        } 
     };

    stream.write(&[1])?;

    let msg_package = stream.read(&mut [0; 128])?;
    println!("{}", msg_package);

    // create encryption key -- same used througout
    let key =  Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);

    

    Ok(())

}
