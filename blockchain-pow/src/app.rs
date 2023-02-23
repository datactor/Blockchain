use super::*;

pub fn run() {
    // let mut input = String::new();

    println!("Enter Sender's addr: ");
    let sender = input().inner;

    println!("Enter Recipient's addr: ");
    let recipient = input().inner;

    println!("Enter transfer amount: ");
    let amount = input().to_u64().expect("please input correct number");

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

    let mut blockchain = Blockchain::new();

    let mut utxo_set = UtxoSet::new();

    genesis_block.check_merkle_and_mining().expect("Failed to execute mining");

    println!("Mined genesis Satoshi {:?}", &genesis_block);

    blockchain.update_with_block(genesis_block, &mut utxo_set, &"genesis".to_string()).expect("Failed to add genesis block");


    // new_block(네트워크에서 tx를 받아 block을 생성할 때)
    // 1. block을 생성하고 inputs, outputs들이 있는 tx들로 각각의 txid들을 생성해 하나의 블록에 하나의 merkle_root를 생성함.
    let mut new_block = blockchain.spawn_block(difficulty, sender.to_owned(), recipient.to_owned(), amount, &utxo_set);

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

    for output in &blockchain.chain[1].transactions[1].outputs {
        println!("{}, {}", output.to_addr, output.value)
    }
}