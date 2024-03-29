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

부트스트랩은 초기에 프로그램을 init하는 역할을 한다.
부트스트랩 노드는 네트워크의 초기 설정에서 중요한 역할.
솔라나와 같은 큰 분산 시스템에서도 부트스트랩 노드는
네트워크에 참여하는 노드가 서로를 발견하고 통신할 수 있도록 돕는 역할을 한다.

이 경우에는, 부트스트랩 노드는 종종 네트워크의 '리더' 또는 '관리자' 역할을 하는
노드에 연결하여, 필요한 노드들이 서로를 알아볼 수 있도록 돕는다.
이렇게 하면, 초기 연결 설정을 간소화하고, 네트워크가 안정적으로 작동하도록
보장할 수 있다.

따라서 부트스트랩 노드를 사용하여 필수 노드들 간의 연결을 초기화하고 확인하는 방법은,
이러한 유형의 시스템에서 매우 효과적인 방법이 될 수 있다.
그러나 이 방법을 사용하면서도, 부트스트랩 노드가 너무 많은 부하를 받거나
단일 실패 지점이 되지 않도록 주의해야 한다.

이런 문제를 방지하기 위해, 부트스트랩은 일반적으로 다른 노드들에 비해 더 높은 용량과
성능을 가지며, 부트스트랩 노드는 종종 여러 개로 구성되거나(이 경우 이 노드들 간에도
정보 공유가 이뤄지며 지속적으로 동기화 해야한다. 이는 노드가 실패했을 때 다른 노드가
그 역할을 대신 수행할 수 있도록 해줘 안정성과 가용성을 높인다.), 부트스트랩 노드의 역할을
하는 노드들이 동적으로 선택되는 등의 방법이 사용될 수 있다.
이는 부트스트랩 노드의 redundancy를 확보하는 한 가지 방법이며, 이를 통해 네트워크의
안정성과 가용성을 높일 수 있다. (Raft 또는 Pbft 사용 고려. 여기서는 redundancy를
줄이는 것이 리소스 효율적, PBFT는 많은 redundancy 필요하고 관리자 노드는
byzantine일 가능성이 매우 낮기 때문에 Raft의 사용 고려)
또한 부트스트랩 노드가 너무 많은 부하를 받는다면, 동적으로 여러 부트스트랩을
동시에 구동할 수 있는 로드 밸런싱 전략을 사용하는 것도 좋은 선택이다.
이는 추가적으로 네트워크의 확장성이 향상될 수 있다.

초기에 필수 노드들 간에 연결을 부트스트랩에서 확인하기 위해 "중앙 집중식 방식"
으로 필수 노드에 부트스트랩에 연결시키고, 실행상태 확인 후에
부트스트랩 프로세스가 완료되면,
이후에 탈중앙화 방식으로 노드들은 독립적으로 연결된다.(여기서 validator를 포함한 노드간
연결은 Raft로는 byzantine을 방지하지 못하기 때문에 PoS와 함께 사용하거나,
Pbft가 적절해보인다.)

솔라나는 "고립 방지" 문제를 해결하기 위해 "health check" 프로토콜을 사용한다.
이 프로토콜은 네트워크 노드 간의 연결을 유지하고, 노드의 상태를 지속적으로 모니터링하고,
정기적으로 헬스 체크를 수행하여 문제가 생긴 노드를 빠르게 감지하고 대응하여
잠재적인 문제를 식별하며, 네트워크 내에서 노드의 상태를 알리는 데 사용된다.

솔라나 네트워크는 모든 노드가 서로 독립적으로 연결되는 방식을 취하며,
각 노드는 자신의 주변 노드와 독립적으로 통신한다. 이것은 솔라나 네트워크가
대규모로 확장될 수 있게 하고, 네트워크 내의 임의의 노드가 실패하더라도
네트워크 전체가 계속 작동하도록 보장한다.

솔라나의 부트스트랩 프로세스는 새 노드가 네트워크에 가입하고 네트워크의
상태를 파악하는 데 필요한 정보를 얻을 수 있도록 돕는 역할을 한다.
새 노드는 부트스트랩 노드(일반적으로 미리 정의된 트러스티드 노드 집합 중 하나)에
연결하여 네트워크의 현재 상태와 네트워크의 다른 노드에 대한 정보를 얻는다.
그러나 한 번 부트스트랩 프로세스가 완료되면, 각 노드는 네트워크의 다른 노드와
직접 통신하게 된다.

따라서, 솔라나의 경우 필수 노드 또는 관리자 노드들이 먼저 부트스트랩에
연결되는 방식을 사용한다. 그러한 부트스트랩 프로세스가 성공적으로 끝나면,
모든 노드는 독립적으로 서로 연결되며, 이는 네트워크의 탄력성과 확장성을 높인다.


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
유동성 pool 및 yield farming을 가능하게 해준다.

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


