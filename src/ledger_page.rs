extern crate serde;

use std::fmt;
use std::str::FromStr;
use crate::block::Block;
use serde::{Serialize, Deserialize};

/// A Ledger Page is a collection of Blocks
#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerPage {
    /// Storage for blocks
    entries: Vec<Block>,
    first_hash: String,
    last_hash: Option<String>
}

impl LedgerPage {
    /// Constructs a new `LedgerPage`
    /// 
    /// # Arguments
    ///
    /// * `initial_parent_hash` - The first parent hash on the page
    #[allow(dead_code)]
    pub fn new(initial_parent_hash: &str) -> Self {
        let entries = Vec::new();

        LedgerPage {
            entries,
            first_hash: String::from(initial_parent_hash),
            last_hash: None
        }
    }

    /// Adds a transaction to the ledger
    pub fn add_transaction(&mut self, transactions: &str) {
        if self.entries.len() > 0 {
            let parent_hash = self.entries.last().unwrap().hash();
            let block = Block::new(parent_hash, transactions);
            self.last_hash = Some(String::from(block.hash()));
            self.entries.push(block);
        } else {
            let block = Block::new(self.first_hash.as_str(), transactions);
            self.last_hash = Some(String::from(block.hash()));
            self.entries.push(block);
        }
    }

    /// Checks to see if all Blocks in the collection verify and linked list integrity stands
    /// 
    /// Linked list integrity means each item in the collection satisfies the requirement 
    /// `Block[n].hash` equals `Block[n+1].parent_hash`
    #[allow(dead_code)]
    pub fn verify(&self) -> bool {
        let mut parent_hash = self.first_hash.as_str();
        let mut is_verified = true;

        for block in &self.entries {
            is_verified &= block.parent_hash() == parent_hash;
            is_verified &= block.verify();
            parent_hash = block.hash();
        }

        is_verified
    }

    pub fn first_hash(&self) -> &str {
        self.first_hash.as_str()
    }

    pub fn last_hash(&self) -> Option<&str> {
        if self.last_hash.is_some() {
            let hash = self.last_hash.as_ref().unwrap().as_str();
            Some(hash)
        } else {
            None
        }
    }

    pub fn entries(&self) -> &[Block] {
        &self.entries[..]
    }
}

impl fmt::Display for LedgerPage {
    /// Represents the block as a JSON object string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

impl FromStr for LedgerPage {
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
        let lp = LedgerPage::new("MA==");

        assert_eq!(0, lp.entries.len());
    }

    #[test]
    fn first_hash() {
        let mut lp = LedgerPage::new("MA==");
        lp.add_transaction("Hello World");

        assert_eq!("MA==", lp.first_hash());
    }

    #[test]
    fn last_hash() {
        let mut lp = LedgerPage::new("MA==");
        lp.add_transaction("Hello World");

        assert_eq!("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=", lp.last_hash().unwrap());
    }

    #[test]
    fn entries() {
        let mut lp = LedgerPage::new("MA==");
        lp.add_transaction("Hello World");

        let entries = lp.entries();

        assert_eq!(1, entries.len());
        assert_eq!("MA==", entries[0].parent_hash());
        assert_eq!("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=", entries[0].hash());
        assert_eq!("Hello World", entries[0].transactions());
        assert_eq!(true, entries[0].verify())
    }

    #[test]
    fn add_initial_transaction() {
        let mut lp = LedgerPage::new("MA==");
        lp.add_transaction("Hello World");

        assert_eq!(1, lp.entries.len());
        assert_eq!("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=", lp.last_hash.unwrap());
        assert_eq!("MA==", lp.first_hash);
    }

    #[test]
    fn add_many_transactions() {
        let mut lp = LedgerPage::new("MA==");
        lp.add_transaction("Hello World");
        lp.add_transaction("Hello World Again");

        assert_eq!(2, lp.entries.len());
        assert_eq!("Ga5LMtyBWQuto1w4WWAttuHQMrHn2N5YqJ1T7f+INJ4=", lp.last_hash.unwrap());
        assert_eq!("MA==", lp.first_hash);
    }

    #[test]
    fn verify_true() {
        let mut lp = LedgerPage::new("MA==");
        lp.add_transaction("Hello World");
        lp.add_transaction("Hello World Again");

        println!("{}", lp);

        assert_eq!(2, lp.entries.len());
        assert_eq!(true, lp.verify());
    }

    #[test]
    fn to_string() {
        let mut lp = LedgerPage::new("MA==");
        lp.add_transaction("Hello World");

        let lp_serialized = lp.to_string();
        let expected_result = r#"{"entries":[{"parent_hash":"MA==","transactions":"Hello World","hash":"RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}],"first_hash":"MA==","last_hash":"RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}"#;

        assert_eq!(expected_result, lp_serialized);
    }

    #[test]
    fn from_str() {
        let lp_serialized = r#"{"entries":[{"parent_hash":"MA==","transactions":"Hello World","hash":"RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}],"first_hash":"MA==","last_hash":"RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}"#;
        let lp = LedgerPage::from_str(&lp_serialized).unwrap();

        assert_eq!(1, lp.entries.len());
        assert_eq!("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=", lp.last_hash.unwrap());
        assert_eq!("MA==", lp.first_hash);
    }

    #[test]
    fn verify_false() {
        let lp_serialized = r#"{"entries":[{"parent_hash":"MA==","transactions":"Hello World","hash":"AGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}],"first_hash":"MA==","last_hash":"RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}"#;
        let lp = LedgerPage::from_str(&lp_serialized).unwrap();

        assert_eq!(false, lp.verify());

        let lp_serialized2 = r#"{"entries":[{"parent_hash":"MA==","transactions":"Hello World","hash":"RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}],"first_hash":"GA==","last_hash":"RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g="}"#;
        let lp2 = LedgerPage::from_str(&lp_serialized2).unwrap();

        assert_eq!(false, lp2.verify());
    }
}