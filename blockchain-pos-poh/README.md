# BLOCKCHAIN - PoS & PoH(SOL)

## Structure

### 1. state structure
A state structure that maintains the state of the program. (e.g. accounts, blockchain)

This state can be anything from a simple variable to a complex data structure.
State is stored on the Solana blockchain, a distributed ledger maintained by all nodes in the Solana network.

### 2. program code
Program code that defines state manipulation rules. (e.g. mint, sys, token, stake)

This code runs on the Solana network whenever a transaction is submitted to the network.
Program code can read and write to the state, and can also emit log messages.

### 3. Actor (validator, participants, nodes)
Modify the state of a program by executing the state manipulation rules defined by the program. (e.g. Full node, validator, archiver, edge, bootstrapper)

## Configuration
### 1. Nodes
These configuration files are used to specify parameters such as network address, port numbers, performance settings, and security settings.
#### Full node
The full node configuration file is used to configure the behavior of a full node in the Solana network. It includes parameters such as the network address, port numbers, and file paths for the node's data and log files. It also includes settings for the node's performance and security, such as the number of threads to use for transaction processing and the amount of memory to allocate for the node's database.

#### Validator
The validator configuration file is used to configure a validator node in the Solana network. It includes parameters such as the node's validator identity and staking information, as well as the network address and port numbers for the node. It also includes settings for the node's performance and security, such as the number of threads to use for block production and the maximum transaction rate the node can handle.

#### Archiver
The archiver configuration file is used to configure an archiver node in the Solana network. It includes parameters such as the node's network address and port numbers, as well as the file paths for the node's data and log files. It also includes settings for the node's performance and security, such as the maximum amount of disk space to use for storing historical blockchain data.

#### Edge
The edge configuration file is used to configure an edge node in the Solana network. It includes parameters such as the node's network address and port numbers, as well as settings for the node's performance and security, such as the maximum number of requests the node can handle and the maximum amount of memory to use for request processing.

#### bootstrapper(optional)
This is a type of node that helps new nodes join the Solana network by providing them with information about the existing nodes in the network.

### 2. DB
This category includes the components and files related to the storage and management of blockchain data.
These components and files help ensure the accuracy and consistency of the blockchain data and improve the availability and reliability of the network.

Using RocksDB, which supports atomic read/write and snapshots of key value pairs.

#### Blockchain DB
The blockchain DB is a database that stores the state of blockchain on the Solana.

- In the real Solana network, the blockchain data is not stored in a separate configmap, but rather in the distributed ledger itself.
- In the Solana architecture, both the account database and the blockchain are stored in a single global state.
  One huge global db, distributing this.
  It is distributed to the nodes of the network in the form of accountdb and blockchaindb.
  And each node gets distributed accountdb and blockchaindb from configmap.
  Then, it is accessed to perform verification and operation.
#### Snapshot Archives
Todo!();
A snapshot archive is a compressed file that contains a copy of the Solana blockchain data at a specific point in time. Snapshot archives are used to help new nodes quickly synchronize with the network, by providing them with a pre-built copy of the blockchain data that they can use to bootstrap their local copy. Snapshot archives can be created manually or automatically by a validator node using the Solana CLI.

#### Accounts DB
The Accounts DB is a database that stores the state of all accounts on the Solana blockchain. It is used by full nodes and validators to validate transactions and produce new blocks. The Accounts DB can be configured to use different storage backends, such as local disk storage or cloud-based storage services like Amazon S3.

#### Vote Account
A vote account is a special type of account that is used by validator nodes to participate in the consensus process and produce new blocks. Vote accounts are associated with a specific validator node, and are used to store the node's staking information and vote tokens. Validators must stake a certain amount of SOL tokens to participate in the consensus process, and the amount of stake determines the node's voting power and ability to produce new blocks.

#### Replicator
A replicator is a type of node that helps distribute and replicate the Accounts DB across the Solana network. Replicators store a copy of the Accounts DB and use a gossip network to exchange updates with other replicators and nodes in the network. Replicators help improve the availability and reliability of the Accounts DB, and can help reduce the time and bandwidth required to synchronize new nodes with the network.

### 3. CLI
The CLI is a tool that allows developers and node operators to interact with the Solana network and perform various operations, such as creating and deploying programs, querying the blockchain, and managing wallet accounts. The CLI configuration file is used to specify parameters such as default network address and port numbers, as well as developer-specific settings.

