use crate::block::Block;

#[test]
fn constructor() {
    let b = Block::new("fakehash", "Hello World");

    assert_eq!("fakehash", b.parent_hash);
    assert_eq!("foobar", b.hash);
    assert_eq!(1,0)
}