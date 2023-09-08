//! Module for command line argument handling and stuff
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Cli {
    /// Global flags
    #[clap(flatten)]
    pub config: Config,
}

#[derive(Debug, Parser)]
pub struct Config {
    /// Control colored output
    #[clap(
        long,
        env = "APP_COLOR",
        global = true,
        hide_env = true,
        default_value = "auto",
        hide_default_value = true,
        value_parser(["always", "auto", "never"])
    )]
    pub color: String,
}

impl Config {
    // enable/disable color based on passed value
    pub fn set_color(&self) {
        let enabled = match self.color.as_str() {
            "always" => true,
            "never" => false,
            "auto" => std::env::var_os("NO_COLOR").is_none() && atty::is(atty::Stream::Stdout),
            _ => unreachable!(),
        };
        if !enabled {
            yansi::Paint::disable()
        }
    }
}

// parse arguments
pub fn parse() -> Cli {
    let cli = Cli::parse();
    cli.config.set_color();
    cli
}
