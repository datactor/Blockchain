use blockchainlib::{app, app2};
use rocksdb::{DB, Options};

fn main() {
    // let path = "./db/db";
    // let mut opts = Options::default();
    // opts.create_if_missing(true);
    //
    // let db = DB::open(&opts, path).unwrap();
    // let key = b"hello";
    // let value = b"world";
    // db.put(key, value).unwrap();
    // let result = db.get(key);
    // match result {
    //     Ok(Some(value)) => println!("retrieved value: {:?}", value),
    //     Ok(None) => println!("value not found"),
    //     Err(e) => println!("operational problem encountered: {}", e),
    // }

    app2::run();

    app::run();
}
