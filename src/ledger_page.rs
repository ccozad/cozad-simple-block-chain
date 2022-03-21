extern crate serde;

use std::fmt;
use crate::block::Block;
use serde::{Serialize, Deserialize};

/// A Ledger Page is a collection of Blocks
#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerPage {
    /// Storage for blocks
    entries: Vec<Block>,
    first_hash: String,
    last_hash: Option<String>,
    finalized: bool
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
            last_hash: None,
            finalized: false
        }
    }

    /// Adds a transaction to the ledger if it hasn't been finalized
    pub fn add_transaction(&mut self, transactions: &str) {
        if !self.finalized {
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
}

impl fmt::Display for LedgerPage {
    /// Represents the block as a JSON object string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

#[test]
fn test_constructor() {
    let lp = LedgerPage::new("MA==");

    assert_eq!(0, lp.entries.len());
}

#[test]
fn test_add_initial_transaction() {
    let mut lp = LedgerPage::new("MA==");
    lp.add_transaction("Hello World");

    assert_eq!(1, lp.entries.len());
    assert_eq!("RGUWhlfUKobrBmf5xjKPHUCBVe2wuP+FbDrLfQXEz2g=", lp.last_hash.unwrap());
}

#[test]
fn test_add_many_transactions() {
    let mut lp = LedgerPage::new("MA==");
    lp.add_transaction("Hello World");
    lp.add_transaction("Hello World Again");

    assert_eq!(2, lp.entries.len());
    assert_eq!("Ga5LMtyBWQuto1w4WWAttuHQMrHn2N5YqJ1T7f+INJ4=", lp.last_hash.unwrap());
}

#[test]
fn test_verify_true() {
    let mut lp = LedgerPage::new("MA==");
    lp.add_transaction("Hello World");
    lp.add_transaction("Hello World Again");

    println!("{}", lp);

    assert_eq!(2, lp.entries.len());
    assert_eq!(true, lp.verify());
}