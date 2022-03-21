# cozad-simple-block-chain
A Rust implementation of simple block chain operations

![MIT License](https://img.shields.io/github/license/ccozad/cozad-simple-block-chain) 
![Build Status](https://img.shields.io/github/workflow/status/ccozad/cozad-simple-block-chain/Build) 
![Code Size](https://img.shields.io/github/languages/code-size/ccozad/cozad-simple-block-chain) 
![Top Language](https://img.shields.io/github/languages/top/ccozad/cozad-simple-block-chain)

# Quick Start

This work is still in progress, check back later for more content!

## Run the tests

```
cargo test
```

# Concepts
 - Block
   - A Block is one element in a linked list data structure
   - A Block has data about the parent's hash, transactions and a hash over the (parent hash + transaction)
   - The Genesis Block is the first block in the linked list, it has a parent hash that is a sentinel value
   - A Block is verified if the hash matches the calculated has of the (parent hash + transaction)
 - Ledger Page
   - A collection of Blocks
   - The first node can be a Genesis Block or a normal block
   - The last block is a normal Block
   - A Ledger Page is verified if all Blocks in the collection verify and each item in the collection satisfies the requirement `Block[n].hash` equals `Block[n+1].parent_hash`
 - Ledger
   - A list of Ledger Pages


# Support
 - How do I request a change?
   - Please submit an issue or a pull request
 - How fast will my request be added?
   - Probably not very fast for requests outside of a support package because this repo is maintained by a working professional
   - If you require fast, predictable responses, please purchase a support package
 - Can support package be purchased?
   - Yes, various support packages can be purchased and customized for your needs. Support areas available include:
   - On demand support videos
   - 1:1 and team coaching
   - New features and other modifications