이 코드는 샤딩되지 않은 input_dbs를 기존의 샤딩된 샤드들 중 완전하지 않은 샤드 1개와 결합해서 새로운 샤드를 만들고,
계속해서 새로운 샤드들을 추가로 만들거나, 기존의 샤딩된 샤드들이 없다면 새롭게 input_dbs들을 샤드로 나누는 작업이다.

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

#### 3월 24일

##### Arc<Mutex<Shard>> new()
실제 솔라나에서 database를 ArcMutex로 접근하는 것은 validator이지만,
validator는 data에 접근할 수 있지만 접근할 위치는 모른다.
즉 ShardPath나 database의 객체를 가지고 있는 주체는 validator가 아니다.
validator는 그저 네트워크의 공용 db에 접근 권한만 갖고 있다.

때문에 validator는 해당 ID가 저장된 database(shard)의 위치를 알아내기 위해
솔라나 네트워크에 요청을 보내야한다. 솔라나의 ShardPath 객체는 공용 네트워크가 아니라
솔라나 네트워크의 중앙 시스템에 저장되어 있기 때문이다. 이유는 순식간 변할 수 있는 인덱스를
노드의 업데이트마다 동시에 반영하여 배포하기 어렵기 때문이다.

솔라나에서 ShardPath는 sys program에서 관리한다. sys program은
계정 생성, 스토리지 할당 및 트랜잭션 처리와 같은 블록체인의 low-level 작업을 관리한다.
ShardPath는 stroage할당 및 액세스를 관리하는 sys 프로그램의 책임 중 일부이다.

##### Arc<Mutex<Shard>> get()
우리가 솔라나 cli를 키고 로그인을 한다고 가정해보자.
그렇다면 우리는 validator와 같은 노드가 아니라, client이며 네트워크 생태계에는 관여하지 않는 일시적인 노드이다.
즉 클라이언트 노드는 합의 프로세스에 관여하지 않으며 블록 생산 또는 검증에 참여하지 않는다. 대신
네트워크와 상호작용하여 트랜잭션 제출, 네트워크 상태 쿼리, 자신의 계정 관리와 같은 특정 작업을 수행할 수 있다.
클라이언트 노드는 validator에 의존하여 네트워크의 무결성과 보안을 유지한다.

##### Login process
1. 우리는 초기에 로그인을 할때, 저장된 키페어를 입력하고 로그인을 한다.
   이때 우리는 키페어에서 입력된 accountID를 가지고 get_shard_index메소드로 솔라나 네트워크에 쿼리를 날린다.

2. 솔라나 중앙 집중 네트워크는 쿼리를 받고 Consistent Hash ring 내부에서 accountID에 상응하는 shardIndex와 shardpath를 포함하는
   Shard 객체를 반환한다.

3. 반환된 Shard 객체는 우리의 클라이언트로 바로 전달되는 것이 아니라, 해당 샤드를 담당하는 validator에서
   Shard database에서 계정 데이터에 엑세스한다. 그 후에 특정 accountID를 찾아서 accountID와 pubkey가 일치하는지 검증한 후,
   특정 AccountID만 네트워크로 전송하고 네트워크에서 client로 전송해준다. 이것은 ShardPath 관리는 중앙 집중화하고,
   validator가 실행되기 전에 요청의 유효성을 확인할 수 있도록 하여 네트워크의 무결성과 보안을 보장하는 데 도움이 된다.
   또한 클라이언트에서 database에 직접 접근할 수단이 없게하여 데이터의 무결성을 보장하는데 도움을 준다.
- 여기서 validator가 shard에 접근할때, 로그인같은 데이터 변동성이 없는 작업이더라도 Arc<Mutex<Database>>에 readOption으로 접근한다.
  이론상 lock이 필요하지 않을 수 있지만 솔라나의 설계 철학은 읽기전용 작업을 포함한 모든 작업에서 스레드 안전성과 일관성을 우선시한다.
  즉 read에도 일관적으로 정확한 reading 철학을 고수한다.

- 여기서 발생하는 또다른 문제. shard의 평균 크기는 32gb로 매우 크다. 이렇게 큰 db를 검색할때마다 매번 lock을 거는 것은 매우 비효율적으로 보인다.
  때문에 솔라나는 여기서 Shard를 "accounts"라는 chunk로 나눈다. db를 shard로 나누듯이 역시나 여기서도 jump consistent hashing을
  사용해서 나눈다. 역시나 ShardPath struct 처럼 AccountPath struct가 따로 있고 sys program에서 담당한다.
  
