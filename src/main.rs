/*
 * 1c0111001f010100061a024b53535009181c
 */
extern crate rustc_serialize as serialize;
use serialize::hex::FromHex;
fn main(){
  let test = xor("1c","2c");
  for x in test{
    println!("{}", x);
  }
}
fn xor(input: &str, byte: &str)-> Vec<u8>{
    let input2hex = input.from_hex().unwrap(); 
    let byte2hex = byte.from_hex().unwrap();
    let mut xor = Vec::new();
    for i in 0..input.len(){
        xor.push(input2hex[i] ^ byte2hex[i]);
    }
    return xor;
}

