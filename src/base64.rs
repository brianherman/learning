extern crate rustc_serialize as serialize;
use serialize::base64::{self, ToBase64};
use serialize::hex::FromHex;

fn base64(input: str) -> Vec<u8>{
    return input.from_hex().unwrap().to_base64(base64::STANDARD);
}
fn main() {
    let hex = vec![0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x62, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20, 0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20, 0x6d, 0x75, 0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d];

    let CODES = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    let mut encoded = String::new();
    let mut index = 0;
    while index < hex.len(){
        let mut b:i32= hex[index] & 0xFC >> 2;
        encoded.push(CODES.chars().nth(b as usize).unwrap());
        b = hex[index] & 0x03 << 4;
        if index + 1 < hex.len(){
            let position = index + 1;
            b = b | (hex[index + 1] & 0xF0) >> 4;
            encoded.push(CODES.chars().nth(b as usize).unwrap());
            if index + 2 < hex.len(){
                b = b | (hex[index + 2] & 0xC0) >> 6;
                encoded.push(CODES.chars().nth(b as usize).unwrap());
                b = b | (hex[index + 2] & 0x3F);
                encoded.push(CODES.chars().nth(b as usize).unwrap()); 
            }else{
                encoded.push(CODES.chars().nth(b as usize).unwrap()); 
                encoded.push('=');
            }
        }else{
            encoded.push(CODES.chars().nth(b as usize).unwrap()); 
            encoded.push('=');
            encoded.push('=');
        }
        index += 3;
    }
    println!("{}", encoded);
    assert_eq!(encoded, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}
