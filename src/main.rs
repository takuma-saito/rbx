#![allow(dead_code)]

use std::collections::HashMap;
use sha2::{Sha256, Digest};

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
        s.reverse();
        String::from_utf8(s).unwrap()
    }
    
    fn decode(&self, text: &str) -> Vec<u8> {
        let mut s = text.as_bytes().to_vec();
        let mut bin = Vec::new();
        let mut carry = 0u16;
        while let Some(c) = s.pop() {
            carry = carry * 58 + (self.rev_table[&c] as u16);
            while carry >= 256 {
                bin.push((carry%256) as u8);
                carry /= 256;
            }
        }
        if carry > 0 { bin.push((carry%256) as u8); }
        bin.reverse();
        bin
    }
}

struct Base58check {
    base58: Base58
}

// https://github.com/satoshilabs/slips/blob/master/slip-0132.md
impl Base58check {
    fn new() -> Self {
        Base58check {
            base58: Base58::new()
        }
    }

    fn double_hash(&self, bin: &[u8]) -> Vec<u8> {
        Sha256::digest(Sha256::digest(bin)).to_vec()
    }

    fn encode(&self, mut version: Vec<u8>, mut bin: Vec<u8>) -> String {
        version.append(&mut bin); // TODO: もう少しいいやり方を考える
        let mut checksum = self.double_hash(&version)[0..4].to_vec();
        version.append(&mut checksum); // TODO: Vec に変更せず slice で実装する
        self.base58.encode(version)
    }

    fn decode(&self, text: &str) -> Vec<u8> {
        let data = self.base58.decode(text);
        let res = data[4 .. (data.len()-4)].to_vec(); // TODO: checksum のコードを入れる, version drop 対応
        println!("{:?}", data.iter().map(|x| format!("{:02X}", x)).collect::<String>());
        res
    }
}

fn main() {
    let str = "xpub6BosfCnifzxcFwrSzQiqu2DBVTshkCXacvNsWGYJVVhhawA7d4R5WSWGFNbi8Aw6ZRc1brxMyWMzG3DSSSSoekkudhUd9yLb6qx39T9nMdj";
    let base58check = Base58check::new();
    let mut version = vec![0x04, 0x88, 0xb2, 0x1e];
    println!("{:?}", base58check.encode(version, base58check.decode(str)));
}
