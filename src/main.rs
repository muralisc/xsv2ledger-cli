mod exclude_condition;
mod posting;
mod xsv2ledger;
mod xsv_to_entry;
use xsv2ledger::Xsv2Ledger;

mod settings;
use settings::Settings;

use clap::Parser;
use std::fs::File;
use tracing::info;
use tracing_core::Level;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Configuration for the xsv / financial institution we are importing
    #[arg(short, long)]
    config: String,

    // CSV with transactions
    #[arg(short, long)]
    transactions_csv: String,
}

fn main() {
    let debug_file_appender = File::create("xsv2ledger.debug.log").unwrap();
    let (non_blocking_debug, _guard) = tracing_appender::non_blocking(debug_file_appender);

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_writer(non_blocking_debug)
        .init();

    let args = Args::parse();
    info!("Config path: {}!", args.config);
    let settings = match Settings::new(&args.config) {
        Ok(settings) => settings,
        Err(error) => {
            println!("Error opening settings file, Exiting. Error: {:?}", error);
            return;
        }
    };
    info!("xsv path: {}!", args.transactions_csv);

    // Print editor shebangs !
    println!("; vim:ft=ledger");
    println!("; vim:set tw=200");

    let xsv2ledger = Xsv2Ledger::new(settings, args.transactions_csv);
    xsv2ledger.print();
}
