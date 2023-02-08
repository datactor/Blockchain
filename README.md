# Generic blockchain(with PoW)
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
  1. 입력값이 더 클 경우 탐색을 중단하고 블록을 chain에 push한다. 이것이 mining이다.
  2. 크지 않을 경우 continue해서 계속 탐색
  
brute force로 탐색하지 않고 O(1)로 계산하고 싶겠지만, 앞에서도 강조한 것처럼 block header의 hash는 비가역성을 가졌기 때문에
역으로 찾을 수 없어 O(n)의 탐색을 해야 한다.

## 3. transactions

## 4. smart contracts