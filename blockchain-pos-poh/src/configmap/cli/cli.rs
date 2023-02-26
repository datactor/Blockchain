// 1. Login interaction or sign in (create wallet)
// 2. If there is a wallet, enter the key and interact with the account DB to bring information such as balance. If there is no wallet, go to step 3
// 3. If there is no wallet, create a wallet by issuing a private key and updating the account db.
//
// cli such as stake will be implemented later
// 1. If run tx, it does not need to reopen the account db because the balance loaded from the account db will be loaded into the memory of the cli.
// 2. If the recipient's address, balance, and private key are entered, some option is provided to user (speed according to fee, related meta)
// 3. it is checked whether they match the signature retrieved when logging in, and at the same time, whether the balance is satisfied.
// 4. If the above process is confirmed, send a tx to the network for a request.
