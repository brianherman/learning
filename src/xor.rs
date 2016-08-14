/*
 * 1c0111001f010100061a024b53535009181c
 */
extern crate rustc_serialize as serialize;
use serialize::hex::FromHex;
fn main(){
    let input_raw = "1c0111001f010100061a024b53535009181c";
    let input = input_raw.from_hex().unwrap(); 
    let against_raw = "686974207468652062756c6c277320657965";
    let against = against_raw.from_hex().unwrap();
    let mut xor = Vec::new();
    for i in 0..input.len(){
        xor.push(input[i] ^ against[i]);
    }
    for x in xor{
        println!("{:x}", x);
    }
}
