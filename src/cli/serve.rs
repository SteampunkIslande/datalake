use clap::Parser;
use core::net::IpAddr;
use rocket::{Config, routes};
use std::str::FromStr;

use datalake::backend::*;

use crate::Cli;

#[derive(Parser, Debug, Clone)]
pub struct Serve {
    /// IP address to listen to (default is 127.0.0.0)
    #[arg(long)]
    pub ip: Option<String>,

    /// port to listen to (default is 8888)
    #[arg(long, short)]
    pub port: Option<u16>,

    /// route prefix.
    #[arg(long)]
    pub route_prefix: Option<String>,
}

impl Serve {
    pub fn run(&self, _cli: &Cli) -> anyhow::Result<()> {
        let address = IpAddr::from_str(match &self.ip {
            Some(ip) => ip.as_str(),
            None => "127.0.0.1",
        })?;

        let config = Config {
            port: self.port.unwrap_or(8000),
            address: address,
            ..Config::default()
        };

        let prefix = self
            .route_prefix
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("/datalake");

        rocket::execute(async {
            rocket::build()
                .configure(config)
                .mount(prefix, routes![import_file_post])
                .launch()
                .await
        })?;
        Ok(())
    }
}