4. AccountPath까지 고려한 validator의 접근방식
   validator는 먼저 ShardPath로 계정이 있는 shard를 찾는다. 그런 다음 ShardIndex를 사용하여
   속한 chunk를 계산(jumpConsistent Hashing으로 계산)하고 마지막으로 필요한 작업을 수행하기 위해 해당 청크만 잠근다.
   Accounts가 생성되는 시기는, shard가 생성됨과 동시에 그 shard를 새로운 account chunk들로 나눈다.
   즉, 솔라나 네트워크에 ShardPath 객체는 하나지만, AccountPath 객체는 각 샤드에 대해서 인덱싱 해야하기 때문에 샤드의 개수만큼 있다.
   그러므로 validator는 전체 샤드가 아닌 Accounts만 잠갔기 때문에 샤드의 나머지 부분에는 다른 노드들이 접근할 수 있다.

##### validator 정의
validator로 참여하기 위해서는 일정량의 SOL을 스테이킹하고 네트워크에 참여하기에 충분한 컴퓨팅 자원을 갖춘
validator node를 운영해야 한다. validator는 CPU, 메모리 및 스토리지 용량과 같이 네트워크에 참여하는 데
필요한 컴퓨팅 리소스를 제공할 책임이 있다.

1. validator는 솔라나 네트워크에 연결되고 합의 과정에 참여하는 솔라나 노드를 실행한다(여기서 말하는 솔라나 노드가 validator의 구현).
2. 솔라나 노드는 비동기로 쓰레드를 열고 AccountsPath, transaction from sender to recipient 요청을 기다린다.
3. 요청이 오면 단순 요청(login, balancing)인지, tx 요청인지 확인한다. AccountID받는다.
4. 어떤 경우이든 일단 솔라나 네트워크에 AccountID를 보내고 ShardPath or AccountPath를 요청하여 반환받는다.
5. 반환 받은 ShardPath로 Shard를 검색하여 들어가서 Arc<mutex>로 lock을 건다.
6. accountID와 pubkey가 일치하는지 확인한다.
7. 1) accountID와 pubkey가 일치할때 여기서 단순 요청(login, balancing)이면 네트워크(sys program이 받는다)에 accountID만 보낸다.
      실패하면 실패 메시지를 보냄
   2) accountID와 pubkey가 일치할때 여기서 tx 요청이라면 PoH, 서명확인, 간단한 PoW로 검증한다. 검증이 성공하면 validator가 서명하고
      accountID뿐만 아니라 블록 정보를 네트워크로 보낸다. 실패하면 실패 메시지를 보냄.
8. 블록 업데이트는 stake를 비롯한 tx가 있을 경우에만 수행하기 때문에, 업데이트가 완료되면 네트워크의 다른 노드에 블록정보를 propagates한다.
   이것은 노드가 네트워크의 다른 노드와 생성하거나 수신한 블록에 대한 정보를 공유하는 gossip으로 알려진 프로세스를 통해 발생한다.
   블록이 충분한 validator에게 전파되고 충분한 확인을 받으면 블록체인이 완성된것으로 간주되어 블록체인에 추가된다.
   validator는 네트워크의 다른 노드에 블록을 propagating할 책임이 있지만 네트워크 전체는 합의 프로세스를 조정하고
   모든 노드가 궁극적으로 블록체인 state에 동의하는지 확인하는 역할을 한다.
9. 블록 생성 및 유효성 검사에 대해 SOL 토큰으로 보상을 받는다.
10. 검증자로서 프로토콜 업그레이드 또는 네트워크 매개변수 변경 제안에 대한 투표와 같은 네트워크 거버넌스 결정에 참여할 수도 있다.

#### 3월 30일

##### ShardPath 및 database 컴파일

##### for loop에서 lazy하게 move하는 기술
for (account_id, path) in std::mem::take(&mut self.inner) {}
이렇게 하면 복제나 할당없이 self.inner의 원소를 하나씩 take하여 가져올 수 있다.

##### 추후에 ShardPath::get_shard() 메소드의 정해진 시간마다 rebalancing 메소드 수정하기
Consistent Hash ring의 구조에서 현재의 샤드와 path의 결정론적을 유지하면서 re-indexing하는 것은 의미가 없으며 불필요한 리소스 낭비다.
그러니 추후에 정해진 시간이 되면 load가 큰 path의 shard와 load가 적은 path의 shard를 교환하는 로직을 짜보자.

##### DBPool 생성 및 코드 수정

#### 3월 31일

##### DBPool::put_account

##### DB::put

##### DB::new, DB::put의 잠재적 취약점
DB::new에서 DB가 없으면 create_if_missing(true)가 default 설정이기 때문에,
샤드가 검색되지 않으면 자동생성함.
이것의 이점은 따로 초기화시키지 않아도 검색할때마다 필요에 의해 생성할 수 있지만,
경로가 잘못될 경우 새롭게 생성되는 문제가 발생할 수 있음.

