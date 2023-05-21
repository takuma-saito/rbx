use std::collections::HashMap;

struct Base58 {
    table: Vec<u8>,
    revTable: HashMap<u8, u8>
}

impl Base58 {
    fn new() -> Self {
        let table = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".as_bytes().to_vec();
        let mut revTable = HashMap::new();
        for (i, c) in table.iter().enumerate() {
            revTable[c] = i as u8;
        }
        Base58 {
            table: table,
            revTable: revTable
        }
    }

    fn encode(&self, bin: Vec<u8>) -> String {
        let mut s = Vec::new();
        let mut carry = 0u32;
        while let Some(u) = bin.pop() {
            let v = u as u32 + carry;
            s.push(self.table[(v%58) as usize]);
            carry = v/58;
        }
        while carry != 0 {
            s.push(self.table[(carry%58) as usize]);
            carry /= 58;
        }
        String::from_utf8(s).unwrap()
    }
    
    fn decode(&self, text: &str) -> Vec<u8> {
        let mut s = text.as_bytes().to_vec();
        let mut bin = Vec::new();
        let mut carry = 0u32;
        while let Some(c) = s.pop() {
            let u = self.revTable[&c] as u32 + (carry*58);
            if u > 255 { bin.push((u%256) as u8); }
            carry = u/58;
        }
        while carry != 0 {
            bin.push((carry%256) as u8);
            carry /= 256;
        }
        bin
    }
}

fn main() {
    
}
