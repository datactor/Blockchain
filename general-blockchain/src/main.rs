use blockchainlib::*;

fn main() {
    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Genesis block".to_owned(),
        0x000000ffffffffffffffffffffffffff);

    block.hash = block.hash();

    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);
}