이것의 해결은 metadata에 대한 조건을 걸어서 해당 디렉토리에 메타데이터 파일이 있으면,
생성시키고, 없으면 생성시키지 않는다.

##### DB::get 메서드에 unwrap 제거, 실패시 존재하지 않는 아이디 라는 메시지 반환시키기

##### Error handling database

##### client에 rate-limiter 구현해놓기
user의 요청 수를 추적하고 해당 요청이 일정 시간 내에 특정 임계값을 초과할 경우,
일시적으로 차단하거나 제한함으로써 악의적인 행위자가 대량으로 시스템을 압도하는 것을 방지

##### remove_inactive_database 수정
DBPool은 validator의 capacity가 한정되어 있기때문에, Pool에 db를 언제까지 추가할 수 만은 없다.
기존의 remove_inactive_database는 4시간동안 접근이 없으면 삭제하는 무차별적 삭제방식이었는데, 
이보다 더 정교한 기준이 필요할 듯 하다.

1. property_int_value("rocksdb.estimate-live-data-size") 메서드를 통해 live data 양을 추정하여 각 db의 사용패턴 계산.
2. 사용량 별로 db를 정렬하고 가장 적게 사용된 db를 정리하여 데이터베이스의 절반 이상을 유지.
3. 나머지 db는 마지막으로 엑세스한 이후 시간을 기준으로 가지치기

##### 추가 잠재적인 위험성 및 수정해야할 사항
1. error handling: panic하거나 문자열 오류 메시지를 반환하는 대신 결과 유형을 반환하여 오류 처리를 개선.
   예를 들어, get 함수는 Result<Option<Vec<u8>>, String> 대신 Result<Option<Vec<u8>>, Error>를 반환해야함.
   실패할 수 있는 다른 기능에도 동일하게 적용시키기

2. 동시 액세스: 여러 스레드가 동일한 데이터베이스에 액세스할 수 있으므로 경합 상태 또는 교착 상태의 위험이 있다.
   현재 구현에서는 'Mutex'를 사용하여 각 데이터베이스에 대한 액세스를 동기화하지만 동시성이 높은 시나리오에서는 비효율적일 수 있다.
   RwLock 또는 Crossbeam의 범위가 지정된 스레드와 같은 보다 효율적인 동기화 메커니즘을 사용하는 것이 좋다.

3. 메모리 관리: 현재 구현은 과도한 메모리 사용으로 이어질 수 있는 열린 데이터베이스의 수를 제한하지 않는다.
   열린 데이터베이스의 수를 제한하는 정책 구현을 고려하거나 연결 풀을 사용하여 데이터베이스 연결을 관리하기.
   - ~~한 번에 열 수 있는 데이터베이스 수에 대한 고정 제한 설정: 주어진 시간에 메모리에 보관할 수 있는 고정된 수의 열린 데이터베이스를 정의할 수 있다.
     한도에 도달하면 새 데이터베이스를 추가로 만들려고 하면 오류가 발생하거나 기존 데이터베이스가 닫힐 때까지 기다려야 한다.~~  
     -> _DBPool.max_dbs 필드 추가하여 해결_
   - ~~Use a connection pool: 연결 풀을 사용하여 고정된 수의 데이터베이스 연결을 관리할 수 있다.
     매번 새 데이터베이스를 여는 대신 풀에서 기존 연결을 검색할 수 있다. 데이터베이스 작업이 완료되면 연결이 다시 풀로 해제된다.
     이렇게 하면 열린 데이터베이스 연결 수를 제한하여 메모리 사용량을 줄일 수 있다.~~  
     -> _DBPool로 여러 DB를 Pool로 관리, DBPool.max_dbs 필드를 추가하여 임계값 설정 및, remove_inactive_database로 임계값 이하의 DB수를 유지함으로 해결_

4. 디스크 사용: remove_inactive_database 방법은 현재 사용 패턴 및 마지막 액세스 이후 시간에 따라 비활성 데이터베이스를 제거한다.
   그러나 이것은 특히 작은 데이터베이스가 많은 경우 과도한 디스크 사용을 방지하는 데 충분하지 않을 수 있다.
   디스크 사용량을 줄이기 위해 주기적으로 데이터베이스를 압축하거나 병합하는 정책을 구현하는 것이 좋다.

5. create_account 및 update의 오류 처리: 현재 구현에서는 데이터베이스에 쓰는 동안 오류가 발생하면 오류 메시지가 단순히 기록되고 함수가 반환된다.
   이로 인해 데이터 불일치 또는 기타 예기치 않은 동작이 발생할 수 있다.
   로깅 및 Ok(())를 반환하는 대신 오류 반환을 고려하기.

