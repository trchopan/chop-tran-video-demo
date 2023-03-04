use anyhow::{Context, Result};
use config::Config;
use serde::Deserialize;

pub fn config_with_path<T>(config_path: &str) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
        .context("build config")?
        .try_deserialize::<T>()
        .context("deserialize config")
}
