mod account;
use account::{Bank, Transaction};

use std::error::Error;
use std::io;

use clap::Parser;
use csv::{ReaderBuilder, Trim};

//Using clap for command line arguments even thought there is
//only one as it provides nice error handling with help messages.
#[derive(Parser, Debug)]
struct Cli {
    pub(crate) input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_path(cli.input)?;
    let mut raw_transaction = csv::StringRecord::new();
    let headers = rdr.headers()?.clone();

    let mut bank = Bank::new();

    //Read one transaction into memory at a time https://docs.rs/csv/latest/csv/tutorial/index.html#serde-and-zero-allocation
    while rdr.read_record(&mut raw_transaction)? {
        let transaction: Transaction = raw_transaction.deserialize(Some(&headers))?;

        //Ignoring errors for now, but in the real world these should be handled
        let _ = bank.process_transaction(transaction);
    }

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for (_, account) in bank.get_accounts() {
        wtr.serialize(account)?;
    }

    wtr.flush()?;
    Ok(())
}