#### CLI
The CLI configuration file is used to configure the Solana command-line interface (CLI). It includes parameters such as the default network address and port numbers, as well as settings for the CLI's performance and security, such as the default transaction fee and the maximum transaction rate the CLI can handle.


## Programs
### sys program
시스템 내부에 account가 있으며, 블록체인 업데이트에 및 가장 로우 레벨인 프로그램 계정을 생성하는데 사용된다.
solana의 blockchain update를 직접적으로 할 수 있는 수단은 오직 sys program이다.
blockchain을 업데이트하는데 있어서 account가 필요하지 않다. 그렇지만 sys 프로그램은 블록체인의 가장 low-level 작업을 수행하기 때문에
(예를 들면 program account를 생성하는데 sys account가 필요하다.) 계정이 필요하다.
추가로 sys account는 초기의 sys program state를 나타내는 역할도 한다.
blockchain이 시작되면 sys program이 메모리에 로드되고 state는 해당 account의 데이터로 표시된다.
이 account는 blockchain을 부트스트랩하고 sys program이 제대로 로드되고 blockchain을 업데이트할
준비가 되었는지 확인하는데 필요하다.

### mint program
genesis에서 account 생성, 새로운 Sol token을 생성하는데 사용되며 Mint program에서 관리한다.
mint의 잔액은 총 sol token수와 직접 관련된다. 모든 solana token은 Mint account를 거쳐서 가기 때문이다.
solana가 처음 발행되면 mint account로 입금된다. Validator가 block보상을 받는 것 역시 mint로부터 입금되고,
tx 수수료에서 보상을 제외한 mod 또한 mint account로 입금된다.
tx에서 필연적으로 mint에게 입금되는 금액은 원금의 Valdator가 원금을 Validating할 때 tx 유효성 검사 및 합의 프로세스의 일부로
자동으로 발생하므로 validator는 fee를 별도로 확인할 필요가 없고 이부분을 따로 validating하지 않는다(재귀적 validating 방지).
민트 프로그램에서 mint account로 입금시킬 때는 fee를 부과하거나 validating을 요구하지 않고 진행된다.
fake tx나 유저가 임의로 mint에 입금시켜 발행량을 조작할 수 없으며, mint program은 유효한 유저의 tx라도 거절할 수 있는 기능이 있다.
solana의 유동성을 조절하는 역할을 하며 이것을 위해 token을 소각하는 경우에도
mint account의 balance에서 소각한다.
(tx에서 fee의 목적지: 1. to validator, 2. mint account, 3. burnt) 

여기서 주의해야 할점은 mint에서 수행하는 소각은 거시적 유동성 관리목적이긴 하지만 macro liquidity managing으로,
자동적으로 소각되는 양으로 천천히 수행된다. 숏텀에 급진적으로 유동성을 조절하기 위해 destruct하는 역할은 Token program에서 수행한다.

+add)
1. Token Metadata: Depending on use case, you may want to include additional metadata about the token, such as a name, symbol, or decimal places. This information can be stored in the token account's data field, and can be read by other programs that interact with the token.

2. Token Supply Limit: If I want to set a limit on the total supply of tokens that can be issued. This can be done by setting a maximum supply value when creating the token account.

3. Token Burn: In addition to controlling the token supply by minting.

4. Token Freezing: If I want to implement the ability to freeze certain token accounts. This can be useful for preventing fraudulent or unauthorized use of the token.

5. Token Transfers: Need to implement the logic for transferring tokens between accounts. This includes validating the sender's balance, checking the receiver's account, and updating the account balances.

6. Token Minting Fees: Plan to charge a fee for minting new tokens, I have to implement the logic for calculating and deducting this fee from the minted tokens.

### token program
genesis에서 account 생성, Token program은 토큰의 생성, tx, destruction을 담당한다.
총 공급량, 개별 유저 잔액 및 토큰 메타데이터를 포함하여 토큰에 대한 정보를 저장하려면 계정이 필요하다.
mint 프로그램은 주로 토큰 생성 및 burnt를 담당하는 반면, Token program은 유통 중인
토큰 공급 관리, 토큰 소유권 추적 및 토큰 tx와 관련된 모든 규칙 시행을 담당한다.
여기에는 개별 유저의 balance 관리 및 유통 중인 토큰의 총 공급량 추적이 포함된다.

얼핏 보면 mint program과 token program을 나눈 이유가 명확하게 보이지 않을 수 있다.
개인적인 의견이지만 개입 없이 거시적 유동성 관리를 위해 invisible hand처럼 macro program을 mint로 두고, 무언가 조정이 필요한 issue가 있을 경우에
macro를 건드리지 않고 조치할 수 있는 수단인 transparent hand를 만들어 토큰을 관리할 때 더 많은 유연성과 사용자 정의를 허용하기 위함으로 보인다.

