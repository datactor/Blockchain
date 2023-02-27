use super::*;
use std::{
    io::{
        self, prelude::*,
    },
    env,
    error::Error,
};
use std::any::Any;
use std::num::ParseIntError;
use bs58::{decode, encode};
use ring::signature::{Ed25519KeyPair, KeyPair};

pub fn interpreter(mut blockchain: Blockchain, mut accountset: AccountSet) {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Handle command line arguments if any
        // ...
        println!("Arguments will be implemented later. Please run it without entering any arguments.")
    } else {
        // Start REPL
        println!("\n\
                  ================================================== =\n\
                  Welcome to RustyVault Interface!\n\
                  ================================================== =");

        println!("\n\
                  1: log in\n\
                  2: create a new wallet\n\
                  0: quit this program\n\
                  ");
        loop {
            let mut input = String::new();
            print!(">>> ");
            io::stdout().flush().expect("Failed to flush stdout"); // 명령 메시지에  >>> flush
            io::stdin().read_line(&mut input).expect("Failed to read line");

            // let login_cmd = input.trim().parse::<usize>().expect("Please input valid number");
            let mut login_cmd;
            if let Ok(n) = input.trim().parse::<usize>() {
                login_cmd = n;
            } else {
                println!("Please input valid number");
                continue
            };


            match login_cmd {
                0 => {
                    break
                }
                1 => {
                    println!("log in");
                    input.clear();
                    println!("\n\
                              Please input your private key.\n\
                              or Enter 0 if you want to go back");
                    print!(">>> ");
                    io::stdout().flush().expect("Failed to flush stdout");
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    let input_keypair = input.trim().replace(" ", "");
                    let input_keypair = input_keypair.strip_prefix("[").unwrap_or(&*input_keypair);
                    let input_keypair = input_keypair.strip_suffix("]").unwrap_or(input_keypair);

                    let keypair_bytes_result: Result<Vec<u8>, ParseIntError> = input_keypair
                        .split(',')
                        .map(|s| s.trim().parse())
                        .collect::<Result<Vec<u8>, ParseIntError>>();

                    let keypair_vec = match keypair_bytes_result {
                        Ok(bytes) => {
                            if bytes.len() != 64 {
                                println!("Error: Invalid keypair length");
                                return;
                            }
                            bytes
                        },
                        Err(err) => {
                            println!("Error: {}", err);
                            return;
                        }
                    };
                    println!("{:?}", keypair_vec);
                    let keypair_array: [u8; 32] = (32..64).map(|i| keypair_vec[i]).collect::<Vec<u8>>().try_into().unwrap();
                    if let Some(pubkey) = accountset.get_account(&Pubkey(keypair_array)) {
                        println!("log in success");
                    } else {
                        println!("This keypair does not exist.")
                    }


                },
                2 => {
                    println!("sign up")
                },
                // "balance" => {
                //     // Handle balance inquiry logic
                //     // ...
                // },
                // "send" => {
                //     // Handle send transaction logic
                //     // ...
                // },
                _ => {
                    println!("Unknown command: {}", login_cmd);
                },
            }
        }
    }
}