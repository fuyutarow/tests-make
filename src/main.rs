use std::path::PathBuf;
use structopt::StructOpt;

mod lib;
use lib::Manager;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input toml file
    #[structopt(parse(from_os_str,), default_value = "tests-make.toml")]
    fpath: PathBuf,
}

fn main() -> anyhow::Result<()> {
    match Opt::from_args() {
        Opt { fpath } => {
            let manager = Manager::from_fpath(fpath)?;

            manager.run()?;
        }
    }
    Ok(())
}