#### 4월 1일

##### 3월 31일 todo중 메모리 관리 두가지
3. 메모리 관리: 현재 구현은 과도한 메모리 사용으로 이어질 수 있는 열린 데이터베이스의 수를 제한하지 않는다.
   열린 데이터베이스의 수를 제한하는 정책 구현을 고려하거나 연결 풀을 사용하여 데이터베이스 연결을 관리하기.
   - ~~Use lazy loading: 한 번에 모든 데이터베이스를 로드하는 대신 지연 로딩을 사용하여 필요할 때만 데이터베이스를 로드할 수 있다.
     현재 열려 있지 않은 데이터베이스에 대한 요청이 들어오면 데이터베이스를 열고 요청을 처리한 다음 데이터베이스를 닫을 수 있다.
     이렇게 하면 필요할 때만 데이터베이스를 메모리에 보관하여 메모리 사용량을 줄일 수 있다.~~
   -> _DBHandler 구현._
   ~~- Close idle databases: 일정 시간 동안 사용하지 않은 유휴 데이터베이스를 주기적으로 확인하고 닫아 메모리를 확보할 수 있다.
     이는 각 데이터베이스의 마지막 액세스 시간을 확인하고 한동안 액세스하지 않은 데이터베이스를 닫는 백그라운드 프로세스를 사용하여 수행할 수 있다.~~
   -> remove_inactive_database 메서드 내부에 4시간 룰 적용.

##### DB::update 메서드 수정하기
기존 데이터에 값이 존재할 경우, 새로 옮겨진 데이터 또는 변하는 데이터를 각각의 필드를 찾아 합산하여 저장해줘야함.
hash값을 encoding, decoding만으로는 필드에 따른 값을 정밀하게 찾을 수 없다.

##### DBHandler 구현
lazy loading으로 필요할 때만 DB를 로딩하도록 DBpool을 필드로 갖는 Handler 구현.

#### 4월 2일
동시 액세스: 여러 스레드가 동일한 데이터베이스에 액세스할 수 있으므로 경합 상태 또는 교착 상태의 위험이 있다.
현재 구현에서는 'Mutex'를 사용하여 각 데이터베이스에 대한 액세스를 동기화하지만 동시성이 높은 시나리오에서는 비효율적일 수 있다.
RwLock 또는 Crossbeam의 범위가 지정된 스레드와 같은 보다 효율적인 동기화 메커니즘을 사용하는 것이 좋다.

범위가 지정된 스레드에서 효율적인 동기화 메커니즘을 사용하기 위한 대한 초석을 다지기 위해 동기화 프리미티브 정리

##### Validator::login, signup, update 구현

##### Validator에 RateLimiter 필드 추가하여, 과다 요청 방지

#### 4월 3일
Atomic types & Ordering guarantees  
https://github.com/datactor/Rustic-data-solving/blob/main/forge/rust_concepts_explained/advanced_sync_primitives1.md

#### 4월 4일
Atomic types & Ordering guarantees  

#### 4월 5일
Atomic types & Ordering guarantees  

#### 4월 6일
Atomic types & Ordering guarantees  

#### 4월 7일
Atomic types & Ordering guarantees  

spin-lock을 사용해서 Condvar를 기다리는 Barrier의 대체재:
tree Barrier & tournament barrier.
tree barrier는 이름에서 알 수 있듯이 참여 스레드 간에 트리 구조를 형성하고 각 스레드는 진행하기 전에 부모와 자식이 barrier에 도착하기를 기다린다.
이는 spin barrier에 비해 더 나은 캐시 활용과 경합 감소로 이어질 수 있다.  
(merkle tree 해싱 연산을 구현할 때 다중 스레드에 분산 연산으로 분배하고, tree-barrier를 고려해보자)

#### 4월 8일
Atomic types & Ordering guarantees

##### update semaphore

##### deadlock을 방지하는 해결책 중 상황에 맞는 가장 효율적인 방법 고려하기.
lock ordering으로 순서를 직접 정하는 방식은 programmer에게 전적으로 책임이 전가된다.
1. lock ordering + barrier 동기로 각 프로세스에 대해서 여러 리소스에 대한 의존성을 줄이고 기다리게 하는 방식
2. lock ordering + consistent Ring
3. lock ordering + timeout mechanism
4. lock ordering + std::sync::DeadlockDetection으로 deadlock 감시
5. lock ordering + Rwlock
각 사용 사례의 특정 요구 사항을 고려하고 가장 적절한 primitives 선택하기

#### 4월 9일
Atomic types & Ordering guarantees

##### update Mutex

