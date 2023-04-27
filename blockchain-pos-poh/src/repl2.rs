use super::*;
use rocksdb::{DB, Options, WriteBatch, WriteOptions, ReadOptions, IteratorMode, DBWithThreadMode, SingleThreaded};
use std::{
    io::{self, prelude::*, },
    env,
    fmt::Debug,
    str::FromStr,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use bs58::{decode, encode};
use ring::signature::{Ed25519KeyPair, KeyPair};
use rand::{Rng, thread_rng, rngs::OsRng};
use crate::nodes::bootstrap::bootstrap;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Error {
    ParseError,
    ParseIntError,
    KeypairError,
    InvalidConversionError,
    HumanIdentificationError,
    BackToPrevious,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError => write!(f, "Error parsing input"),
            Error::ParseIntError => write!(f, "Error parsing int input"),
            Error::KeypairError => write!(f, "Error creating keypair"),
            Error::InvalidConversionError => write!(f, "Error invalid phrase"),
            Error::HumanIdentificationError => write!(f, "Error identifying non-robot"),
            Error::BackToPrevious => write!(f, "Back to previous"),
        }
    }
}

// impl std::error::Error for Error {}

pub fn login_menu_main() {
    let args: Vec<String> = env::args().collect();
    // let mut accountset = accountset;
    let mut accountset = AccountSet::new();

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345);
            bootstrap(12345, socket).await.expect("Boot failure");
        });


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

                        // let dbhandler = DBHandler::handle_request_get();

                        let db = DB::open_default("./db/accountDB").unwrap();
                        if let Some(mut account) = result_wrapper(login(&accountset, &db)) {
                            // action_menu(&mut account, accountset)
                        }
                    },
                    2 => {
                        println!("create new wallet\n");
                        // let account = result_wrapper(create_new_wallet(accountset));
                        if let Some(mut account) = result_wrapper(create_new_wallet(&accountset)) {
                            accountset.insert_account(account.owner.clone(), account.clone());
                            action_menu(&mut account, &mut accountset)
                        }
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

fn action_menu(account: &mut Account, accountset: &mut AccountSet) {
    println!("action menu");

    loop {
        println!("\n\
                 1: Transfer\n\
                 2: balance\n\
                 0: back to main menu\n\
                 ");

        match input::<usize>() {
            Ok(n) => match n {
                0 => break,
                1 => {
                    println!("Transaction\n");
                },
                2 => {
                    println!("Balance\n");
                    get_balance(account);
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

fn get_balance(account: &mut Account) {
    println!("balance: {}", account.balance);
}

fn create_new_wallet(accountset: &AccountSet) -> Result<Account, Error> {
    if let Err(e) = is_human() {
        return Err(e)
    }

    let new_private = loop {
        let tmp_key = Privatekey::new();
        if accountset.get_account(&tmp_key.pubkey()).is_none() {
            break tmp_key
        }
    };

    for i in 0..5 {
        println!("({}/5) Would you like to register this wallet? (y/n)", 5-i);
        let input = input::<String>()?;
        match input.as_ref() {
            "y" => {
                let new_account = Account::new(0, new_private.pubkey(), 0, vec![], false);
                // accountset.insert_account(new_private.pubkey(), new_account);
                let path = "./db/accountDB";
                let mut opts = Options::default();
                // opts.create_if_missing(true); // default is true

                let db = DB::open(&opts, path).unwrap();
                let key = new_private.pubkey().0;
                let value = new_account.finalize().0;
                db.put(key, value).unwrap();
                let result = db.get(key);
                match result {
                    Ok(Some(value)) => println!("retrieved value: {:?}", value),
                    Ok(None) => println!("value not found"),
                    Err(e) => println!("operational problem encountered: {}", e),
                }

                println!("The wallet has been successfully registered");
                return Ok(new_account)
            },
            "n" => {
                println!("Creation canceled. Return to the previous menu.");
                return Err(Error::BackToPrevious)
            },
            _ => continue
        }
    }

    Err(Error::InvalidConversionError)
}

fn login(accountset: &AccountSet, db: &DBWithThreadMode<SingleThreaded>) -> Result<Hash, Error> {
    let read_opts = ReadOptions::default();
    println!("\n\
             Please input your private key.\n\
             or Enter 0 if you want to go back");
    loop {
        match input::<String>() {
            Ok(n) => {
                if n == "0".to_string() {
                    println!("Back to main menu\n");
                    return Err(Error::BackToPrevious);
                } else {
                    let mut input_keypair = n.replace(" ", "");
                    input_keypair = input_keypair.replace("\n", "");
                    let input_keypair = input_keypair
                        .strip_prefix("[").unwrap_or(&input_keypair)
                        .strip_suffix("]").unwrap_or(&input_keypair);
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

                    // if let Some(pubkey) = accountset.get_account(&Pubkey(keypair_array)) {
                    //     println!("log in success");
                    //     return Ok(pubkey.clone());
                    if let Ok(Some(account)) = db.get_opt(&Pubkey(keypair_array), &read_opts) {
                        println!("log in success");
                        // let account_hash: [u8; 32] = account.try_into().unwrap();
                        return Ok(Hash(account.try_into().unwrap()));
                    } else {
                        println!("This keypair does not exist.")
                    }
                }
            },
            Err(_) => continue
        }
    }
}

fn input<T: FromStr>() -> Result<T, Error>
    where
        <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    print!(">>> ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<T>() {
        Ok(val) => Ok(val),
        Err(err) => {
            println!("{:?}\nError: {:?}\n\
            Please enter a valid value", input.trim(), err);
            Err(Error::ParseError)
        },
    }
}

fn result_wrapper<T>(result: Result<T, Error>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(err) => {
            println!("Back to main: {}", err);
            None
        }
    }
}

fn is_human() -> Result<(), Error> {
    // thread_rng() vs OsRng?
    // OsRng는 OS별 메서드를 사용해 안전하고 예측할 수 없도록 설계된 난수를 사용한다.
    // thread_rng는 범용 사례에 적합한 단순한 난수 생성기이지만 민감한 응용프로그램에 대해 암호학적으로는
    // 안전을 보증하지는 못함.
    // 즉 OsRng는 보다 암호화 응용프로그램에 더 적합하다.
    // 그렇지만 humancheck 수단에 있어서는 암호화 목적으로 사용하지 않기 때문에 OsRng를 사용할 필요는 없다.
    let mut is_human: Option<bool> = None;

    for i in 0..5 {
        let rand_num = thread_rng().gen_range(100_000_000_000..1_000_000_000_000);
        println!("Type this 12 digit number: {} ({}/5)", rand_num, 5 - i);
        match input::<usize>() {
            Ok(n) if n == rand_num => {
                is_human = Some(true);
                println!("Identified");
                break;
            }
            Ok(_) => println!("Wrong number. try again"),
            Err(_) => {},
        }
    }

    if let Some(false) | None = is_human {
        println!("Identification failed. Returning to main menu.");
        return Err(Error::HumanIdentificationError);
    }

    Ok(())
}