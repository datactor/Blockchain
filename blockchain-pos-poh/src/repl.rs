use super::*;
use std::{
    io::{
        self, prelude::*,
    },
    env,
    // error::Error,
};
use std::any::Any;
use std::num::ParseIntError;
use bs58::{decode, encode};
use ring::signature::{Ed25519KeyPair, KeyPair};
use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Error {
    ParseError,
    KeypairError,
}

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

        loop {
            // Parse input (0..=2)
            println!("\n\
                  1: log in\n\
                  2: create a new wallet\n\
                  0: quit this program\n\
                  ");

            let mut first_cmd = input::<usize>();
            match first_cmd {
                Ok(_) => {},
                Err(_) => {
                    println!("Please enter a valid value");
                    continue
                }
            }

            match first_cmd.unwrap() {
                1 => println!("log in"),
                2 => {
                    println!("create new wallet");
                    create_new_wallet(accountset.clone())
                },
                0 => {
                    println!("quit");
                    break
                },
                _ => {
                    println!("Please enter a valid number (0 - 2)");
                    continue
                }
            }
        }
    }
}

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: &str) -> Self {
        ParseError {
            message: message.to_string(),
        }
    }
}

fn create_new_wallet(mut accountset: AccountSet) {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(1_000_000_000..10_000_000_000);

    let mut is_human = false;

    for _ in 0..5 {
        println!("Type this number: {}", rand_num);
        let input = input::<usize>();
        match input {
            Ok(n) => {
                if n == rand_num {
                    is_human = true;
                    println!("Identified");
                    break
                } else {
                    println!("Wrong number. try again");
                    continue
                }
            },
            Err(_) => {
                println!("Please enter a valid value");
                continue
            }
        }
    }

    if !is_human {
        println!("Not identified. Return to previous procedure.");
        return
    }

    println!("hello human");


    let new_private = Privatekey::new();

    let new_account = Account::new(0, new_private.pubkey(), 0, vec![], false, Some(new_private.sign(&[0u8; 32])));

    for _ in 0..5 {
        println!("Would you like to register this wallet?\n\
         y/n?");
        let input = input::<String>();
        match input.clone() {
            Ok(n) => {
                if n == "y" {
                    accountset.insert_account(new_private.pubkey(), new_account);
                    println!("The wallet has been successfully registered");
                    break
                } else if n == "n" {
                    println!("Creation cancelled. Return to the previous menu.");
                    break
                } else {

                }
            },
            Err(_) => {
                println!("Please enter a valid value");
                continue
            }
        }
    }




}

fn start(mut blockchain: Blockchain, mut accountset: AccountSet) {
    println!("\n\
                  1: log in\n\
                  2: create a new wallet\n\
                  0: quit this program\n\
                  ");

    let mut input = input::<usize>();

    let mut login_cmd;
    if let Ok(n) = input {
        login_cmd = n;
    } else {
        println!("Please input valid number");
        return
    };
    // let mut login_cmd;
    // if let Ok(n) = input.trim().parse::<usize>() {
    //     login_cmd = n;
    // } else {
    //     println!("Please input valid number");
    //     return
    // };
    match login_cmd {
        0 => {
            return
        }
        1 => {
            println!("log in");
            let mut input = String::new();
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
            println!("sign up");
            // println!("log in");
            // input.clear();
            // println!("\n\
            //                  Please input your private key.\n\
            //                  or Enter 0 if you want to go back");
            // print!(">>> ");
            // io::stdout().flush().expect("Failed to flush stdout");
            // io::stdin().read_line(&mut input).expect("Failed to read line");
            // let input_keypair = input.trim().replace(" ", "");
            // let input_keypair = input_keypair.strip_prefix("[").unwrap_or(&*input_keypair);
            // let input_keypair = input_keypair.strip_suffix("]").unwrap_or(input_keypair);
            // let keypair_bytes_result: Result<Vec<u8>, ParseIntError> = input_keypair
            //     .split(',')
            //     .map(|s| s.trim().parse())
            //     .collect::<Result<Vec<u8>, ParseIntError>>();
            // let keypair_vec = match keypair_bytes_result {
            //     Ok(bytes) => {
            //         if bytes.len() != 64 {
            //             println!("Error: Invalid keypair length");
            //             return;
            //         }
            //         bytes
            //     },
            //     Err(err) => {
            //         println!("Error: {}", err);
            //         return;
            //     }
            // };
            // println!("{:?}", keypair_vec);
            // let keypair_array: [u8; 32] = (32..64).map(|i| keypair_vec[i]).collect::<Vec<u8>>().try_into().unwrap();
            // if let Some(pubkey) = accountset.get_account(&Pubkey(keypair_array)) {
            //     println!("log in success");
            // } else {
            //     println!("This keypair does not exist.")
            // }
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


fn input<T: std::str::FromStr>() -> Result<T, Error> {
    let mut input = String::new();
    print!(">>> ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let trimmed_input = input.trim();
    match trimmed_input.parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::ParseError),
    }
}