##### Once
개념정리, 분산처리에서 Once의 용도?
여러 스레드에서도 정확히 한번 수행하는 Once의 특성을 이용해서 초기에 Expensive initialization 과정을 Once로 진행.  
또는 exactly once 작업의 수행 구현을 Once의 사용을 고려.
그러나 Once가 실패시, 더이상 그 작업에 대해서 시도하지 않기 때문에, 실패할 경우를 항상 염두에 두고 있어야 한다.

##### RwLock
다중 스레드에서 공유데이터에 대한 read/write 옵션을 따로 사용하여 구현할 때 사용.
write에 대해서만 상호 배타적 독점 lock을 걸고, read에 대해서는 다중 스레드에서 동시 접근이 가능한 lock을 건다.
이는 Rust의 borrow rule과 비슷한 논리로 작동한다.

RwLock은 spin-lock과 CAS, atomic primitives로 구현되었으며 spin-lock의 단점을 futex의 활성/비활성 전환 기능을 활용하여 상쇄한다.
또한 내부의 `wake_writer_or_readers()` 메서드로 대기 중인 reader와 writer를 특정 순서로 깨워
ordering을 부여해, deadlock을 방지하고, read lock에 대해서는 다중 스레드의 접근을 허용하여 경합을 없애 오버헤드를 줄였다.
또한 writer에게는 상호 베타적 독점 lock이지만, 우선순위를 지정하여 공유 데이터에 엑세스할 공정한 기회를 갖도록 보장한다.

#### 4월 19일
솔라나의 초기 수행은 단일 스레드에서 수행한다.
즉, sys 프로그램, mint 프로그램, token 프로그램의 init에 시퀀스를 부여하여,
솔라나의 상태를 결정론적으로 유지하기 위함이다. 그 이후에 app 내부의 각각의 기능들은 비동기로 병렬로 처리한다.

#### 4월 20일

app 모듈에서 app의 run 메서드로 sys와 token, mint 프로그램을 실행한다.
이것은 단일 스레드에서 실행시켜 동기적으로 시퀀스 순서를 지켜 프로그램의 결정론적을 유지하기 위함.

그리고 sys 프로그램을 실행시킨다면

1. 시스템 프로그램 계정 생성 및 관리: sys 프로그램은 블록체인의 핵심 기능을 담당하므로, 시스템 프로그램 계정을 생성하고 관리합니다. 시스템 프로그램 계정은 블록체인의 핵심 기능을 구현하는 데 필요한 자원을 확보하는 데 사용됩니다.
- 그렇다면 시스템 계정 생성은 시스템의 계정은 최초 1회에만 생성하면 되니, 1회에만 생성하고 이후에는 여러가지 방법으로 재시도를 하고 실패시 패닉 또는 에러처리로 프로그램의 실행 흐름을 유지.
2. 블록 생성 및 추가: sys 프로그램은 새로운 블록을 생성하고 블록체인에 추가합니다. 이를 위해 Sys 구조체를 사용하여 블록체인의 현재 상태를 유지하고, 새로운 블록을 생성하기 위한 데이터를 관리합니다.
- 체인 생성 및 추가는 먼저 체인이 있는지 확인하고, 없다면 여러가지 방법으로 재시도를 해보고 패닉을 일으키거나 최초 1회에 한해서 제네시스를 시키면되겠고,
  블록의 생성 및 추가는 비동기로 이벤트 루프에 감시 기능을 구현하여 여러 합의를 통해 들어온 블록을 감시하면 될것같아.
3. 계정 생성 및 관리: sys 프로그램은 새로운 프로그램 계정을 생성하고, 기존의 계정을 관리합니다. 이를 위해 ProgramAccount 구조체를 사용하여 프로그램 계정의 정보를 유지하고, 계정에 대한 접근 권한을 관리합니다.
- 계정의 생성 및 관리는 1번과 마찬가지로 다른 계정에 대해서 같은 동작을 하면 될 거 같고,
4. 트랜잭션 처리: sys 프로그램은 블록체인에 새로운 트랜잭션을 추가하고, 이전 트랜잭션들을 처리합니다. 이를 위해 Transaction 구조체를 사용하여 트랜잭션 정보를 유지하고, 트랜잭션에 대한 검증과 실행을 수행합니다.
- tx처리도 2번의 블록 생성 및 추가 처럼 비동기로 이벤트 루프에 브로드캐스트 및 PoS방식으로 validator에게 블록의 분배, 또는 전파를 수행하면 될거 같고,
5. 시스템 리소스 관리: sys 프로그램은 블록체인의 시스템 리소스를 관리합니다. 이를 위해 Sys 구조체를 사용하여 시스템 리소스의 상태를 유지하고, 리소스의 할당과 해제를 처리합니다.
- 시스템 리소스 관리는 ShardIndex 및 Consistent hashring로 인덱싱 관련 제어로 밸리데이터의 시스템 리소스의 관리에 도움을 준다는 뜻인가?
  예를 들어서 전체의 블록체인을 할당하는 일은 없고, 공유 리소스에 해당하는 특정 체인의 일부(샤드)를 블록에 추가하고 해당 슬롯으로 인덱싱하고 해당 공유 메모리에 락을 해제하는 동작을 말하는건가?
