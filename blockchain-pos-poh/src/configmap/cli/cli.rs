// 1. Login interaction or sign in (create wallet)
// 2. If there is a wallet, enter the key and interact with the account DB to bring information such as balance. If there is no wallet, go to step 3
// 3. If there is no wallet, create a wallet by issuing a private key and updating the account db.
//
// cli such as stake will be implemented later
// 1. If run tx, it does not need to reopen the account db because the balance loaded from the account db will be loaded into the memory of the cli.
// 2. If the recipient's address, balance, and private key are entered, some option is provided to user (speed according to fee, related meta)
// 3. it is checked whether they match the signature retrieved when logging in, and at the same time, whether the balance is satisfied.
// 4. If the above process is confirmed, send a tx to the network for a request.

// use clap::{App, Arg, SubCommand};
//
// pub fn cli() {
//     let matches = App::new("Solana CLI")
//         .version("0.1.0")
//         .author("Your Name <you@example.com>")
//         .about("A command line interface for Solana")
//         .subcommand(
//             SubCommand::with_name("login")
//                 .about("Log in to your Solana wallet")
//                 // Add arguments for login subcommand
//                 .arg(
//                     Arg::with_name("username")
//                         .help("Your username for Solana")
//                         .required(true)
//                         .index(1),
//                 )
//                 .arg(
//                     Arg::with_name("password")
//                         .help("Your password for Solana")
//                         .required(true)
//                         .index(2),
//                 ),
//         )
//         .subcommand(
//             SubCommand::with_name("create-wallet")
//                 .about("Create a new Solana wallet")
//                 // Add arguments for create-wallet subcommand
//                 .arg(
//                     Arg::with_name("username")
//                         .help("Your desired username for Solana")
//                         .required(true)
//                         .index(1),
//                 )
//                 .arg(
//                     Arg::with_name("password")
//                         .help("Your desired password for Solana")
//                         .required(true)
//                         .index(2),
//                 ),
//         )
//         .subcommand(
//             SubCommand::with_name("tx")
//                 .about("Send a transaction")
//                 // Add arguments for tx subcommand
//                 .arg(
//                     Arg::with_name("recipient")
//                         .help("The address of the recipient")
//                         .required(true),
//                 )
//                 .arg(
//                     Arg::with_name("amount")
//                         .help("The amount to send in SOL")
//                         .required(true),
//                 ),
//         )
//         .get_matches();
//
//     match matches.subcommand() {
//         ("login", Some(sub_m)) => {
//             // Logic for the login subcommand
//         }
//         ("create-wallet", Some(sub_m)) => {
//             // Logic for the create-wallet subcommand
//         }
//         ("tx", Some(sub_m)) => {
//             // Logic for the tx subcommand
//         }
//         _ => {
//             // No subcommand was used, so print help message
//             println!("{}", matches.usage());
//         }
//     }
// }
