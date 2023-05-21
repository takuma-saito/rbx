#![allow(dead_code)]

use std::collections::HashMap;

struct Base58 {
    table: Vec<u8>,
    rev_table: HashMap<u8, u8>
}

impl Base58 {
    fn new() -> Self {
        let table = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".as_bytes().to_vec();
        let mut rev_table = HashMap::new();
        for (i, c) in table.iter().enumerate() {
            rev_table.insert(*c, i as u8);
        }
        Base58 {
            table: table,
            rev_table: rev_table
        }
    }

    fn encode(&self, mut bin: Vec<u8>) -> String {
        let mut s = Vec::new();
        let mut carry = 0u16;
        while let Some(u) = bin.pop() {
            carry = carry * 256 + (u as u16);
            while carry != 0 {
                s.push(self.table[(carry%58) as usize]);
                carry /= 58;
            }
        }
        String::from_utf8(s).unwrap()
    }
    
    fn decode(&self, text: &str) -> Vec<u8> {
        let mut s = text.as_bytes().to_vec();
        let mut bin = Vec::new();
        let mut carry = 0u16;
        while let Some(c) = s.pop() {
            carry = carry*58 + (self.rev_table[&c] as u16);
            while carry != 0 {
                println!("{:?}", carry%58);
                bin.push((carry%58) as u8);
                carry /= 58;
            }
        }
        println!("{:?}", bin);
        bin
    }
}

fn main() {
    let str = "9xpXFhFpqdQK3TmytPBqXtGSwS3DLjojFhTGht8gwAAii8py5X6pxeBnQ6ehJiyJ6nDjWGJfZ95WxByFXVkDxHXrqu53WCRGypk2ttuqncb";
    let base58 = Base58::new();
    println!("{:?}", base58.encode(base58.decode(str)));
}