6. 기타 시스템 관리: sys 프로그램은 블록체인 시스템의 다양한 관리 작업을 수행합니다. 예를 들어, 블록체인의 성능을 모니터링하고, 이에 대한 통계 정보를 수집하고, 블록체인의 보안성을 유지하기 위한 감사 로그를 생성합니다.

sys 프로그램이나 token, mint 프로그램을 실행하는 관리자 노드의 로컬에 프로그램 아이디 또는 블록체인의 인덱스 정보등의 메타데이터들이 저장된다.
이것은 공유 메모리가 아니라 프라이빗 정보이다. 그러므로 validator에서는 접근이 불가능하다. 그렇지만 프로그램을 관리하는 노드도 여러개가 있으며,
여기서도 BFT의 원리에 따라 Tower bft를 수행하여 리더노드를 선출하고 fault tolerance를 부여하기 때문에
프라이빗 로컬에 저장된 정보도 Arc<RwLock>으로 구현하여 데이터 일관성을 보장하고 데이터 레이싱을 방지해야한다.

#### 5월 1일
consistent hashing은 리밸런싱 될때, node가 실패하거나 사용할 수 없을 때(이 경우 이전에 해당 노드에서 제공한 키를 시스템의 다른 노드로 마이그레이션
해야하므로 마이그레이션이 완료될 때까지 일시적인 불일치가 발생할 수 있음) 일시적인 불일치가 발생할 수 있다.
이러한 weak consistency는 여러개의 replicas로 인해 strong consistency를 달성할 수 있다. 
그렇지만 이는  quorum-based replication scheme를 사용하는 것으로, 이 접근 방식은 모든 복제본이 동기화된 상태로 유지되고
모든 노드가 동일한 데이터 사본을 볼 수 있도록 하는 것이다.
이 접근 방식에서 각 write operation은 commit된 것으로 간주되기 전에 quorum of replicas에서 ack된다.
예를 들어 각 데이터 조각에 대해 3개의 replicas가 있는 경우 commit된 것으로 간주되기 전에 적어도
2개의 복제본이 write operation을 ack해야 한다. 이렇게 하면 모든 복제본에 동일한 데이터 복사본이 있고 모든 replicas가 write operation 순서에
동의하는지 확인할 수 있다. 그렇지만 이는 시스템의 replicas 또는 node 수가 증가함에 따라 복잡해진다. 여기서 consensus 알고리즘이 빛을 발한다.
Paxos 또는 Raft와 같은 합의 알고리즘은 오류나 네트워크 파티션이 있는 경우에도 모든 복제본이 데이터의 현재 상태에 동의하는지 확인하는 더 간단한 방법을 제공한다.

아래는 Raft의 예다.

1. 시스템의 각 노드에는 indentifier 또는 IP주소를 기반으로 consistent hash ring의 위치가 할당된다. 
2. 각 키에는 해시 값을 기반으로 consistent hash ring의 위치도 할당된다.
3. client가 키를 읽거나 쓰려고 할 때 먼저 hash ring에서 키의 위치를 계산하고 해당 키에 대한 요청을 처리할 책임이 있는 노드를 결정한다.
4. 그런 다음 client는 적절한 노드에 read/write 요청을 보낸다(일반적으로 leader node).
5. 노드가 write 요청을 받으면 Raft를 사용하여 시스템의 다른 모든 노드에 write operation을 복제한다. Raft는 모든 노드가 write 작업
   순서에 동의하고 모든 노드가 동일한 데이터 복사본을 갖도록 한다.
6. 노드가 read 요청을 받으면 데이터의 local 복사본에서 데이터를 읽는다. 모든 노드가 동일한 데이터 복사본을 가지고 있기 때문에 모두 동일한 키에
   대해 동일한 값을 반환한다.

그렇지만 솔라나에서는 PoH(network)와 Tower BFT(validator)를 사용하여,
weak consistency인 jump consistent hashing에서 strong consistency를 달성하게 한다.
이것을 위해 네트워크의 모든 노드가 블록체인의 현재 상태에 동의하는지 확인하는 역할을 하는 validator와
PoH consensus를 포함한 기술 조합을 사용한다.

