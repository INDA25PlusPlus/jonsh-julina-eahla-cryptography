use crypto::MerkleTree;
use serde_json;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::cell::RefCell;

thread_local! {
    pub static MERKLE_TREE: RefCell<MerkleTree> = RefCell::new(MerkleTree::new());
}

fn add_node(file_id: u64, data: Vec<u8>) {
    MERKLE_TREE.with(|tree| {
        tree.borrow_mut().add_leaf_node(data, file_id);
    })
}

#[derive(serde::Deserialize, Debug)]
struct Metadata {
    file_id: u64,
}

fn read_tcp_message(stream: &mut TcpStream) -> std::io::Result<()> {
    // Format på meddelande:
    // [metadata length: 4 bytes]
    // [metadata JSON]
    // [ciphertext length: 4 bytes]
    // [ciphertext bytes]  <-- encrypted nonce + filename + file content + 16-byte tag
    let mut metadata_len_bytes = [0u8; 4]; // 4 byte buffer
    stream.read_exact(&mut metadata_len_bytes)?;

    let metadata_len = u32::from_be_bytes(metadata_len_bytes);

    let mut metadata_bytes = vec![0u8; metadata_len as usize];
    stream.read_exact(&mut metadata_bytes)?;

    // metadata = {file_id: _, ...}
    let metadata_json = String::from_utf8_lossy(&metadata_bytes);
    let metadata: Metadata = serde_json::from_str(&metadata_json)?;

    let file_id = metadata.file_id;

    let mut ciphertext_len_bytes = [0u8; 4];
    stream.read_exact(&mut ciphertext_len_bytes)?;

    let ciphertext_len = u32::from_be_bytes(ciphertext_len_bytes);

    let mut ciphertext_bytes = vec![0u8; ciphertext_len as usize];
    stream.read_exact(&mut ciphertext_bytes)?;


    println!("Successfully received encrypted file {} from client\n\n", file_id);

    // Spara {file_id: ciphertext_bytes} på nåt smart sätt

    // Spara i merkelträd!
    // add to merkelträd
    add_node(file_id, ciphertext_bytes);

    // eller hämta från merkelträd

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        read_tcp_message(&mut stream)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to listen on port --");
    println!("Listening on port --");

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
