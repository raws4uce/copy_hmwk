use aes::cipher::{generic_array::GenericArray, BlockEncrypt};
use aes::cipher::{BlockDecrypt, KeyInit};
use aes::Aes256;
use base64::{engine, Engine};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

const KEY_SIZE: usize = 32;

pub fn generate_key() -> [u8; KEY_SIZE] {
    let mut key = [0u8; KEY_SIZE];
    rand::thread_rng().fill(&mut key[..]);
    key
}

fn encrypt_data(key: &[u8], data: &[u8]) -> Result<String, Box<dyn Error>> {
    let cipher = Aes256::new(&GenericArray::from_slice(key));
    let mut buffer = data.to_vec();
    let padding = 16 - (buffer.len() % 16);
    buffer.extend(vec![padding as u8; padding]);

    for chunk in buffer.chunks_mut(16) {
        cipher.encrypt_block(&mut GenericArray::from_mut_slice(chunk));
    }

    Ok(engine::general_purpose::STANDARD.encode(&buffer))
}

fn decrypt_data(key: &[u8], encode_data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Aes256::new(&GenericArray::from_slice(key));
    let data = engine::general_purpose::STANDARD.decode(encode_data)?;

    let mut buffer = data;
    for chunk in buffer.chunks_mut(16) {
        cipher.decrypt_block(&mut GenericArray::from_mut_slice(chunk));
    }

    let padding = buffer.last().unwrap_or(&0) & 0xFF;
    let end = buffer.len() - padding as usize;

    Ok(buffer[..end].to_vec())
}

pub fn encrypt_file(file_path: &str, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let encrypted_data = encrypt_data(key, &data)?;
    let mut file = File::create(file_path)?;
    file.write_all(encrypted_data.as_bytes())?;
    Ok(())
}

pub fn decrypt_file(file_path: &str, key: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut encoded_data = String::new();
    file.read_to_string(&mut encoded_data)?;

    let decrypted_data = decrypt_data(key, &encoded_data)?;
    let mut file = File::create(file_path)?;
    file.write_all(&decrypted_data)?;
    Ok(())
}
