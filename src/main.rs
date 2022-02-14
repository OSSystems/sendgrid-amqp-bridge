// Copyright (C) 2018, 2019 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: MIT

use crate::{amqp::AMQP, config::Config, sendgrid::SendGrid};
use clap::Parser;
use slog::info;
use std::path::PathBuf;

mod amqp;
mod build_info;
mod config;
mod log;
mod payload;
mod sendgrid;

#[derive(Parser, Debug)]
#[clap(
    name = "sendgrid-amqp-bridge",
    author = "O.S. Systems Software LTDA. <contact@ossystems.com.br>",
    about = "A SendGrid AMQP Bridge.",
    version = build_info::version()
)]
struct Cli {
    /// Configuration file to use
    #[clap(short = 'c', long)]
    config: PathBuf,
    /// Increase the verboseness level
    #[clap(short = 'v', long, parse(from_occurrences))]
    verbose: usize,
    /// Log output to use ('human' or 'json')
    #[clap(short = 'l', long, default_value = "human")]
    log: log::Output,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let logger = log::init(cli.verbose, cli.log);

    info!(logger, "starting"; "version" => build_info::version());
    let config = Config::load(&cli.config, &logger)?;
    let amqp = AMQP::from_config(&config);
    let sendgrid = SendGrid::from_config(&config);

    amqp.create_consumers(sendgrid, logger).await?;

    Ok(())
}
