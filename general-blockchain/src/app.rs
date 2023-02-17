use std::io;
use super::*;

pub fn run() {
    let mut input = String::new();
    println!("Enter Sender's addr: ");
    io::stdin().read_line(&mut input).unwrap();
    let sender = input.trim().to_owned();
    input.clear();

    println!("Enter Recipient's addr: ");
    io::stdin().read_line(&mut input).unwrap();
    let recipient = input.trim().to_owned();
    input.clear();

    println!("Enter transfer amount: ");
    io::stdin().read_line(&mut input).unwrap();
    let amount = input.trim().parse::<u64>().unwrap();

    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![],
        difficulty
    );

    let satoshi_tx = Transaction {
        inputs: vec![],
        outputs: vec![
            transaction::Output {
                to_addr: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_owned(),
                value: 50,
            },
        ],
    };

    genesis_block.add_transaction(satoshi_tx);

    let mut utxo_set = UtxoSet::new();

    genesis_block.check_merkle_and_mining().expect("Failed to execute mining");

    println!("Mined genesis Satoshi {:?}", &genesis_block);

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block, &mut utxo_set, &"genesis".to_string()).expect("Failed to add genesis block");


    // new_block(네트워크에서 tx를 받아 block을 생성할 때)
    // 1. block을 생성하고 inputs, outputs들이 있는 tx들로 각각의 txid들을 생성해 하나의 블록에 하나의 merkle_root를 생성함.
    let mut new_block = spawn_block(difficulty, blockchain.chain.last().unwrap(), sender.to_owned(), recipient.to_owned(), amount, &utxo_set);

    // 2. 채굴자가 다른 node로부터 갱신된 block을 받아 mining함(mining 수행 전에 txid들로 merkle_root를
    // 자체적으로 계산해 보고 블록헤더에서 받은 merkle_root와 동일한지 체크하고 동일하면 mining, 다르다면 버린다.)

    // Integrity check with merkle root and mining
    new_block.check_merkle_and_mining().expect("Failed to execute mining");
    println!("Mined {:?}", &new_block);

    // mining이 성공적으로 완료된다면 네트워크로 보낸다. 네트워크는 완료된 블록을 blockchain에 추가하기 전에
    // broadcast해 다른 node들(miner)에게도 merkle root를 추가적으로 검증하게 한다. 이 과정은 채굴이 아니다.
    // 추가적 검증이 완료되면 blockchain에 추가시킨다. 그렇지만 이것으로 최종 blockchain이 결정되는 것은 아니다.
    // btc에는 confirmation thresholds(확인 임계값) chain rule이 있는데, 그 위에 6개의 추가적인 block이 쌓일
    // 때까지 최종 블록으로 간주하지 않는다. 총 7개의 blockchain에 추가된 block 중, 누적 PoW가 가장 많은,
    // 가장 긴 chain(longest 또는 heaviest chain이라고 함) 하나가 Winner가 되어 네트워크의 유효한 block으로 간주된다.
    // Winner로 선택되지 않은 나머지 6개의 block은 여전히 네트워크에 존재하고, 블록체인에도 같은 layer에 존재하지만
    // 현재로서는 invalid상태이다. 즉 TX가 유효한 chain의 part로 인정되지 않는다. 그러나 추후에 새로운 layer에서
    // 이 버려진 invalid block을 history의 일부로 포함하는 더 긴 chain이 구성되어
    // 새로운 block으로서 또 다른 6개의 경쟁 block을 이길 경우, 이 invalid block은 다시 valid로 간주되고
    // 새로운 chain에 포함된다.
    // btc의 경우 여기서 한가지 overcompensate 될 여지가 남겨진다.
    // 예를 들어, 만약 같은 tx들로 구성된 새로운 block들이 경쟁한다면? 하나의 강한 block만 유효하게 되고,
    // 유효한 block과 같은 tx를 가진 invaild block이 blockchain의 같은 layer에 남게된다.
    // 추후에 다른 layer에서 Winner block이, 이미 nonce가 밝혀진, 이전의 winner block과
    // tx가 같은 invalid block을 history로 갖는다면 이 중복 block도 보상을 받고 layer에 추가 된다.(중복 Tx, nonce를 가진 block들이 존재)
    // 그렇지만 이것을 막으면 채굴자들의 보상을 줄이게 된다.

    blockchain.update_with_block(new_block, &mut utxo_set, &sender).expect("Failed to add block");

    println!("{}, {}", blockchain.chain[1].transactions[1].outputs[0].to_addr, blockchain.chain[1].transactions[1].outputs[0].value);
    println!("{}, {}", blockchain.chain[1].transactions[1].outputs[1].to_addr, blockchain.chain[1].transactions[1].outputs[1].value);
}

fn spawn_block(difficulty: u128, prev_block: &Block, sender: String, recipient: String, mut amount: u64, utxo_set: &UtxoSet) -> Block {
    let fee = 0;

    let block_reward = 7; // 블록보상 6.25 + 추가적인 transaction fee
    // println!("{:?}", val);
    let mut block = Block::new(
        prev_block.index + 1,
        now(),
        prev_block.hash.clone(),
        vec![],
        difficulty
    );

    // coinbase transaction
    // 블록을 생성한 광부. 마이닝 해서 블록체인에 붙이려고 시도한다.
    // 이 coinbase tx의 sender도 광부, recipient도 광부. coinbase address라고 불린다.
    let coinbase_tx = Transaction {
        inputs: vec![],
        outputs: vec![
            transaction::Output {
                to_addr: "coinbase_miner".to_owned(),
                value: block_reward,
            },
        ],
    };

    block.add_transaction(coinbase_tx);

    ///////////

    let inputs;
    if let Some(input) = utxo_set.get_optimal_inputs(amount) {
        inputs = input.iter().map(
            |(txid, txid_idx, input_amount, script_pubkey)| (format!("{}:{}", txid, txid_idx), input_amount.clone(), script_pubkey.clone()))
            .collect::<Vec<_>>();
    } else {
        panic!("You cannot transfer more than the remaining UTXO.");
    }

    let mut sub_amount = amount;
    for (txid_idx, input_amount, script_pubkey) in inputs {
        let input_to_addr = script_pubkey.split(":").nth(1).unwrap();
        if input_amount < amount {
            sub_amount = input_amount + fee;
            amount -= sub_amount;
        }

        let mut outputs = vec![
            transaction::Output {
                to_addr: recipient.clone(),
                value: sub_amount,
            }
        ];

        if input_amount > sub_amount {
            // change.
            // btc network에서 요구하는 대로 Input의 총 가치가 출력의 총 가치와 동일하도록 하기 위해
            // 본인에게 반환되는 Output 추가.
            outputs.push(
                transaction::Output {
                    to_addr: sender.clone(),
                    value: input_amount - sub_amount,
                },
            )
        };

        let mut inputs = Vec::new();
        inputs.push((
            transaction::Output {
                to_addr: input_to_addr.to_owned(),
                value: input_amount,
            }, txid_idx
        ));

        let transaction = Transaction {
            inputs,
            outputs,
        };

        block.add_transaction(transaction);
    }

    block.clone()
}