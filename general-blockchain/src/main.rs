use blockchainlib::*;
use std::{
    io::{self, prelude::*},
};
use blockchainlib::transaction::Output;

fn main() {
    let mut recipient = String::new();
    println!("Enter recipient: ");
    io::stdin().read_line(&mut recipient).unwrap();
    let recipient = recipient.trim().to_lowercase();

    let mut amount = String::new();
    println!("Enter transfer amount: ");
    io::stdin().read_line(&mut amount).unwrap();
    let amount = amount.trim().parse::<u64>().unwrap();

    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        // vec![],
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    // 이중 성공한 output만 UTXO로서 추후에 input으로 사용될 수 있다.
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 50,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 7,
                    },
                ]
            }
        ],
        difficulty
    );

    // let transaction = Transaction {
    //     inputs: vec![],
    //     outputs: vec![
    //         transaction::Output {
    //             to_addr: "Alice".to_owned(),
    //             value: 50,
    //         },
    //     ],
    // };
    //
    // genesis_block.add_transaction(transaction);
    //
    // let transaction2 = Transaction {
    //     inputs: vec![],
    //     outputs: vec![
    //         transaction::Output {
    //             to_addr: "Bob".to_owned(),
    //             value: 7,
    //         },
    //     ],
    // };
    //
    // genesis_block.add_transaction(transaction2);

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");






    // new_block
    let mut new_block = spawn_block(difficulty, blockchain.blocks.last().unwrap(), recipient.to_owned(), amount, vec![]);

    new_block.mine();

    println!("Mined block {:?}", &new_block);

    blockchain.update_with_block(new_block).expect("Failed to add block");


    println!("{}, {}", blockchain.blocks[1].transactions[1].outputs[0].to_addr, blockchain.blocks[1].transactions[1].outputs[0].value);
    println!("{}, {}", blockchain.blocks[1].transactions[1].outputs[1].to_addr, blockchain.blocks[1].transactions[1].outputs[1].value);
}


// 이전 블록들에서 UTXO를 불러와서 최적의 transaction value를 맞추는 Input을 자동으로 넣어야 함.
// merkle tree에 대해 알아보자
fn spawn_block(difficulty: u128, prev_block: &Block, recipient: String, amount: u64, _opt_input: Vec<Output>) -> Block {
    let input = prev_block.transactions[0].outputs[0].clone(); // 임시 단일 input
    let val = input.value;
    let block_reward = 6;
    println!("{:?}", val);
    let mut block = Block::new(
        prev_block.index + 1,
        now(),
        prev_block.hash.clone(),
        vec![],
        // vec![
        //     Transaction {
        //         inputs: vec![],
        //         outputs: vec![
        //             transaction::Output {
        //                 to_addr: "coinbase_to_miner".to_owned(),
        //                 value: block_reward,
        //             },
        //         ],
        //     },
        //     Transaction {
        //         inputs: vec![ // merkle root 구현해서 넣을 것
        //             input.clone()
        //         ],
        //         outputs: vec![
        //             transaction::Output {
        //                 to_addr: recipient.clone(),
        //                 value: amount,
        //             },
        //             // btc network에서 요구하는 대로 Input의 총 가치가 출력의 총 가치와 동일하도록 하기 위해
        //             // 본인에게 반환되는 Output 추가.
        //             transaction::Output {
        //                 to_addr: "Alice".to_owned(),
        //                 value: val - amount,
        //             },
        //         ]
        //     }
        // ],

        difficulty
    );

    let transaction = Transaction {
        inputs: vec![],
        outputs: vec![
            transaction::Output {
                to_addr: "coinbase_to_miner".to_owned(),
                value: block_reward,
            },
        ],
    };

    block.add_transaction(transaction);

    let transaction2 = Transaction {
        inputs: vec![ // merkle root에서 색인할 것
            input
        ],
        outputs: vec![
            transaction::Output {
                to_addr: recipient.clone(),
                value: amount,
            },
            // btc network에서 요구하는 대로 Input의 총 가치가 출력의 총 가치와 동일하도록 하기 위해
            // 본인에게 반환되는 Output 추가.
            transaction::Output {
                to_addr: "Alice".to_owned(),
                value: val - amount,
            },
        ]
    };
    block.add_transaction(transaction2);

    block.clone()
}