또한 mint와 token 프로그램을 분리함으로써 개발자는 다양한 type과 properties, func를 가진 토큰을 생성할 수 있으며,
solana chain에서 다양한 dapp 및 사용을 장려하는 수단인 것 같다.

mint program과 토큰 program의 주요 차이점 중 하나는 mint program은 필요에 따라 새 토큰을
생성하고 burnt에 중점을 둔 역할을 하는 반면 Token program은 유통 중인 토큰을 관리하고 tx가 유효하고 준수되는지 확인하고,
잔액 및 메타데이터를 포함하여 개별 토큰 자체를 관리하는 역할을 한다는 것이다.

### stake program

### BPF(Berkeley Packet Filter)
솔라나 위에 Dapp을 구축할 수 있게 만들어주는 핵심 구성 요소.
Rust, C, AssemblyScript를 비롯한 프로그램으로 smart contract를 작성하고 배포할 수 있는 경량 가상 머신.
#### Serum program
user가 Solana blockchain에서 토큰을 거래할 수 있는 탈중앙화 거래소(Dex)
serum 프로그램은 솔라나의 tx만 가능하기 때문에 Erc20을 사용하지 않으며,
대신 자체 프로토콜(serum protocol)을 사용한다. 솔라나의 체인 상에서 동작하도록 최적화 되어있기 때문에
빠르고 효율적인 tx처리가 가능하다.
### Wormhole program
교차 체인 상호 운용성을 위해 서로 다른 블록체인 간에 자산을 전송 할 수 있게 해줌.
서로 다른 블록체인 네트워크 간에 자산과 데이터를 전송할 수 있는 cross-chain protocol이기 때문에,
wrapping된 토큰을 사용하여 이더리움 기반 자산을 솔라나의 체인에 연결할 수 있도록 지원함.(Wormhole Token Bridge라고 한다)
(e.g. ERC20 프로토콜을 사용해서 tx한다고 하더라도, 실제로 wormhole 프로그램에서 tx하는 수단은 WTB이다.
즉 솔라나를 wrapping하여 custom protocol인 WTB에서 tx할 수 있는 token으로 만들고,
다른 프로토콜로 보낼때나 그로부터 받을때는 그에 맞는 방식으로 wrapping하여 lock하거나 unwrapping한다.)

### Raydium program
또 다른 DEX로 유동성 pool 및 yield farming을 가능하게 해준다.


#### Transaction processing

#### Smart contract language: Rust
#### Token standard
The token standard that will be used for creating and managing different types of tokens on your blockchain.
#### Node software
The software that can run different types of nodes on your blockchain, such as full nodes, validators, and edge nodes.
#### Wallet software
The software that can manage private keys and interact with your blockchain.
#### Development tools
The tools and libraries that can be used for developing and deploying smart contracts, interacting with the blockchain, and testing the network.


### Todo!();
이 코드는 샤딩되지 않은 input_dbs를 기존의 샤딩된 샤드들 중 완전하지 않은 샤드 1개와 결합해서 새로운 샤드를 만들고,
계속해서 새로운 샤드들을 추가로 만들거나, 기존의 샤딩된 샤드들이 없다면 새롭게 input_dbs들을 샤드로 나누는 작업임.

몇가지 문제가 있어보인다.
1. 
```rust
// Try to open all existing shards
for shard_path in shard_paths {
    while let Ok(db) = DB::open(&shard_opts, format!("{}_shard_{}", shard_path, shard_count)) {
        shard_dbs.push(db); 
        last_shard_index = shard_count;
        shard_count += 1;
    }
}
```

