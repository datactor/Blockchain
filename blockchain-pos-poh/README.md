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
genesis에서 account 생성, 새로운 Sol token을 생성하는데 사용되며 Mint program에서 관리한다.
mint의 잔액은 총 sol token수와 직접 관련된다. 모든 solana token은 Mint account를 거쳐서 가기 때문이다.
solana가 처음 발행되면 mint account로 입금된다. Validator가 block보상을 받을 때 역시 mint로부터 입금되고,
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
Todo!();
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