extern crate crypto;
extern crate rand;
use std::str;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use std::io;
use std::fs::File;
use std::io::prelude::*;
fn open_file(path: &str)-> Result<String, io::Error>{
    let mut f = try!(File::open(path));
    let mut encrypted_data_string = String::new();
    try!(f.read_to_string(&mut encrypted_data_string));
    Ok(encrypted_data_string)
}
fn decrypt()-> Result<Vec<u8>, symmetriccipher::SymmetricCipherError>{
    let key = "YELLOW SUBMARINE".as_bytes();
    let encrypted_data = open_file("/home/brianherman/pysimplerust/learning/7.txt"); 
    let mut decryptor = aes::cbc_decryptor(aes::KeySize::KeySize128,
                                           key,
                                           &[0u8],
                                           blockmodes::NoPadding);
    let mut final_result = Vec::<u8>::new();
    let ed = encrypted_data.ok().unwrap();
    let mut read_buffer = buffer::RefReadBuffer::new(ed.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop{
         let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
         final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result{
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    println!("{:?}", final_result);
    Ok(final_result)
}

fn main(){
    let decrypted = decrypt().ok().unwrap();
    println!("{:?}", decrypted);
    let end = str::from_utf8(&decrypted).unwrap();
    println!("{:?}", end);
}