위의 코드에서 이미 생성된 샤드들이라면 미리 모든 샤드들을 가져올 필요는 없어.
완전하지 않은 샤드 하나만 가져오면 되고.
즉 샤드들이 존재한다면,
shard_paths들에 샤드가 존재한다면(이 부분은 파일/폴더 읽기 시스템을 사용해서 읽는 것이 좋겠지 않을까?),
순차적으로 샤드를 추가했다면, 0부터 시작해서 마지막 샤드 순으로 추가되었을테니까,
shard_path별로 마지막 인덱스의 샤드들의 내부만 열어봐서,
```rust
let max_shard_cap = 200_000_000; // Maximum capacity per shard
let mut last_shard_name = String::new();
// find insufficient shard
for shard_path in shard_paths {
    // After getting the list of folders in "shard_path" with the OS command,
    // set the last index file name as last_shard_name
    let Some(tmp_shard_name) = todo!(); {
        let last_shard_opts = Options::default();
        let last_shard = DB::open(&last_shard_opts, tmp_shard_name).unwrap();
        let mut tmp_capacity = 0;
        let mut last = last_shard.iterator(IteratorMode::Start);
        while let Some(Ok((key, value))) = last.next() {
            tmp_capacity += value.len();
        }
        if tmp_capacity < max_shard_cap {
            last_shard_capacity = tmp_capacity;
            last_shard_name = tmp_shard_name;
        }
    }
}
```
위의 코드를 거쳤다면,
last_shard_capacity가 밝혀졌을 것이다(초기값인 0 아니면 기존의 insufficient shard의 값)
그리고 last_shard_name도 규칙에 의해 순차적으로 저장될 위치이거나 insufficient shard로 변경되었을것임.

그렇다면 이제
shard_dbs에 push하면 된다.

```rust
let db = DB::open(&shard_opts, last_shard_name).unwrap();
shard_dbs.push(db);
```

이 시점에서 shard_dbs 내에는 반드시 하나의 샤드만 존재.


그 이후에
```rust
    // Merge all input databases into shard databases
    let mut shards_per_path = vec![shard_count / shard_paths.len(); shard_paths.len()];
    let mut current_path_index = 0;
    for input_db_path in input_dbs {
        let input_opts = Options::default();
        let input_db = DB::open(&input_opts, input_db_path).unwrap();
        let mut iter = input_db.iterator(IteratorMode::Start);

        while let Some(Ok((key, value))) = iter.next() {
            let shard_path = shard_paths[current_path_index];

            let shard_index = calculate_shard_index(&key.to_vec().to_vec()[..], shards_per_path[current_path_index], shard_size, last_shard_index, last_shard_capacity);
            let mut shard_db = shard_dbs.get_mut(shard_index).unwrap();


            let mut batch = WriteBatch::default();
            batch.put(&key.to_vec()[..], &value.to_vec()[..]);

            let write_opts = WriteOptions::default();
            shard_db.write_opt(batch, &write_opts).unwrap();

            // Update last shard index and capacity
            last_shard_index = shard_index;
            last_shard_capacity += value.len();

            // If last shard is full, create a new one
            if last_shard_capacity >= shard_size {
                last_shard_capacity = 0;
                shard_count += 1;
                if shard_count % SHARDS_PER_PATH == 0 {
                    current_path_index = (current_path_index + 1) % shard_paths.len();
                    shards_per_path[current_path_index] += 1;
                }
                let db = DB::open(&shard_opts, format!("{}_shard_{}", shard_path, shard_count)).unwrap();
                shard_dbs.push(db);
            }
        }
    }
```
위의 코드로 샤드들을 추가

2. shard indexing  
   shard_paths: &[&str] 에서 SHARD_PER_PATH에 따라서 path마다 인덱싱을 유기적으로 해야한다.
   예를 들면 처음 생성하는 샤드의 path와 다음 생성하는 샤드의 path의 관계에 명백한 규칙이 있어야 다음 shard들을 구체적으로 정확하게 찾아갈 수 있음.
   1. 최초의 path부터 SHARD_PER_PATH만큼 shard의 개수를 채우고 다음 path에서 shard 생성
      - 초기 단계에서 선택할만한 옵션이지만 추후에 고가용성, 내결함성, 로드 분산이 고르지 않고 처리시간이 일관적이지 않을 수 있음.
   2. 알려진 모든 path에 고르게 분포하기 위해 path들을 indexing하는 규칙을 세워, 샤드 하나가 업데이트 되면 다른 path를 찾아가  looping하는 방식으로 업데이트하는 방식.
      - 부하분산, 내결함성, 확장성이 보장되지만 초기부터 모든 premise들을 켜둬야 하는 resource 낭비가 있음
   3. 2중 shard_path matrix. 1번 모델 집합 내부에 2번 모델 집합이 있는 타입?
      예를 들면 확장성과 내결함성, 고성능을 고려하기 위해 2번 집합으로 샤드를 직접 업데이트하고, 이 2번 타입의 집합을 여러개 만들어서,
      2번 집합을 원소 갖는 1번타입. 1번 집합의 원소인 2번집합이 모두 꽉차면, 다음 2번 집합으로 넘어가는 방식.
      이렇게 한다면 로드가 적을 경우에는, 나머지 premise들의 리소스를 절약할 수 있음.
      1,2번의 장점을 합쳐놓은 타입이지만 구현하기 복잡하고, 추가적인 오버헤드 관리가 필요함.
   4. half capacity를 갖는 1번 모델 타입?
      path 정해진 shard_capacity를 모두 채우지 않고 half까지 채우면서, 1번 모델 방식으로 순차 증가 방식.
      예를 들면 "qqq" path에 max capa가 32개의 shard라면, 16개의 샤드를 채울때까지 샤드 생성은 "qqq" path에서만 진행하고,
      16개가 채워졌을 경우에는 다음 path로 넘어가서 채움. 모든 path의 capa가 half로 채워지면, 그때부터 2번 방식으로 진행하기.
      리소스사용과 확장성의 균형을 맞춰보자.
   
   4번으로 가보자


