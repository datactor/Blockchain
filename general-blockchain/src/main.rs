use blockchainlib::*;

fn main() {
    let mut block = Block::new(1, now(), vec![0; 32], 0, "Genesis block".to_owned());

    println!("{:?}", &block);

    let h = block.hash();

    println!("{:?}", &h);

    block.hash = h;

    println!("{:?}", &block);
}