use std::path::PathBuf;
use structopt::StructOpt;

mod lib;
use lib::Config;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input toml file
    #[structopt(parse(from_os_str,), default_value = "tests.toml")]
    fpath: PathBuf,
}

fn main() -> anyhow::Result<()> {
    match Opt::from_args() {
        Opt { fpath } => {
            let config = Config::from_fpath(fpath)?;

            config.run()?;
        }
    }
    Ok(())
}
