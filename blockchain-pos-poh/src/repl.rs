use super::*;
use std::{
    io::{self, prelude::*, },
    env,
    fmt::Debug,
    str::FromStr,

};
use bs58::{decode, encode};
use ring::signature::{Ed25519KeyPair, KeyPair};
use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Error {
    ParseError,
    ParseIntError,
    KeypairError,
    InvalidConversionError,
    HumanIdentificationError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError => write!(f, "Error parsing input"),
            Error::ParseIntError => write!(f, "Error parsing int input"),
            Error::KeypairError => write!(f, "Error creating keypair"),
            Error::InvalidConversionError => write!(f, "Error converting data to a different type"),
            Error::HumanIdentificationError => write!(f, "Error checking robot"),
        }
    }
}

// impl std::error::Error for Error {}

pub fn login_menu_main(mut accountset: AccountSet) {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Handle command line arguments if any
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

            match input::<usize>() {
                Ok(n) => match n {
                    0 => break, // exit program
                    1 => {
                        println!("log in\n");
                        login(accountset.clone()).expect("Failed to login");
                        continue
                    },
                    2 => {
                        println!("create new wallet\n");
                        create_new_wallet(accountset.clone()).expect("Failed to create wallet");
                    },
                    _ => {
                        println!("Please enter a valid number (0 - 2)\n");
                        continue
                    },
                },
                Err(_) => continue
            }
        }
    }
}

fn create_new_wallet(mut accountset: AccountSet) -> Result<(), Error> {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range(100_000_000_000..1_000_000_000_000);

    let mut is_human: Option<bool> = None;

    for i in 0..5 {
        println!("Type this 12 digit number: {} ({}/5)", rand_num, 5-i);
        match input::<usize>() {
            Ok(n) => {
                if n == rand_num {
                    is_human = Some(true);
                    println!("Identified");
                    break
                } else {
                    println!("Wrong number. try again");
                    continue
                }
            },
            Err(_) => continue
        }
    }

    if is_human.is_none() || is_human.unwrap() == false {
        println!("Identification failed. Returning to main menu.");
        return Err(Error::HumanIdentificationError)
    }

    let new_private = Privatekey::new();

    let new_account = Account::new(0, new_private.pubkey(), 0, vec![], false, Some(new_private.sign(&[0u8; 32])));

    for i in 0..5 {
        println!("({}/5) Would you like to register this wallet? (y/n)", 5-i);
        let input = input::<String>()?;
        match input.as_ref() {
            "y" => {
                accountset.insert_account(new_private.pubkey(), new_account);
                println!("The wallet has been successfully registered");
                return Ok(())
            },
            "n" => {
                println!("Creation canceled. Return to the previous menu.");
                return Ok(())
            },
            _ => continue
        }
    }

    Err(Error::InvalidConversionError)
}

fn login(mut accountset: AccountSet) -> Result<(), Error> {
    println!("\n\
             Please input your private key.\n\
             or Enter 0 if you want to go back");
    loop {
        match input::<String>() {
            Ok(n) => {
                if n == "0".to_string() {
                    println!("Back to main menu\n");
                    return Ok(());
                } else {
                    let mut input_keypair = n.replace(" ", "");
                    input_keypair = input_keypair.replace("\n", "");
                    let input_keypair = input_keypair
                        .strip_prefix("[").unwrap_or(&*input_keypair)
                        .strip_suffix("]").unwrap_or(&*input_keypair);
                    let keypair_bytes_result: Result<Vec<u8>, Error> = input_keypair
                        .split(',')
                        .map(|s| s.trim().parse::<u8>().map_err(|_| Error::ParseIntError))
                        .collect::<Result<Vec<u8>, Error>>();
                    let keypair_vec = match keypair_bytes_result {
                        Ok(bytes) => {
                            if bytes.len() != 64 {
                                println!("Error: Invalid keypair length\n\
                                Please input corret keypair\n\
                                or Enter 0 if you want to go back\n");
                                continue
                            }
                            bytes
                        },
                        Err(err) => {
                            println!("Error: {:?}\n\
                            Please input corret keypair\n\
                            or Enter 0 if you want to go back\n", err);
                            continue
                        }
                    };

                    let keypair_array: [u8; 32] = match keypair_vec[32..64].try_into() {
                        Ok(array) => array,
                        Err(_) => {
                            return Err(Error::InvalidConversionError);
                        }
                    };

                    if let Some(pubkey) = accountset.get_account(&Pubkey(keypair_array)) {
                        println!("log in success");
                        return Ok(());
                    } else {
                        println!("This keypair does not exist.")
                    }
                }
            },
            Err(_) => continue
        }
    }
}

fn input<T: FromStr>() -> Result<T, Error> where <T as FromStr>::Err: Debug {
    let mut input = String::new();
    print!(">>> ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<T>() {
        Ok(val) => Ok(val),
        Err(err) => {
            println!("Error: {:?}\n\
            Please enter a valid value", err);
            Err(Error::ParseError)
        },
    }
}