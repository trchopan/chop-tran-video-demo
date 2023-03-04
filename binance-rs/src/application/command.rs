use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::infrastructure::binance_repo::BinanceRepo;

/// Making order to Binance trading platform using HMAC SHA 256 API key and Secret
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Config path
    #[clap(long)]
    pub config: PathBuf,

    /// Command
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Account,
    GetPrice { symbol: String },
    GetAllOrders { symbol: String },
    NewOrder { order: PathBuf },
}

pub struct CommandHandler {
    binance: BinanceRepo,
}

impl CommandHandler {
    pub fn new(binance: BinanceRepo) -> Self {
        Self { binance }
    }
    pub fn handle_args(&self, args: Args) -> Result<()> {
        match args.command {
            Command::Account => {
                let account = self.binance.get_account()?;
                println!("Accounts:");
                for balance in account.balances {
                    println!("{:<5}: {}", balance.asset, balance.free);
                }
            }
            Command::GetPrice { symbol } => {
                let price = self.binance.get_price(&symbol)?;
                println!("Price {}: {}", symbol, price.price);
            }
            Command::GetAllOrders { symbol } => {
                let orders = self.binance.get_orders(&symbol)?;
                if orders.is_empty() {
                    println!("{}: Empty orders", symbol);
                    return Ok(());
                }
                for order in orders {
                    println!(
                        "ID: BinanceID {} - ClientID {}",
                        order.order_id.unwrap_or_default(),
                        order.client_order_id.unwrap_or_default()
                    );
                    println!(
                        "Symbol {:<5} - Side {}",
                        order.symbol.unwrap_or_default(),
                        order.side.unwrap_or_default()
                    );
                    println!(
                        "Price {:<8} - Quantity {}",
                        order.price.unwrap_or_default(),
                        order.orig_qty.unwrap_or_default()
                    );
                    let status = order.status.unwrap_or_default();
                    println!("Status {:<8}", status);
                    if status == "FILLED" {
                        println!("Executed {:<8}", order.executed_qty.unwrap_or_default());
                        println!(
                            "└─Cummulative quote qty {:<8}",
                            order.cummulative_quote_qty.unwrap_or_default()
                        );
                    }
                    println!("==========")
                }
            }
            Command::NewOrder { order } => {
                let order = BinanceRepo::read_order_from_file(order)?;
                let re = self.binance.make_spot_order(order)?;
                println!("Created Order: {:?}", re);
            }
        }
        Ok(())
    }
}
