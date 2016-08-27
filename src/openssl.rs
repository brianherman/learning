extern crate openssl;
extern crate serialize;
use openssl::crypto::symm::{AES_128_ECB, Crypter, Encrypt, Decrypt};
use serialize::hex::ToHex;
use std::io::prelude::*;
use std::fs::File;

fn main(){
    let mut f = try!(File::open("cypher.txt"));
    let mut cyphertext = String::new();
    
    let decrypter = Crypter::new(AES_128_ECB);
    decrypter.init(Decrypt, key.as_slice(), Vec::from_elem(16,0));

    let decrypted = decrypter.update(ciphertext.as_slice());

    println!("{}", ciphertext.as_slice().to_hex());
    println!("{}", decrypted.as_slice().to_hex());

    println!("{}", ciphertext.len());
    println!("{}", decrypted.len());
} 
