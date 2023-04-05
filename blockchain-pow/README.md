# Generic blockchain(with pow)
Blockchain = chronological, sequential list of blocks

## Why Rust?

- Interoperable with C/C++
- smart compiler
- Strict but safe and fast type system(monomorphism)
- Simple GC
- "Pointers" are always safe. even multi thread env


## 1. Blocks & hashing
block & block chain? linked list or linked array. 

Blocks contain this informs:
- Index: this block's location within the list of blocks
- Payload: any relevant information or events that have
- Timestamp: gives our blockchain a sense of time
- Nonce: special number used for mining (for PoW verification)
- Previous block hash: cryptographic fingerprint of previous block
- Hash: cryptographic fingerprint of all of the above data concatenated together

### Hashing? Generate digital fingerprint

In a nutshell, a hash algorithm consists of a set of irreversible computations
that can be performed on a datum to generate a (usually) unique byte sequence.
In a blockchain, each block contains a set of transactions and
a reference to the previous block's hash. The current block's hash is calculated by
applying the hash function to the combination of the block's transactions and
the previous block's hash. This creates a chain of hashes that is resistant to tampering,
as any change in the block data will result in a different hash.
`SHA2`, `SHA3(Keccak-256)`

1. Concatenate together all the bytes composing the block's fields
   (aside from the hash field) 
   - In the early days, hashcash algorithm(SHA-256^2) was used,
   but now SHA-256 is used. Hashcash is too inefficient due to excessive amount of calculation,
   limited scalability, lack of adaptability, for these reasons, it is less attractive than other algorithms.
2. Generate unique data fingerprint:the hash
3. One-way & Deterministic
   - When you interpret same series of bytes, you will always get same hash. 
     However, you cannot get the series of bytes from the hash.


4. Difficulty?
   
   SHA-256 generates a 32-byte hash. Difficulty (in our case) specifies the unsigned 128-bit
   integer value that the most significant 16bytes of the hash of a block must be less than
   before it is considered "valid" (if those bytes are interpreted as a single number instead
   of a series of bytes). Difficulty will be stored as a field of the Block struct.
   
   Difficulty could also be expressed as:
   - The first `n` bytes of the hash that must be zero.
   - The number of bits or bytes at the beginning of the hash that must be zero.
   
   These options are essentially different ways of expressing the same thing.
5. Little vs Big Endian
   
   Endianness: Order of bytes stored in memory.
   
   Example: 42u32
   
   Hex representation
   - Stored in big-endian order
   - Stored in little-endian order(most common)
   
   0x0000002a
   - 00 00 00 2a
   - 2a 00 00 00 // Reversing the order of bytes(not bit)
   
   If we treat it like a little endian representation of a number, the most
   significant 16 bytes of our hash will appear at the end of our hash's byte vector[16, 32].

6. Nonce
   A hash is a unique, reproducible fingerprint for some data. Therefore.
   to make a "valid" hash(per difficulty), we must somehow change the bytes
   we send to the function(the pre-image). Remember that even one small change to the input changes
   the resultant hash drastically. This effect is commonly called avalanching.
   \
   \
   Of course. we can't actually change the information stored in a block willy-nilly.
   Therefore, we introduce an additional piece of data called a `nonce`:
   an arbitrary(but not necessarily random) value added as a filed to each block,
   and hashed along with the data. Since it has been declared arbitrary, we can change it as we please.
   \
   \
   You can think of it like this: generating the correct hash for a block is like the puzzle,
   and the nonce is the key to that puzzle. The process of finding that key is called mining.

## 2. mining
Generating the correct hash for a block is like the puzzle,
and the nonce is the key to that puzzle. The process of finding that key is called mining.
### Mining Strategy
1. Generate new nonce
2. Hash bytes(this is the computationally heavy step)
3. Check hash against difficulty
   1. Insufficient? Go back to step 1
   2. Sufficient? Continue to step 4
4. Add block to chain
5. Submit to peers, etc.

요약하면, 난이도 목표는 목표와 같거나 더 큰 난이도의 hash를 생성하는 nonce를 찾는 데 평균 일정 시간이 걸리도록 설계되었으며,
이것이 채굴 프로세스를 안전하게 만들고 사람들이 블록체인을 쉽게 조작하는 것을 방지하는 것이다.
이 과정이 mining이며 목표값을 찾으면 mining은 완성된다.

