use anyhow::Result;
use binance_rs::{
    app_config::AppConfig,
    application::command::{Args, CommandHandler},
    domain::binance::BinanceKeypair,
    infrastructure::{binance_repo::BinanceRepo, config_facade::config_with_path},
};
use clap::Parser;
use env_logger::Env;

fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let args = Args::parse();
    let config_path = args.config.to_str().unwrap();
    let cfg = config_with_path::<AppConfig>(config_path)?;

    let keypair = BinanceKeypair {
        key: cfg.api_key,
        secret: cfg.secret_key,
    };
    let binance_svc = BinanceRepo::new(cfg.binance, keypair);

    CommandHandler::new(binance_svc).handle_args(args)
}