PoH는 네트워크의 노드 간 low-latency communication에 최적화되어 있으며,
합의를 달성하기 위한 빠르고 확장 가능한 메커니즘을 제공한다.
PoH는 블록체인의 각 블록에 대해 검증 가능한 timestamp(slot)를 생성하여 노드가 체인의 트랜잭션 및 블록 순서를 빠르고 쉽게 확인할 수 있도록 한다.

솔라나의 validator는 Tower BFT를 사용해 체인의 트랜잭션 및 블록 순서에 대한 합의를 달성하며,
네트워크 전체에서 PoH를 사용해 블록체인의 시간 참조를 생성하여 합의를 달성한다.

#### 5월 10일
실행 시퀀스

1. bootstrap.rs
   bootstrap은 비동기로 요청을 보내고 받을 수 있는 서버를 열고,
   클러스터의 네트워크 구성정보(보통 JSON 파일)을 읽어서 클러스터의 구성 정보,
   노드의 주소, 포트, 키, 샤드, 리더 노드 등의 정보를 가져와서 해당 노드에 요청을 보낸다.

그리고 해당 노드들이 실행되면 bootstrap의 서버로 응답을 보내
모든 요청에 대해 200(정상)응답을 받으면, bootstrap은 종료된다.

2. app.rs
   bootstrap이 성공적으로 역할을 하고 종료되면 app.rs에서 프로그램을 실행한다.
   app.rs에서는 key를 확인하고 해당 key에 해당하는 프로그램을 실행시킨다.

Todo!

bootstrap.rs에서 cluster 및 노드 정보를 json에서 읽고 peer struct를 구현해서 peer에는 json에서 읽어온 노드정보를 기반으로 peer를 만들어서 요청을 보내고 응답을 받을 수 있는 기능 구현하기

#### 5월 12일
Bootstrap

trusted node인 관리자 노드들을 부트스트래핑하고 유지시킬때는 raft를 사용하며,
이 노드들과 validator들을 연동시킬때는 pbft와 pos를 사용하는 것을 고려.
PBFT는 비잔틴 장애에 대한 내성을 제공하며, 모든 노드가 각 트랜잭션에 대해 동일한 순서로
합의를 이뤄야한다. 이 과정은 더 많은 메시지 교환을 요구하며, 더 큰 네트워크 오버헤드와
latency를 가지게 할 수 있다. 따라서 일반적으로 PBFT가 더 리소스 집약적이라고 할 수 있다.

Raft는 중앙 집중식 노드(리더)가 결정을 조정하고 나머지 follower들은 결정을 수용한다.
이 방식을 통해 raft는 합의에 도달하는 데 필요한 메시지의 수와 latency를 최소화 시킨다.
이 방법을 사용하려면
trusted노드들중 필요한 만큼만 raft 프로토콜을 유지하고,
나머지 노드들은 redundancy로 대기 상태에 들어가는 방식으로 구현하더라도,
raft 프로토콜을 유지하고 있는 노드들은 여전히 PBFT와 PoS를 Raft를 유지하는
중에도 사용해야한다. raft를 유지하는 노드들은 연결되는 Validator를 최소화 시켜야 하며,

그렇기 위해서는 trusted node가 충분한 리소스가 있어서 연동된 validator와
trusted node의 개수를 그 리소스 내로 제한한다면, 각기 다른 코어 또는 스레드로
작동할 것이기 때문에 성능드랍 이슈가 최소화 될 것이고, 이것은 validator를 유지함과 동시에
일관되고 빠른 부트스트래핑 프로세스 성능을 보장할 수 있다.

여기서 trusted 노드에 대한 보장도 있어야한다.
bootstrap 프로세스는 이것만을 위해 pbft를 사용하기에는 적절하지 않아보인다.
중앙 집중식 결정이기 때문에, 하나의 결정을 위해 여러 노드가 합의를 이루지 않기 때문이다.
그러므로 여러 노드들의 결정들간에 66%이상의 결정을 다수결로 채택하는 pbft는
좋은 선택이 아닌것 같다.
해결책:
1. 고립된 환경: 서버를 제외하고는 인터넷 환경을 고립시킨다.
2. 프로그램 실행의 무결성: 범용적인 무결성 검증, 실행 결과의 해시값 비교 등은
   머클 트리 연산으로 수행한다.
3. 안정성과 가용성: 고립된 환경에서 노드 실패는 치명적이지만 이부분에 대해서는
   replicas를 포함한 로드 밸런싱으로 해결한다.
4. 프로그램 업데이트: 초기 부트스트래핑 노드와는 연결이력이 있으며 이 노드와는
   업데이트를 받는 노드는 새로운 코드가 신뢰할 수 있는 출처에서 왔음을 확인하고,
   코드가 변조되지 않았음을 검증한다. 이러한 점을 보장하기 위해,
   머클 트리와 같은 무결성 검증 메커니즘을 사용한다.