- Vec[16..32]에 들어있는 difficulty 값(u128로 치환, 입력한 임의의 난이도)의 hash를 생성하는 nonce를 찾기 위해
  nonce값을 0~2^64까지 1씩 증가 시키며 block을 byte array로 치환 
- 치환한 byte array를 hashing(digest)해서 입력한 임의의 난이도(u128)값과 비교한다. 
  1. 입력값이 더 클 경우 탐색을 중단하고 블록을 chain에 push한다.
  2. 크지 않을 경우 continue해서 계속 탐색
  
앞에서도 강조한 것처럼 block header의 hash는 비가역성을 가졌기 때문에
역으로 찾을 수 없어(복호화 불가) O(n)의 brute force 탐색을 해야 한다.

### Reveiew: Mining
A block having been "mined" means that an amount of effort has been put into discovering
a nonce "key" that "unlocks" the block's hash-based "puzzle".

Mining has the property that it is a hard problem to solve while its solution is easy to check and verify. 

It has a customizable difficulty that should adapt to the amount of effort being put forth by the
miners on the network to maintain the average time it takes to mine a block.

Bitcoin adjusts its difficulty every 2,016 blocks such that the next 2,016 blocks should take two weeks to mine.

### Blcok Verification

#### Blockchain?

when we store blocks in memory, we use a plain old vector (resizable array). This is a blockchain
(A non-decreasing, one-way, push-only Vector),
and if it’s actually being used in real life, we’ll receive new blocks from other people: other untrusted people.
We need to make sure they’re being honest, conforming to the protocol.

We aren’t able to validate the information stored in blocks yet — as of now, it’s just arbitrary string data — 
but we can make sure that the blocks themselves look all right. Remember that mining a block is like
finding a key to a lock or a solution to a puzzle. The solution is difficult to come by, but once you know it,
it’s easy to make sure it’s correct.

Given the implementation we have so far, we can also implement a few rudimentary block verification tests.
These steps would be executed whenever we receive a new block from a peer.

Each supposed valid block has a nonce attached to it that we assume took an approximately certain amount
of effort to generate. This "approximately certain amount of effort" is described by the difficulty value.

We will verify four things now:

1. Actual index == stored index value(note that Bitcoin blocks don't store their index)
2. Block's hash fits stored difficulty value(we'll just trust the difficulty for now)(insecure)
3. Time is always increasing(IRL network latency/sync demands leniency here)
4. Actual previous block's hash == stored prev_block_hash value(except for genesis block)

블록 검증 프로세스는 블록체인 네트워크에서 peer로부터 받은 새로운 블록의 무결성을 보장하는 방법이다.
The verification process는 다음 네가지 사항을 확인해야 한다.

1. block의 index가 예상 값과 일치(Bitcoin의 block은 index를 저장하지 않음)
   - 비트코인의 경우 index를 저장하지 않지만, chain에서 존재하는 블록 수를 세어 index를 셀 수 있다.
   - 비트코인은 총유통량이 2100만개로 정해져 있기 때문에 block의 수를 세는데 많은 리소스가 들지 않는다.
   - 반면에 유통량이 정해져있지 않고 무한정 증가할 수 있는 코인들도 있기 때문에 무결성 검사 마다 일일히 세는 것 보단 
     추가 저장공간을 차지하더라도 index를 저장해 놓는 것이 효율적일 수 있다. 
   - 뿐만 아니라 코인마다 디자인 선택은 효율성과 보안 간의 절충, 그리고 의도된 사용 사례, 원하는 탈중앙화 수준, 
     사용 가능한 계산 리소스도 이 결정에 중요한 역할을 한다.
2. block의 hash는 특정 난이도를 충족
3. block의 timestamp는 항상 이전 블록의 timestamp보다 커야함.
4. prev. block의 hash는 chain의 첫 번째 블록(genesis block)을 제외하고 예상 값과 일치

이 프로세스는 블록체인의 무결성을 유지하고 변조 또는 사기를 방지하는 데 중요하다.


## 3. transactions

### Transaction Verification Requirements

https://en.bitcoin.it/wiki/Protocol_rules#.22tx.22_messages

