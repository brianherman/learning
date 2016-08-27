/*
 * 1c0111001f010100061a024b53535009181c
 */
extern crate rustc_serialize as serialize;
use serialize::hex::FromHex;
use serialize::base64::{self, ToBase64};
fn main(){
    let cipher = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let dec = cipher.from_hex().unwrap();
    let mut out = Vec::new();
    for d in dec{
        for x in 0..255{
            out.push(d ^ x as u8);
        }
    }
    for x in out{
        print!("{}",x as char);
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
