//! [disktop rust cli app to monitor disk usage]

use std::error::Error;
use std::time::Duration;

use clap::Parser;

mod app;
#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(all(not(windows), feature = "termion"))]
mod termion;
#[cfg(feature = "termwiz")]
mod termwiz;
mod ui;

/// Demo
#[derive(Debug, Parser)]
struct Cli {
    /// time in ms between ticks
    #[arg(short, long, default_value_t = 250)]
    tick_rate: u64,

    /// whether unicode symbols are used to improve the overall look of the app
    #[arg(short, long, default_value_t = true)]
    unicode: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let tickrate = Duration::from_millis(cli.tick_rate);
    #[cfg(feature = "crossterm")]
    crate::crossterm::run(tick_rate, cli.unicode);
    #[cfg(all(not(windows), feature = "termion", not(feature = "crossterm")))]
    crate::termion::run(tick_rate, cli.unicode)?;
    #[cfg(all(
        feature = "termwiz",
        not(feature = "crossterm"),
        not(feature = "termion")
    ))]
    crate::termwiz::run(tick_rate, cli.unicode)?;
    Ok(())
}
