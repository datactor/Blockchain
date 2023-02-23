use super::*;
use std::collections::HashMap;

/// 솔라나의 블록체인은 다른 블록체인들과 다르게 1차원 체인이다. 이렇게 설계된 이유에는 주로 성능측면에 있다.
/// 1차원 구조는 다른 체인 구조에 비해 더 빠른 consensus 합의 및 tx throughput을 허용한다.
/// 솔라나의 블록체인에서 validator는 한 번에 단일 블록(single timestamped event)의 내용에
/// 동의(validation and confirmation)하므로 validator가 동의하는 데 필요한 시간을 최소화 할 수 있다.
///
/// 여기서 PoH라는 프로세스를 사용한다. PoH에서 validator는 네트워크의 cryptographic clock 역할을 하는
/// 단일 historical value에 대한 consensus에 도달한다. 이렇게 하면 모든 validator가 블록이 체인에
/// 추가되는 시간에 동의할 수 있다.
/// 이더리움이나 비트코인도 한 번에 단일 블록의 내용에 validation and confirmation 한다는 것은 동일하지만,
/// 그 합의 알고리즘의 차이가 속도 차이를 만든다. 특히나 validation의 부분에서 PoW와 PoS에 비해 PoH는 extremely 빠르다.
///
/// 또한 1차원 구조는 데이터 모델을 단순화하여 구현,
/// 테스트 및 추론을 더 쉽게 만든다. 또한 더 복잡한 데이터 모델의 여러 체인이 아니라
/// 동기화할 블록의 단일 체인이 있기 때문에 노드 간에 1차원 블록체인을 동기화하는 것이 더 쉬울 수 있다.
#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub height: u64,
    pub rewards: HashMap<Pubkey, u64>,
}

impl Blockchain {
    pub fn new(genesis: Block) -> Self {
        Blockchain {
            blocks: vec![genesis],
            height: 0,
            rewards: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
        self.height += 1;
        self.update_rewards();
    }

    fn update_rewards(&mut self) {
        self.rewards.clear();
        for block in &self.blocks {
            let rewards = block.rewards.clone();
            for (pubkey, reward) in rewards {
                let entry = self.rewards.entry(pubkey).or_insert(0);
                *entry += reward;
            }
        }
    }

    pub fn get_balance(&self, pubkey: Pubkey) -> u64 {
        *self.rewards.get(&pubkey).unwrap_or(&0)
    }
}