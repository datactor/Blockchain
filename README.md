# Generic blockchain(with pow)
Blockchain = chronological, sequential list of blocks

### Why Rust?

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
1. Concatenate together all the bytes composing the block's fields
   (aside from the hash field) 
   - In the early days, hashcash algorithm(SHA-256^2) was used,
   but now SHA-256 is used. Hashcash is too inefficient due to excessive amount of calculation,
   limited scalability, lack of adaptability, for these reasons, it is less attractive than other algorithms.
2. Generate unique data fingerprint:the hash
3. One-way & Deterministic
   - When you interpret same series of bytes, you will always get same hash. 
     However, you cannot get the series of bytes from the hash.

In a nutshell, a hash algorithm consists of a set of irreversible computations 
that can be performed on a datum to generate a (usually) unique byte sequence.
In a blockchain, each block contains a set of transactions and
a reference to the previous block's hash. The current block's hash is calculated by
applying the hash function to the combination of the block's transactions and
the previous block's hash. This creates a chain of hashes that is resistant to tampering,
as any change in the block data will result in a different hash.
`SHA2`, `SHA3(Keccak-256)`

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
역으로 찾을 수 없어 O(n)의 brute force 탐색을 해야 한다.

#### Reveiew: Mining
A block having been "mined" means that an amount of effort has been put into discovering
a nonce "key" that "unlocks" the block's hash-based "puzzle".

Mining has the property that it is a hard problem to solve while its solution is easy to check and verify. 

It has a customizable difficulty that should adapt to the amount of effort being put forth by the
miners on the network to maintain the average time it takes to mine a block.

Bitcoin adjusts its difficulty every 2,016 blocks such that the next 2,016 blocks should take two weeks to mine.

#### Blcok Verification

##### Blockchain?

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

### Transaction Verification REquirements

https://en.bitcoin.it/wiki/Protocol_rules#.22tx.22_messages

We have to protect against:
- Overspending(Where did the money come from?)
- Double-spending(Is the money available?)
- Impersonation(Who owns the money and who is sending it?)
- ...(there are more, but we're just going to cover these three today)

#### The Blockchain as a "Distributed ledger"
This meaning everyone has a copy.

`ledger`? like the history of transactions that have occurred in our cryptocurrency network. 

##### Structure of a Transaction
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


Transaction update example:

1. Send bitcoins from wallet A to wallet B.
2. The transaction is broadcast to the Bitcoin network.
3. It is verified and processed by nodes (also known as validators or miners) in the network.
4. Each node updates its copy of the ledger to reflect the new transaction.
5. And this updated ledger is then propagated to other nodes in the network.
6. Over time, the updated ledger becomes the consensus ledger, which is agreed upon by a majority of the nodes in the network.
7. This consensus ledger forms the basis of the distributed ledger.
8. And each node's copy of the ledger is updated to reflect this consensus.


##### Relationship between transaction and mining (verifying)

In Bitcoin, mining is the process of verifying transactions and adding them to the blockchain as blocks.
Miners compete with each other to verify a set of transactions and add them to the blockchain,
and the miner who succeeds first is awarded a block reward in the form of newly minted bitcoins.
If there are no transactions to verify, there would be nothing for the miners to add to the blockchain,
so they wouldn't be able to mine.

#### solution
Bitcoin transac*t*ions ensure integrity from the following topics by using cryptographic methods:

1. Overspending: Bitcoin uses a transaction ledger called the blockchain to keep track of all transactions.
   The blockchain is a public ledger that is maintained by all nodes in the network,
   and each transaction is verified by the network to ensure that
   the amount being spent is not greater than the amount available in the sender's wallet.


2. Double-spending: Bitcoin uses a mechanism called the "Confirmation" process to prevent double-spending.
   This process involves adding the transaction to the blockchain,
   which takes a certain amount of time (typically 10 minutes). During this time,
   other nodes in the network will verify the transaction, and if the same coins are spent again,
   the network will reject the transaction.


3. Impersonation: Bitcoin uses digital signatures to verify the identity of the sender and prevent impersonation.
   The sender's public key is used to encrypt the transaction, and the private key is used to decrypt it.
   This ensures that the transaction is initiated by the owner of the wallet and not by an impersonator.

#### Additional problems and their solutions

1. Scalability: The increasing number of transactions on the blockchain can lead to scalability issues such as
   slow transaction processing times and high fees. Solutions to this problem include off-chain transactions,
   sharding, and lightning networks.


2. Centralization: As mining becomes more difficult, the number of miners participating in the network decreases,
   leading to centralization of the network. Solutions to this problem include using consensus algorithms that
   are less energy-intensive, such as Proof of Stake, and encouraging more miners to participate in the network.


3. Interoperability: Different blockchains use different protocols, making it difficult for them
   to interact with each other. Solutions to this problem include cross-chain bridges and atomic swaps,
   which allow for the exchange of assets between different blockchains.


4. Privacy: Blockchains are designed to be transparent, but this transparency can put users' privacy at risk.
   Solutions to this problem include using privacy-enhancing technologies like zero-knowledge proofs and
   ring signatures, which allow users to conduct transactions while maintaining their anonymity.


5. Security: Blockchains can be vulnerable to attacks, such as 51% attacks,
   where a group of miners control more than half of the network's computational power and can manipulate the blockchain.
   Solutions to this problem include using consensus algorithms that are less vulnerable to 51% attacks,
   such as Proof of Stake, and implementing better security measures to protect the network.

## 4. smart contracts