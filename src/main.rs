use clap::Parser;
use log::info;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use triangular_earth_calendar::{cmds::SubCmd, DateTime, Errors};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    sub_cmd: SubCmd,
}

fn main() -> Result<(), Errors> {
    TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    info!("Program Started");
    let args = Cli::parse();
    args.sub_cmd.run()
}