We have to protect against:
- Overspending(Where did the money come from?)
- Double-spending(Is the money available?)
- Impersonation(Who owns the money and who is sending it?)
- ...(there are more, but we're just going to cover these three today)

### The Blockchain as a "Distributed ledger"
This meaning everyone has a copy.

`ledger`? like the history of transactions that have occurred in our cryptocurrency network. 

### Structure of a Transaction
Inputs & Outputs? Inputs are Outputs.

Input = A reference to a previous transaction output, known as UTXO(unspent transaction output)

Inputs being references to previous transactions and Outputs being the recipient addresses and
the amounts being sent to those addresses(Returns to the sender if the transaction fails with the recipient address.
Therefore, the output is at least two).

The input specifies the transaction id and the index of the UTXO it is referring to,
along with the digital signature from the owner of the UTXO to prove ownership and authorize the spending of the funds.

One transaction can have multiple inputs, each referring to a different UTXO,
but the total value of the inputs must be equal to or greater than the value of the outputs.
Any difference between the inputs and outputs represents the transaction fee,
which is a reward for the miner who includes the transaction in a block.

The outputs of a transaction define the recipient addresses and the amounts being sent to those addresses.
There can be multiple outputs in a single transaction, allowing the sender to send funds to multiple recipients in a single transaction.

In summary, the relationship between inputs and outputs in a Bitcoin transaction is that inputs refer to
previous transaction outputs (UTXOs) as a way of proving ownership and authorizing the spending of funds,
while outputs define the recipient addresses and the amounts being sent to those addresses.

### Regular Transactions

For us right now, transactions only contain two important pieces of information:
- Set of inputs(which are unused outputs from previous transactions(UTXO))
- Set of outputs(new outputs that can be used in future transactions)

From here we can caculate :
- the value of the transaction: Σinputs
- the value of the fee: Σinputs - Σoutputs

#### Mining rewards? fee + fixed income(block rewards)
Mining serves the purpose of verifying transactions and adding them to the blockchain.
For performing this function, miners receive a reward, which is composed of two parts:
a block reward and transaction fees.

The block reward is a fixed amount of newly minted bitcoins that are
awarded to the miner who successfully adds a block to the blockchain.
This reward is designed to incentivize miners to participate in the network and to secure the blockchain.
Currently, the block reward is 6.25 bitcoins.

Transaction fees are optional payments made by the users of the network to prioritize the processing of their transactions.
When a user sends a transaction, they have the option of including a fee to incentivize miners to include
their transaction in the next block they mine. Miners will generally prioritize transactions with higher fees
as they want to maximize their profits.

So to summarize, mining compensation in Bitcoin is not just a transaction fee,
but it is a combination of a block reward and transaction fees. The block reward is a fixed amount,
while the transaction fees can vary based on the users' choices and the current demand for block space.

### Coinbase Transactions(genesis block)
Where it all starts(created out of thin air)

Coinbase transactions :
- do not require inputs
- produce an output
- allow the miner to collect all the transaction fees in that block and that block's block reward(coin genesis);


### Transaction update example:

1. Send bitcoins from wallet A to wallet B.
   - UTXO를 Input으로 wallet A의 Output, B의 Output을 생성
   - Output을 생성할때는 script를 사용한다. 사용되는 script는 `P2PKH(pay-to-public-key-hash)` script로, transact하는 사람이 특정
     값으로 해시되는 public key와 private key를 사용해 생성된 digital signature를 제공해서 Output을 생성한다.
   - 결국에는 input도 key와 corresponding해서 script로 만들어졌던 Output이기 때문에 signature가 그대로 남아있으며,
     자금의 ownership을 증명하고 전송을 승인하는데 사용된다. signature는 어떤 방식으로도 변경되지 않는다.
     input이 소비되면 참조하는 이전 Output에 대한 정보와 signature를 사용해 자금을 사용하는 사람이 실제로 정당한 소유자인지 확인한다.
     이 확인이 완료되면 input이 사용된 것으로 간주되고 해당 금액이 수취인의 주소로 이체된다.
   - input이 소비되면 이전 출력에 대한 정보는 네트워크 기능에 더 이상 필요하지 않기 때문에 이체를 승인하는데 사용되는 특정 input에 대한
     직접 엑세스는 불가능해진다.
   - 그러나 transaction 자체는 자금 이체에 대한 영구 기록으로 blockchain에 남아있다. 그렇지만 액세스할 수는 없다.
     (거래내역이 공개되고 투명하기 때문에 소비된 input에 대한 정보 자체는 여전히 blockchain에 존재한다는 점은 주목할 가치가 있다!)
   - Genesis Block의 경우에는 Input을 어떻게 생성할까? genesis block의 input은 생성될 당시 이전 transaction이 없었기 때문에
     참조할 이전 출력이나 확인할 서명이 없다. 여기에 포함된 자금은 "created out of thin air"로 간주되며 네트워크의 일반 거래와 동일한 규칙
     및 제한이 적용되지 않는다. 제네시스 블록을 생성한 사람은 자금의 정당한 소유자로 간주되며 일반적으로 블록을 생성한 채굴자에게 보상하는 데 사용된다.
     제네시스 블록의 소유자는 정당한 소유자이기 때문에 transaction 확인을 위한 서명(input 서명)이 요구되지 않는다(output은 당연히 필요함).
   - Genesis block의 Input에는 일반적으로 miner의 메시지와 같은 임의의 데이터와 일반 트랜잭션을 구별하기 위한 "coinbase" identifier가 포함된다.
   - `Coinbase transaction`의 Input은 UTXO를 참조하는 일반적인 방법과 달리 node를 채굴하여 생성된 Block rewards를 참조한다.
   
2. The transaction is broadcast to the Bitcoin network.
3. It is verified and processed by nodes (also known as validators or miners) in the network.
4. Each node updates its copy of the ledger to reflect the new transaction.
5. And this updated ledger is then propagated to other nodes in the network.
6. Over time, the updated ledger becomes the consensus ledger, which is agreed upon by a majority of the nodes in the network.
7. This consensus ledger forms the basis of the distributed ledger.
8. And each node's copy of the ledger is updated to reflect this consensus.


#### Relationship between transaction and mining(verifying)

In Bitcoin, mining is the process of verifying transactions and adding them to the blockchain as blocks.
Miners compete with each other to verify a set of transactions and add them to the blockchain,
and the miner who succeeds first is awarded a block reward in the form of newly minted bitcoins.
If there are no transactions to verify, there would be nothing for the miners to add to the blockchain,
so they wouldn't be able to mine.


### Meeting Tx Verification Requirements(Problems and their Solutions)

Bitcoin's consensus mechanism: Proof of Work (PoW)

Bitcoin transactions ensure integrity from the following topics by using cryptographic methods:

1. Overspending: Bitcoin uses a transaction ledger called the blockchain to keep track of all transactions.
   The blockchain is a public ledger that is maintained by all nodes in the network,
   and each transaction is verified by the network to ensure that
   `the amount being spent(Outputs) is not greater than the amount available(Inputs) in the sender's wallet.`


2. Double-spending: Bitcoin uses a mechanism called the `Confirmation` process to prevent double-spending.
   This process involves adding the transaction to the blockchain,
   which takes a certain amount of time (typically 10 minutes). During this time,
   other nodes in the network will verify the transaction, and if the same coins are spent again,
   the network will reject the transaction.
   `The "Confirmation" process prevents double-spending by adding only the most PoW mined blockchain to the network.`
   \
   \
   In the case of two miners simultaneously solving the same block and generating two different candidate blocks,
   the network chooses the block with the greatest proof-of-work (PoW) as the valid block.
   This is because the PoW is a mechanism to ensure that adding a new block to the blockchain requires computational effort,
   and the block with the greatest PoW represents the most effort put in. The other block is discarded.
   This is the basic consensus mechanism for most public blockchains, including Bitcoin.
   \
   \
   However, there may be a temporary situation where two blocks are
   added to the blockchain at the same time and the network is split into two separate chains, this is called a `fork`.
   In this case, the network will eventually decide which chain is the correct one and abandon the other chain.
   This process is done through the consensus mechanism(PoW), where the longest chain with the most proof of work is
   considered the authoritative chain.
   \
   \
   In the context of a blockchain, each block contains multiple transactions and is considered
   as a `Confirmation` once it is added to the blockchain and verified by the network. Typically, a transaction is
   considered secure and final after 6 confirmations, but the exact number may vary depending on the network or use case.
   - block을 blockchain에 추가하려면 난이도 목표가 hashing된 값을 nonce값을 바꿔 찾은(mining) 다음 block을 blockchain에 추가함.
   - 즉, 안전하고 최종적인 blockchain으로 간주되기 위해서는 6명 이상의 채굴자와 경쟁해야함.
   - PoW mechanism에서 blockchain의 node network는 most PoW와 most cumulative difficulty가 있는 chain을 선택한다.
   - network는 선택되지 않은 나머지 blockchain은 폐기한다. 폐기된 blockchain은 `orphan` 또는 `stale` 블록으로 네트워크에 저장될 수 있지만,
     Tx를 확인하거나 네트워크의 현재상태를 결정하는데 사용되지는 않는다.
   - 그러나 이러한 block들도 network의 과거 이벤트에 대한 정보를 제공하고, 백업 역할을 하여 네트워크 보안 및 안정성에 도움을 줄 수 있으며,
     새 node 또는 재결합 node에 대한 네트워크 동기화에 도움을 줄 수 있다. 또한 일부 블록체인 네트워크는 이러한 블록을 사용해 miner에 대한
     보상 분배를 결정할 수 있으며 일부는 네트워크 기록을 보존하기 위해 향후 블록에 포함할 수 있다. 따라서 orphan or stale blocks들도
     블록체인 시스템의 전체 기능 내에서 여전히 가치와 목적을 가지고 있다.
   
   Make sure that anyone output is never used as an input more than once.
   This can be done by maintaining a pool of unsepent outputs and rejecting any transaction that
   tries to spend outputs that don't exist in the pool.
   

3. Impersonation: Bitcoin uses digital signatures to verify the identity of the sender and prevent impersonation.
   The sender's public key is used to encrypt the transaction, and the private key is used to decrypt it.
   This ensures that the transaction is initiated by the owner of the wallet and not by an impersonator.
   - How about Input's signature(previous output's signature)?
     The previous signature in the input of a transaction is used to verify the transfer of ownership of the bitcoins
     being spent. It doesn't prevent impersonation by itself, but the combination of the previous signature and
     the digital signatures used to verify the identity of the sender help prevent impersonation
     in the overall blockchain system. The digital signatures, which are created using the private key of the sender,
     provide a way to mathematically verify that the sender of a transaction is indeed the owner of the wallet,
     and the previous signature ensures that the bitcoins being spent have not already been spent in a previous transaction.


