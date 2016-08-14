/*
 * 1c0111001f010100061a024b53535009181c
 */
extern crate rustc_serialize as serialize;
use serialize::hex::FromHex;
use serialize::base64::{self, ToBase64};
fn main(){
  let test = xor("111111","ffffff");
  for x in test{
    println!("{}", x);
  }
}
fn xor(input: &str, byte: &str)-> Vec<u8>{
    if input.len() != byte.len(){
        return vec![0];
    }
    let input2hex = input.from_hex().unwrap(); 
    let byte2hex = byte.from_hex().unwrap();
    let mut xor = Vec::new();
    for it in input2hex.iter().zip(byte2hex.iter()){
        let (ai, bi) = it;
        xor.push(ai ^ bi);
    }
    return xor;
}


fn base64(input: &str) -> String{
    return input.from_hex().unwrap().to_base64(base64::STANDARD);
}
