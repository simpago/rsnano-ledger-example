use rsnano_ledger::{Ledger, LedgerConstants};
use rsnano_store_lmdb::LmdbStore;
use std::{path::PathBuf, sync::Arc};

fn main() -> anyhow::Result<()> {
    // Open the ledger
    let home = std::env::var("HOME")?;
    let mut ledger_path = PathBuf::new();
    ledger_path.push(home);
    ledger_path.push("NanoBeta/data.ldb");

    println!("Opening ledger {:?}", ledger_path);
    let store = Arc::new(LmdbStore::open(&ledger_path)?);
    let ledger = Ledger::new(store, LedgerConstants::beta()?)?;

    // Start a database transaction
    let txn = ledger.read_txn();

    // Print number of accounts
    let accounts = ledger.store.account().count(txn.txn());
    println!("{} accounts in the ledger", accounts);

    // Print number of unchecked blocks
    let unchecked = ledger.store.unchecked().count(txn.txn());
    println!("{} unchecked blocks", unchecked);

    // Print a random block hash
    let random_block = ledger.store.block().random(txn.txn()).unwrap();
    println!("a random block: {}", random_block.as_block().hash());

    // Print all blocks that are checked but unconfirmed
    let mut iter = ledger.store.block().begin(txn.txn());
    while let Some((hash, block)) = iter.current() {
        let account = ledger.account(txn.txn(), hash).unwrap();

        let confirmation = ledger
            .store
            .confirmation_height()
            .get(txn.txn(), &account)
            .unwrap_or_default();

        if block.sideband.height > confirmation.height {
            print!("found unconfirmed block: {}", hash);
        }

        iter.next();
    }

    Ok(())
}
