mod cli_opts;
mod room_id;
mod serve;

use clap::Clap;
use cli_opts::{Opts, SubCommand};
use serve::serve;
use std::io::Result;

fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Serve(serve_opts) => serve(serve_opts),
        SubCommand::Validate(_validate_opts) => {
            unimplemented!()
        }
    }
}