3. shard Option

shard를 연다는것은 두가지 경우인데,
단순히 읽고 account_ID에 대한 정보를 가져오거나,
account_ID의 정보를 변경 또는 추가하거나 두 경우이다.

그렇다면 update_shard와,
read_shard를 사용해야 할까?
update_shard일 경우 writeOption을 사용하고,
read를 쓸경우 readOption을 사용하고.

예를 들어서 로그인을 해서 기본적으로 readOption으로 가져왔으면,
tx나 id생성, stake등의 이슈가 있을때는 write_option으로 다시 가져와야하잖아?
이렇게 할 바에 처음부터 write_option으로 가져오는게 나을까?

그렇지만 처음부터 write_option으로 가져오면,
WAL, sync, lock등의 오버헤드가 많이 생긴다.

- rocksDB에서는 set_option() 메서드를 사용해서
이미 열려있는 db 인스턴스의 쓰기 옵션을 변경할 수 있다.
그러므로 기본적으로 read_option으로 가져오고, 필요한 경우에만
write_option을 사용하도록 하자.


#### 3월 18일 

##### ShardPath와 Database의 관계 정리

먼저 ShardPath부터 들어가보자.

1. new() -> 만들수 있는 최대의 샤드를 인덱싱해서 해시맵으로 올려놓음. 아직 실제 database는 만들지 않았음.
2. get_shard() -> 불러올 shard의 위치를 반환해준다.

3. add_shard() -> 이미 new()에서 만들 수 있는 최대의 샤드 리스트가 정해졌기때문에
   call 되는 순간은 scale-out 등의 이슈가 있을 때 뿐이다. 이를 수행하면, indexing을 필수적으로 한번 더 수행해야함.

4. index_shard() -> 리밸런싱 주기나, scale out등의 이슈가 있을 때 수행한다.

5. remove_shard() -> 기존의 샤드를 제거한다. 다른 곳에 옮겼을 경우에만 수행한다. 성공적으로 지워지면 indexing 필수.

6. move_shard() -> 샤드의 path를 변경할 때 수행한다. 옮기고 나서 remove_shard()를 수행한다.

여기까지가 shardPath 메소드의 역할이고,

database의 역할을 보자.

1. get() -> 샤드path에서 정해진 인덱스에 대한 샤드를 불러온다. 즉, ShardPath::get_shard() 메소드의 서브루틴 함수이다.
   없다면 options.create_if_missing(true); 명령으로 생성한다. 때문에 new() method가 필요없다.

2. put() -> get으로 가져온 실제 database에 lock을 걸고 write_option으로 변경하여, db를 업데이트한다.
   여기서 추가해야할 기능은, shard의 capacity를 찾고, batch로 올렸을 경우에 capacity가 넘치지 않는 경우에만 batch한다.
   넘칠 위험이 있는 경우 다음 인덱스의 샤드를 ShardPath::get_shard() 메소드로 가져와서 분배한다.
   어떻게 분배해야 할까?
   (1) 샤드에 데이터 하나씩 batch하는 경우
   분배가 아닌 다음 샤드로 넘긴다.
   (2) 샤드에 데이터가 여러개 batch되는 경우(이러한 경우는 queue로 업데이트 요청을 지연시켜서 일괄처리 하지 않는 이상 없다.)
   어떤 방법이 효율적일지 모르지만 쉬운 방법은, 다중데이터의 batch를 취소하고
   개별데이터 batch 요청으로 전환해서 들어갈만큼 넣고, 꽉차면 다음 샤드로 넘긴다.

