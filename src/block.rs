extern crate sha2;
extern crate base64;
extern crate serde;

use std::fmt;
use std::str::FromStr;
use sha2::{Sha256, Digest};
use base64::{encode, decode};
use serde::{Serialize, Deserialize};

/// A Block is segment of information in a block chain
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct Block {
    /// The hash of the parent
    parent_hash: String,
    /// Information captured in block
    transactions: String,
    /// Hash of parent hash and transactions
    hash: String,
}

impl Block {
    /// Constructs a new `Block`
    /// 
    /// # Arguments
    ///
    /// * `parent_hash` - The hash of the parent
    /// * `transactions` - Information captured in block
    #[allow(dead_code)]
    pub fn new(parent_hash: &str, transactions: &str) -> Self {
        let parent_hash_bytes = decode(parent_hash);
        let transactions_bytes = transactions.as_bytes().to_vec();
        
        let mut hasher = Sha256::new();
        hasher.update([parent_hash_bytes.unwrap(), transactions_bytes].concat());
        let hash_bytes = hasher.finalize();

        Block {
            parent_hash: String::from(parent_hash),
            transactions: String::from(transactions),
            hash: String::from(encode(hash_bytes))
        }
    }

    pub fn parent_hash(&self) -> &str {
        self.parent_hash.as_str()
    }

    pub fn transactions(&self) -> &str {
        self.transactions.as_str()
    }

    pub fn hash(&self) -> &str {
        self.hash.as_str()
    }

    /// Checks to see is block hash is valid
    #[allow(dead_code)]
    pub fn verify(&self) -> bool {
        let parent_hash_bytes = decode(self.parent_hash.as_str());
        let transactions_bytes = self.transactions.as_bytes().to_vec();
        
        let mut hasher = Sha256::new();
        hasher.update([parent_hash_bytes.unwrap(), transactions_bytes].concat());
        let hash_bytes = hasher.finalize();
        let hash = encode(hash_bytes);

        self.hash.eq(&hash)
    }
}

impl fmt::Display for Block {
    /// Represents the block as a JSON object string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

impl FromStr for Block {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let b = Block::new("MA==", "Hello World");

        assert_eq!("MA==", b.parent_hash);
        assert_eq!("Hello World", b.transactions);
        assert_eq!("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=", b.hash);
    }

    #[test]
    fn parent_hash() {
        let b = Block::new("MA==", "Hello World");

        assert_eq!("MA==", b.parent_hash());
    }

    #[test]
    fn transactions() {
        let b = Block::new("MA==", "Hello World");

        assert_eq!("Hello World", b.transactions());
    }

    #[test]
    fn hash() {
        let b = Block::new("MA==", "Hello World");

        assert_eq!("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=", b.hash());
    }

    #[test]
    fn verify_true() {
        let b = Block::new("MA==", "Hello World");

        assert_eq!(true, b.verify());

        let b2 = Block {
            parent_hash: String::from("MA=="),
            transactions: String::from("Hello World"),
            hash: String::from("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=")
        };

        assert_eq!(true, b2.verify());
    }

    #[test]
    fn verify_false() {
        let b = Block {
            parent_hash: String::from("MA=="),
            transactions: String::from("foo bar"),
            hash: String::from("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=")
        };

        assert_eq!(false, b.verify());
    }

    #[test]
    fn to_string() {
        let b = Block::new("MA==", "Hello World");
        let serialized_b = b.to_string();
        let expected_result = r#"{"parent_hash":"MA==","transactions":"Hello World","hash":"RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}"#;

        assert_eq!(expected_result, serialized_b);
    }

    #[test]
    fn from_str() {
        let serialized_b = r#"{"parent_hash":"MA==","transactions":"Hello World","hash":"RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}"#;
        let b = Block::from_str(&serialized_b).unwrap();

        assert_eq!("MA==", b.parent_hash);
        assert_eq!("Hello World", b.transactions);
        assert_eq!("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=", b.hash);
    }
}