4. Scalability: The increasing number of transactions on the blockchain can lead to scalability issues such as
   slow transaction processing times and high fees. Solutions to this problem include off-chain transactions,
   sharding, and lightning networks.


5. Centralization: As mining becomes more difficult, the number of miners participating in the network decreases,
   leading to centralization of the network. Solutions to this problem include using consensus algorithms that
   are less energy-intensive, such as Proof of Stake, and encouraging more miners to participate in the network.


6. Interoperability: Different blockchains use different protocols, making it difficult for them
   to interact with each other. Solutions to this problem include cross-chain bridges and atomic swaps,
   which allow for the exchange of assets between different blockchains.


7. Privacy: Blockchains are designed to be transparent, but this transparency can put users' privacy at risk.
   Solutions to this problem include using privacy-enhancing technologies like zero-knowledge proofs and
   ring signatures, which allow users to conduct transactions while maintaining their anonymity.


8. Security: Blockchains can be vulnerable to attacks, such as 51% attacks,
   where a group of miners control more than half of the network's computational power and can manipulate the blockchain.
   Solutions to this problem include using consensus algorithms that are less vulnerable to 51% attacks,
   such as Proof of Stake, and implementing better security measures to protect the network.

### Updating blockchain
Maintain a list of unspent outputs. This will just be a set of hashes of the unspent outputs.
Note that this does not differentiate between two outputs that are to the same address for the same amount.

This will be fixed later.

validate three more conditions:
- Can we spend the input?
- How many coins are in the output?
- Is the coinbase transaction valid? (We're going to skimp a bit on this check for now.)

### Writing a working example
needs:
1. Create a genesis block with transactions.
2. Mine it.
3. Add it to the blockchain.
4. Create another block with more transactions(particularly some that use transactions from the first block).
5. Mine that one.
6. Add that one to the blockchain.

### Note

Here are some things to take into account about the code:

- The difficulty stored in a block is not validated.
- The value of the coinbase transaction is not validated.
- "Coin ownership" is neither enforced nor existent.
- Two otherwise identical outputs from different transactions are indistinguishable.
- etc