3. remove_database() -> 기존의 샤드를 제거한다. remove_shard()의 서브루틴으로 지정한다.
   데이터가 옮겨졌을 경우가 확인 될 때만, remove를 수행한다. 여기서 옮겨진 데이터베이스를 한번더 불러와서 double check한다.

// 4. move_database() -> 새로운 path에 샤드를 생성하는 절차는 여기서는 필요없다. get() 메서드로 저절로 생성시킬 수 있다.

// 5. index_database() -> 여기서 ShardPath::index_shard() 메서드는 샤드의 위치는 고정하고 index만 변경하는 것이기 때문에,
//    database에서 index_shard() 메서드를 수행할 필요는 없다.

#### 3월 19일

##### ShardPath::new() 구현

##### ShardPath::index_shards() 구현
데이터 지역성(hotspot) 방지, 결정론적, 확장성 유지
샤드와 path가 동적일 경우에도 샤드index에 대해서 결정적임을 보증하면서,
추가적인 리소스 낭비를 줄이기 위해 accountID를 shard index로 사용하는 range sharding 사용한다.
또한 accountID를 해싱하여 노드 집합간에 키를 배포함으로써 데이터를 고르게 분산시켜
핫스팟을 방지하는데 도움을 준다. 또한 해시 링에서 노드를 동적으로 추가 및 제거할 수 있으므로,
확장성 향상과, 데이터의 균형을 재조정할 수 있게 한다.

##### dive to consistentHashRing
https://github.com/datactor/rust-problem-solving/blob/main/forge/distributed_data_management/consistentHashRing.md

#### 3월 20일 - 21일

##### dive to JumpConsistentHashRing
https://github.com/datactor/rust-problem-solving/blob/main/forge/distributed_data_management/jumpConsHash.md

#### 3월 22일

##### Update the dive to sharding
https://github.com/datactor/rust-problem-solving/blob/main/forge/distributed_data_management/sharding.md

#### 3월 23일

##### Update the get_shard() method

##### Implementing the rebuild_path() method

##### move_shard()
Consistent Hash Ring을 사용하기 때문에 account_id의 분포에 따른
move 메소드는 필수적이지 않다. 그렇지만 인기 있는 account와 인기없는 account의
차이에 따른 로드 차이는 Consistent Hash로 해결할 수 없다.
이럴 때 move_shard()로 샤드의 위치를 옮겨준다. 이것은 account별 데이터 쿼리요청량에 따라서 알고리즘을 설계하고
동적 로드밸런싱을 구현해야 하기 때문에 현재 단계에서는 적절하지 않다.

직접적으로 리소스를 사용하는 것은 shard가 아니라 node, 즉 path이기 때문에
가장 로드가 많이 걸리는 path내부의 가장 큰 로드의 shard와,
가장 로드가 적게 걸리는 path내부의 가장 적은 로드의 shard를 맞교환하는 식으로 구현한다.

move shard에 대한 몇가지 기준은 다음과 같다.
1. 로드가 높은 샤드를 로드가 낮은 노드로 이동하여 시스템 전체의 로드 균형을 조정한다.
2. 더 빠른 디스크 액세스 또는 더 많은 사용 가능한 메모리와 같은 더 나은 성능 특성을 가진 노드로 샤드를 이동시킨다.
3. 네트워크 대기 시간을 줄이기 위해 대부분의 사용자에게 지리적으로 더 가까운 노드로 샤드를 이동시킨다.
4. 급증하는 수요를 처리하기 위해 샤드를 CPU 또는 대역폭과 같은 사용 가능한 리소스가 더 많은 노드로 이동시킨다.
5. 네트워크 혼잡이 적거나 네트워크 오류가 적은 노드로 샤드를 이동시킨다.

추후에 머신러닝 등을 통해서 예측 동적 로드밸런싱 자동화에 대해서 알아보자.

##### remove_shard()
remove_shard() 역시 마찬가지이다. 현재 입장에서는 필수적이지 않은 기능이다.
추후에 move_shard()메소드가 구현 되었을때 고려하자

##### Modifying the index_shards() method
기존의 방식은 indexing마다 ConsistentHashRing 초기화였으나,
deterministic이 불안해지고 불필요한 계산 오버헤드가 생김.

ShardPathd::new()에서 ConsistentHashRing을 초기화하고
indexing에서는 생성된 ConsistentHashRing을 불러와서 수정하는 방식으로 변경.
Shard는 ConsistentHashRing에서는 node이기 때문에 shard가 추가되는
rebuild_path() 단계에서만 노드 추가.


