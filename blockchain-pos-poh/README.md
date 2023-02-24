# BLOCKCHAIN - PoS & PoH(SOL)

## Configuration
### 1. Nodes
These configuration files are used to specify parameters such as network address, port numbers, performance settings, and security settings.
#### Full node
Todo!();
The full node configuration file is used to configure the behavior of a full node in the Solana network. It includes parameters such as the network address, port numbers, and file paths for the node's data and log files. It also includes settings for the node's performance and security, such as the number of threads to use for transaction processing and the amount of memory to allocate for the node's database.

#### Validator
Todo!();
The validator configuration file is used to configure a validator node in the Solana network. It includes parameters such as the node's validator identity and staking information, as well as the network address and port numbers for the node. It also includes settings for the node's performance and security, such as the number of threads to use for block production and the maximum transaction rate the node can handle.

#### Archiver
Todo!();
The archiver configuration file is used to configure an archiver node in the Solana network. It includes parameters such as the node's network address and port numbers, as well as the file paths for the node's data and log files. It also includes settings for the node's performance and security, such as the maximum amount of disk space to use for storing historical blockchain data.

#### Edge
Todo!();
The edge configuration file is used to configure an edge node in the Solana network. It includes parameters such as the node's network address and port numbers, as well as settings for the node's performance and security, such as the maximum number of requests the node can handle and the maximum amount of memory to use for request processing.

#### bootstrapper(optional)
Todo!();
This is a type of node that helps new nodes join the Solana network by providing them with information about the existing nodes in the network.

### 2. DB
This category includes the components and files related to the storage and management of blockchain data.
These components and files help ensure the accuracy and consistency of the blockchain data and improve the availability and reliability of the network.

#### Snapshot Archives
Todo!();
A snapshot archive is a compressed file that contains a copy of the Solana blockchain data at a specific point in time. Snapshot archives are used to help new nodes quickly synchronize with the network, by providing them with a pre-built copy of the blockchain data that they can use to bootstrap their local copy. Snapshot archives can be created manually or automatically by a validator node using the Solana CLI.

#### Accounts DB
Todo!();
The Accounts DB is a database that stores the state of all accounts on the Solana blockchain. It is used by full nodes and validators to validate transactions and produce new blocks. The Accounts DB can be configured to use different storage backends, such as local disk storage or cloud-based storage services like Amazon S3.

#### Vote Account
Todo!();
A vote account is a special type of account that is used by validator nodes to participate in the consensus process and produce new blocks. Vote accounts are associated with a specific validator node, and are used to store the node's staking information and vote tokens. Validators must stake a certain amount of SOL tokens to participate in the consensus process, and the amount of stake determines the node's voting power and ability to produce new blocks.

#### Replicator
Todo!();
A replicator is a type of node that helps distribute and replicate the Accounts DB across the Solana network. Replicators store a copy of the Accounts DB and use a gossip network to exchange updates with other replicators and nodes in the network. Replicators help improve the availability and reliability of the Accounts DB, and can help reduce the time and bandwidth required to synchronize new nodes with the network.

### 3. CLI
The CLI is a tool that allows developers and node operators to interact with the Solana network and perform various operations, such as creating and deploying programs, querying the blockchain, and managing wallet accounts. The CLI configuration file is used to specify parameters such as default network address and port numbers, as well as developer-specific settings.

#### CLI
Todo!();
The CLI configuration file is used to configure the Solana command-line interface (CLI). It includes parameters such as the default network address and port numbers, as well as settings for the CLI's performance and security, such as the default transaction fee and the maximum transaction rate the CLI can handle.


## Programs
### sys program
Todo!();
시스템 내부에 account가 있으며, 블록체인 업데이트에 사용된다.
solana의 blockchain update를 직접적으로 할 수 있는 수단은 오직 sys program이다.
시스템 내에 account가 있지만 user account를 생성하거나 관리하는 것이 아니라 blockchain을
업데이트하는데 있어서는 account가 필요하지 않다.
그럼에도 genesis에 account가 생성된 이유는 초기의 sys program state를 나타내기 때문이다.
blockchain이 시작되면 sys program이 메모리에 로드되고 state는 해당 account의 데이터로 표시된다.
이 account는 blockchain을 부트스트랩하고 sys program이 제대로 로드되고 blockchain을 업데이트할
준비가 되었는지 확인하는데 필요하다.

### mint program
Todo!();
시스템 내부에

### token program
Todo!();

### stake program
Todo!();

### BPF(Berkeley Packet Filter)
Todo!();
솔라나 위에 Dapp을 구축할 수 있게 만들어주는 핵심 구성 요소.
Rust, C, AssemblyScript를 비롯한 프로그램으로 smart contract를 작성하고 배포할 수 있는 경량 가상 머신.
#### Serum program
Todo!();
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
Todo!();

#### Smart contract language: Rust
#### Token standard
Todo!();
The token standard that will be used for creating and managing different types of tokens on your blockchain.
#### Node software
Todo!();
The software that can run different types of nodes on your blockchain, such as full nodes, validators, and edge nodes.
#### Wallet software
Todo!();
The software that can manage private keys and interact with your blockchain.
#### Development tools
Todo!();
The tools and libraries that can be used for developing and deploying smart contracts, interacting with the blockchain, and testing the network.